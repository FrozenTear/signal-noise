#!/usr/bin/env bash
# check-approved-not-live.sh — THE-1213
#
# Safe reconciliation sweep: for each docs/published/<slug>/publish.json whose
# LATEST git commit is an approval commit, verify the slug is live. If not live
# (404/error), trigger autopublish for that slug only.
#
# This avoids the blanket-sweep hazard: publish.json is committed at draft stage
# too; only checking the latest commit message distinguishes approved from draft.
#
# Approval commit patterns (case-insensitive):
#   "publish approved"
#   "EIC approve"
#
# Usage:
#   bash scripts/check-approved-not-live.sh            # dry-run (default)
#   APPLY=1 bash scripts/check-approved-not-live.sh    # actually trigger autopublish
#   PUBLIC_BASE=https://news.scuffedcrew.no APPLY=1 bash scripts/check-approved-not-live.sh
#
# Env:
#   PUBLIC_BASE  public URL to check liveness (default https://news.scuffedcrew.no)
#   APPLY        1 = trigger autopublish for each stranded slug; 0 = dry-run (default 0)
#   HERE_ONLY    comma-separated slugs to limit scope (default: all approved)
#   SSH_HOST     host-local API host for the SSH liveness fallback (default root@169.254.1.2)
#   SSH_KEY      SSH key for the fallback (default ~/.ssh/ainory_deploy)
#
# Liveness fallback (THE-1217): the direct-API probe returns HTTP 000 from the
# agent runner because the VPS :8888 API is not reachable directly — only via
# SSH to root@169.254.1.2. Without a fallback, every approved slug looks like a
# false-positive STRAND. When the direct probe yields 000, this script resolves
# liveness once against the SSH live feed and matches on the `slug` field, so the
# report shows the *real* stranded set (typically 0–1) instead of everything.

set -uo pipefail

PUBLIC_BASE="${PUBLIC_BASE:-https://news.scuffedcrew.no}"
APPLY="${APPLY:-0}"
HERE_ONLY="${HERE_ONLY:-}"
SSH_HOST="${SSH_HOST:-root@169.254.1.2}"
SSH_KEY="${SSH_KEY:-$HOME/.ssh/ainory_deploy}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

say()  { printf '%s\n' "$*"; }
info() { printf '[info] %s\n' "$*"; }
warn() { printf '[warn] %s\n' "$*" >&2; }

# ── SSH live-feed fallback (THE-1217) ────────────────────────────────────────
# Fetch the host-local live feed once and cache its slug set. Used when the
# direct-API probe returns 000 (sandbox egress to :8888 is blocked).
declare -A LIVE_SLUGS
LIVE_FEED_LOADED=0   # 1 once we've attempted the fetch (success or failure)
LIVE_FEED_OK=0       # 1 only if the feed parsed to a non-empty slug set

load_live_feed() {
  [ "$LIVE_FEED_LOADED" = "1" ] && return
  LIVE_FEED_LOADED=1
  info "direct probe returned 000 — fetching live feed over SSH ($SSH_HOST) for liveness fallback…"
  local json
  json=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=15 "$SSH_HOST" \
      "curl -s 'localhost:8888/api/articles?limit=500'" 2>/dev/null || echo "")
  if [ -z "$json" ]; then
    warn "SSH live-feed fetch failed — deferring 000 slugs to VPS idempotent autopublish"
    return
  fi
  local slugs
  slugs=$(printf '%s' "$json" | python3 -c '
import json,sys
try:
    d = json.load(sys.stdin)
except Exception:
    sys.exit(0)
arts = d.get("articles", d) if isinstance(d, dict) else d
if isinstance(arts, dict):
    arts = arts.get("articles", [])
for a in (arts or []):
    if isinstance(a, dict) and a.get("slug"):
        print(a["slug"])
' 2>/dev/null || echo "")
  if [ -z "$slugs" ]; then
    warn "SSH live-feed returned no parseable slugs — deferring 000 slugs to VPS"
    return
  fi
  while IFS= read -r s; do [ -n "$s" ] && LIVE_SLUGS["$s"]=1; done <<< "$slugs"
  LIVE_FEED_OK=1
  info "live feed loaded: ${#LIVE_SLUGS[@]} live slug(s)"
}

