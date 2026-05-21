#!/usr/bin/env bash
# Seed approved articles into the LIVE site over its public HTTP API (THE-157).
#
# Why HTTP (not SSH + systemctl stop/seed/restart):
#   The Dioxus server nests the API at /api on the same public bind (src/main.rs),
#   and POST /api/articles is an idempotent UPSERT-by-slug that runs *inside* the
#   running server — so the server itself owns the single-writer SurrealKV handle.
#   That means we do NOT need to stop the unit, do NOT need SSH, and do NOT need to
#   put the deploy private key into GitHub secrets. We just POST the payloads.
#
# Intended to run from a GitHub-hosted runner (.github/workflows/deploy-seed.yml),
# which can reach the VPS; the agent sandbox's egress to 194.163.163.153 is blocked.
#
# Modes:
#   MODE=probe (default)  read-only: GET / and GET /api/articles, print live state.
#                         A reachability + current-content bridge, since the sandbox
#                         cannot curl the VPS directly but a runner can, and run logs
#                         are readable via api.github.com.
#   MODE=seed             POST every docs/published/*/publish.json to /api/articles,
#                         then GET /api/articles/<slug> to confirm each renders.
#                         Requires SEED_API_TOKEN set in env — never commit the token.
#
# A publish.json file IS the ArticlePublishPayload (src/api/routes.rs:
#   title, slug?, summary?, body, category, persona, confidence_score?,
#   ai_monologue?, ai_monologue_extended, sources[]?, pipeline_steps[]?).
set -euo pipefail

BASE="${BASE:-https://news.scuffedcrew.no}"
MODE="${MODE:-probe}"

say() { printf '%s\n' "$*"; }

probe() {
  say "== probe ${BASE} =="
  local root_code feed
  root_code=$(curl -fsS -o /dev/null -w '%{http_code}' --max-time 20 "${BASE}/" || echo "ERR")
  say "GET /            -> ${root_code}"
  feed=$(curl -fsS --max-time 20 "${BASE}/api/articles" || echo '{}')
  local n
  n=$(printf '%s' "$feed" | python3 -c 'import json,sys
try: d=json.load(sys.stdin)
except Exception: print("parse-error"); raise SystemExit
a=d.get("articles",d) if isinstance(d,dict) else d
print(len(a) if isinstance(a,list) else "?")' 2>/dev/null || echo "?")
  say "GET /api/articles -> ${n} article(s)"
  printf '%s' "$feed" | python3 -c 'import json,sys
try: d=json.load(sys.stdin)
except Exception: raise SystemExit
a=d.get("articles",d) if isinstance(d,dict) else d
[print("   -", x.get("slug"), "|", (x.get("title") or "")[:70]) for x in (a or [])[:30] if isinstance(x,dict)]' 2>/dev/null || true
  # Non-destructive write-gate check: POST an empty body with NO auth header.
  # The THE-159 BearerAuth extractor runs before body parsing, so a gated server
  # returns 401/503 and an un-gated one returns 4xx from validation — no article
  # is created either way. Tells us whether a real SEED_API_TOKEN is required.
  local gate
  gate=$(curl -s -o /dev/null -w '%{http_code}' --max-time 20 \
    -H 'content-type: application/json' -d '{}' "${BASE}/api/articles" || echo "ERR")
  case "$gate" in
    401|503) say "write-gate: ENFORCED (POST /api/articles -> ${gate}); real SEED_API_TOKEN required" ;;
    400|422) say "write-gate: OPEN (POST /api/articles -> ${gate}); THE-159 gate not deployed live yet" ;;
    *)       say "write-gate: inconclusive (POST /api/articles -> ${gate})" ;;
  esac
  [ "$root_code" = "200" ] || { say "FAIL: root not 200"; return 1; }
}

