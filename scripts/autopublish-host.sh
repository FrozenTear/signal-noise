#!/usr/bin/env bash
# Autonomous publish — HOST SIDE (THE-233 / closes THE-227 Path A).
#
# Runs AS ROOT ON THE VPS. Invoked over SSH by scripts/autopublish.sh (the
# agent-side trigger) — or manually by an operator. It sweeps the approved
# publish.json artifacts in the deployed source tree and POSTs any that are not
# yet live to the LOCAL bind, authenticating with the host-local bearer.
#
# Why this shape (Path A, see THE-232 recommendation):
#   - The bearer token already lives on the host in /etc/ainory-times.env. We
#     source it HERE and pipe it straight into curl's auth header. The token
#     never leaves the host, never lands in a GitHub secret, never hits a log
#     or an issue thread. Same discipline as THE-229.
#   - POST /api/articles is an idempotent UPSERT-by-slug that runs inside the
#     running server (single-writer SurrealKV handle), so no unit stop/restart.
#
# Idempotency (guardrail: don't double-publish a slug):
#   Live-feed presence is ground truth. For each publish.json slug we GET
#   /api/articles/<slug>; if it already returns 200 we SKIP it. Set FORCE=1 to
#   re-POST anyway (intentional content update / re-publish of a rejected slug).
#
# Env:
#   SRC      source tree to sweep         (default /opt/ainory-times/src)
#   BASE     local API bind               (default http://127.0.0.1:8888)
#   ENV_FILE token source                 (default /etc/ainory-times.env)
#   FORCE    1 = re-POST even if live      (default 0)
#   ONLY     publish only this one slug    (default: all)
set -uo pipefail

SRC="${SRC:-/opt/ainory-times/src}"
BASE="${BASE:-http://127.0.0.1:8888}"
ENV_FILE="${ENV_FILE:-/etc/ainory-times.env}"
FORCE="${FORCE:-0}"
ONLY="${ONLY:-}"

say() { printf '%s\n' "$*"; }
die() { printf '!! %s\n' "$*" >&2; exit 1; }

# ── source the host-local bearer (value is never printed) ───────────────────
[ -f "$ENV_FILE" ] || die "token file $ENV_FILE not found on host"
# shellcheck disable=SC1090
set -a; . "$ENV_FILE"; set +a
[ -n "${SEED_API_TOKEN:-}" ] || die "SEED_API_TOKEN absent from $ENV_FILE"
say "== autopublish-host: src=$SRC base=$BASE force=$FORCE token=loaded(masked) =="
say "deployed sha: $(cat /opt/ainory-times/DEPLOYED_SHA 2>/dev/null || echo '?')"

# helper: HTTP status of a public GET (no auth needed for reads)
feed_has() { # $1 slug -> 0 if live (200), 1 otherwise
  local c
  c=$(curl -s -o /dev/null -w '%{http_code}' --max-time 15 "$BASE/api/articles/$1" || echo ERR)
  [ "$c" = "200" ]
}

mapfile -t ARTS < <(find "$SRC/docs/published" -type f -name publish.json 2>/dev/null | sort)
[ "${#ARTS[@]}" -gt 0 ] || { say "no publish.json under $SRC/docs/published — nothing to do"; exit 0; }

published=(); skipped=(); failed=()
for art in "${ARTS[@]}"; do
  slug=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("slug",""))' "$art" 2>/dev/null || echo "")
  [ -n "$slug" ] || { say "SKIP $art (no slug field)"; continue; }
  [ -n "$ONLY" ] && [ "$slug" != "$ONLY" ] && continue

  if [ "$FORCE" != "1" ] && feed_has "$slug"; then
    say "skip   $slug (already live)"
    skipped+=("$slug"); continue
  fi

  # POST: pipe token straight into the header, never echo it.
  code=$(curl -s -o /tmp/ap-resp.json -w '%{http_code}' --max-time 30 \
    -H 'content-type: application/json' \
    -H "Authorization: Bearer ${SEED_API_TOKEN}" \
    --data @"$art" "$BASE/api/articles" || echo ERR)
  if [ "$code" != "200" ] && [ "$code" != "201" ]; then
    say "FAIL   $slug -> HTTP $code: $(head -c 160 /tmp/ap-resp.json 2>/dev/null)"
    failed+=("$slug"); continue
  fi
  rslug=$(python3 -c 'import json; print(json.load(open("/tmp/ap-resp.json")).get("slug",""))' 2>/dev/null || echo "$slug")
  get=$(curl -s -o /dev/null -w '%{http_code}' --max-time 20 "$BASE/api/articles/$rslug" || echo ERR)
  if [ "$get" = "200" ]; then
    say "PUBLISH $rslug -> POST $code, GET 200"
    published+=("$rslug")
  else
    say "FAIL   $rslug -> POST $code but GET $get"
    failed+=("$rslug")
  fi
done
rm -f /tmp/ap-resp.json

say "== summary: published=${#published[@]} skipped=${#skipped[@]} failed=${#failed[@]} =="
[ "${#published[@]}" -gt 0 ] && say "  published: ${published[*]}"
[ "${#failed[@]}" -gt 0 ] && { say "  failed: ${failed[*]}"; exit 1; }
# Machine-readable tail for the agent-side wrapper (no token, slugs only):
say "RESULT published=${published[*]:-} skipped_count=${#skipped[@]}"
exit 0
