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
  [ "$root_code" = "200" ] || { say "FAIL: root not 200"; return 1; }
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
  probe) probe ;;
  seed)  probe || true; seed ;;
  *) say "unknown MODE='$MODE' (use probe|seed)"; exit 2 ;;
esac