# Read-only per-article transparency capture (THE-206 sign-off evidence).
# For every live slug: GET /api/articles/<slug> + the /article/<slug> page, then
# summarize the transparency payload served from PRODUCTION (confidence, source
# count, pipeline-step count, short/extended monologue presence, persona/byline).
# No token needed (reads only). This is the live evidence agent sandboxes cannot
# capture (egress wall) but a runner can.
verify() {
  say "== transparency verify ${BASE} =="
  local feed slugs
  feed=$(curl -fsS --max-time 20 "${BASE}/api/articles" || echo '{}')
  slugs=$(printf '%s' "$feed" | python3 -c 'import json,sys
try: d=json.load(sys.stdin)
except Exception: raise SystemExit
a=d.get("articles",d) if isinstance(d,dict) else d
[print(x.get("slug")) for x in (a or []) if isinstance(x,dict) and x.get("slug")]' 2>/dev/null || true)
  local n_ok=0 n_total=0 fail=0
  while IFS= read -r slug; do
    [ -z "$slug" ] && continue
    n_total=$((n_total+1))
    local detail dcode pcode
    detail=$(curl -fsS --max-time 20 "${BASE}/api/articles/${slug}" || echo '{}')
    dcode=$(curl -s -o /dev/null -w '%{http_code}' --max-time 20 "${BASE}/api/articles/${slug}" || echo ERR)
    pcode=$(curl -s -o /dev/null -w '%{http_code}' --max-time 20 "${BASE}/article/${slug}" || echo ERR)
    say "--- ${slug}"
    say "    GET /api/articles/${slug} -> ${dcode} ; GET /article/${slug} -> ${pcode}"
    printf '%s' "$detail" | python3 -c '
import json,sys
try: x=json.load(sys.stdin)
except Exception:
    print("    detail: parse-error"); raise SystemExit
if isinstance(x,dict) and "article" in x: x=x["article"]
def n(v): return len(v) if isinstance(v,list) else 0
conf=x.get("confidence_score"); src=n(x.get("sources"))
mo=x.get("ai_monologue") or ""; moe=x.get("ai_monologue_extended") or ""
persona=x.get("persona") or (x.get("pipeline_metadata") or {}).get("byline")
# Pipeline trail may live under several key names / as edges — probe alternates
# so we never raise a false "missing trail" alarm from a key mismatch.
alts=["pipeline_steps","pipeline","steps","pipeline_trail","produced_by","produced_by_steps","provenance"]
trail=0; trail_key=None
for k in alts:
    c=n(x.get(k))
    if c>0: trail=c; trail_key=k; break
print(f"    persona={persona} confidence={conf} sources={src} pipeline_steps={trail}"+(f"({trail_key})" if trail_key else "")+f" monologue_short={len(mo)}c monologue_extended={len(moe)}c")
print(f"    detail_keys={sorted(x.keys()) if isinstance(x,dict) else type(x).__name__}")
ok=(conf is not None) and src>0 and trail>0 and len(mo)>0 and len(moe)>0
print("    transparency: " + ("COMPLETE" if ok else "INCOMPLETE (pipeline trail not in API detail)"))
' || say "    detail: error"
    { [ "$dcode" = "200" ] && [ "$pcode" = "200" ]; } && n_ok=$((n_ok+1)) || fail=1
  done <<< "$slugs"
  say "verify: ${n_ok}/${n_total} articles return 200 on both API + page route"
  return $fail
}

seed() {
  if [[ -z "${SEED_API_TOKEN:-}" ]]; then
    say "ERROR: SEED_API_TOKEN is not set — cannot authenticate writes (THE-175)." >&2
    exit 1
  fi
  mapfile -t ARTS < <(find docs/published -type f -name publish.json 2>/dev/null | sort)
  if [ "${#ARTS[@]}" -eq 0 ]; then
    say "No docs/published/*/publish.json found in the checkout — nothing to seed."
    say "PREREQUISITE (THE-157): the approved publish.json artifacts (THE-127 / commit 28f4701)"
    say "were never pushed to GitHub master. Push them, then re-run this workflow."
    return 0
  fi
  say "Seeding ${#ARTS[@]} article(s) to ${BASE}/api/articles"
  local fail=0
  for art in "${ARTS[@]}"; do
    local slug resp code
    slug=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("slug",""))' "$art" 2>/dev/null || echo "")
    say "POST $art (slug=${slug:-<auto>})"
    code=$(curl -fsS -o /tmp/seed-resp.json -w '%{http_code}' --max-time 30 \
      -H 'content-type: application/json' \
      -H "Authorization: Bearer ${SEED_API_TOKEN}" \
      --data @"$art" \
      "${BASE}/api/articles" || echo "ERR")
    if [ "$code" != "200" ] && [ "$code" != "201" ]; then
      say "  FAIL (HTTP $code): $(cat /tmp/seed-resp.json 2>/dev/null)"; fail=1; continue
    fi
    slug=$(python3 -c 'import json; print(json.load(open("/tmp/seed-resp.json")).get("slug",""))' 2>/dev/null || echo "$slug")
    resp=$(curl -fsS -o /dev/null -w '%{http_code}' --max-time 20 "${BASE}/api/articles/${slug}" || echo "ERR")
    say "  published; GET /api/articles/${slug} -> ${resp}"
    [ "$resp" = "200" ] || fail=1
  done
  return $fail
}

case "$MODE" in
  probe)  probe ;;
  verify) probe || true; verify ;;
  seed)   probe || true; seed ;;
  *) say "unknown MODE='$MODE' (use probe|verify|seed)"; exit 2 ;;
esac