APPROVAL_PAT='publish approved|EIC approve'

# ── gather candidates ────────────────────────────────────────────────────────
mapfile -t PUBLISH_FILES < <(find "$SCRIPT_DIR/../docs/published" -type f -name publish.json 2>/dev/null | sort)

if [ "${#PUBLISH_FILES[@]}" -eq 0 ]; then
  say "no docs/published/*/publish.json files found — nothing to do"; exit 0
fi

stranded=()
approved_count=0
skip_count=0

for pf in "${PUBLISH_FILES[@]}"; do
  slug=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("slug",""))' "$pf" 2>/dev/null || echo "")
  [ -n "$slug" ] || { warn "SKIP $pf (no slug field)"; continue; }

  # Limit scope if HERE_ONLY is set
  if [ -n "$HERE_ONLY" ]; then
    match=0
    IFS=',' read -ra LIMIT <<< "$HERE_ONLY"
    for s in "${LIMIT[@]}"; do [ "$s" = "$slug" ] && match=1; done
    [ "$match" -eq 1 ] || continue
  fi

  # Get the most recent git commit message for this file
  rel_path="${pf#"$SCRIPT_DIR/../"}"
  latest_msg=$(git -C "$SCRIPT_DIR/.." log -1 --format='%s' -- "$rel_path" 2>/dev/null || echo "")

  if ! printf '%s' "$latest_msg" | grep -qiE "$APPROVAL_PAT"; then
    info "SKIP $slug — latest commit is not an approval commit: ${latest_msg:0:80}"
    skip_count=$((skip_count + 1))
    continue
  fi

  approved_count=$((approved_count + 1))

  # Check liveness. curl -w emits the HTTP code, or 000 on connection failure
  # (which is what the agent runner always sees for the public URL).
  http_code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 15 "$PUBLIC_BASE/api/articles/$slug" 2>/dev/null)
  [ -n "$http_code" ] || http_code=000

  if [ "$http_code" = "200" ]; then
    info "OK     $slug (live)"
  elif [ "$http_code" = "000" ]; then
    # Direct API unreachable from the runner — resolve liveness via the SSH feed.
    load_live_feed
    if [ -n "${LIVE_SLUGS[$slug]:-}" ]; then
      info "OK     $slug (live, via SSH feed)"
    elif [ "$LIVE_FEED_OK" = "1" ]; then
      # Feed loaded and slug absent → genuinely stranded.
      warn "STRAND $slug — approved commit, absent from SSH live feed"
      stranded+=("$slug")
    else
      # Couldn't load the feed — defer to VPS (autopublish-host.sh is idempotent).
      info "QUEUE  $slug — approved commit, SSH feed unavailable; deferred to VPS idempotent publish"
      stranded+=("$slug")
    fi
  else
    warn "STRAND $slug — approved commit but live returns $http_code"
    stranded+=("$slug")
  fi
done

say ""
say "== summary: $approved_count approved, ${#stranded[@]} stranded, $skip_count non-approval skipped =="

if [ "${#stranded[@]}" -eq 0 ]; then
  say "all approved articles are live — nothing to do"
  exit 0
fi

say ""
say "stranded slugs:"
for s in "${stranded[@]}"; do say "  - $s"; done

if [ "$APPLY" != "1" ]; then
  say ""
  say "DRY-RUN: set APPLY=1 to trigger autopublish for stranded/queued slugs"
  say "Note: QUEUE entries deferred to VPS liveness check — autopublish-host.sh skips already-live slugs idempotently"
  exit 0
fi

say ""
say "== triggering autopublish for ${#stranded[@]} stranded slug(s) =="
failed=()
for s in "${stranded[@]}"; do
  say "-- publishing $s --"
  if ONLY="$s" bash "$SCRIPT_DIR/autopublish.sh"; then
    say "OK $s"
  else
    warn "FAILED $s"
    failed+=("$s")
  fi
done

say ""
if [ "${#failed[@]}" -eq 0 ]; then
  say "all stranded slugs published successfully"
  exit 0
else
  say "FAILED slugs: ${failed[*]}"
  exit 1
fi
