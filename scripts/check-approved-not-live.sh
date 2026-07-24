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

set -uo pipefail

PUBLIC_BASE="${PUBLIC_BASE:-https://news.scuffedcrew.no}"
APPLY="${APPLY:-0}"
HERE_ONLY="${HERE_ONLY:-}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

say()  { printf '%s\n' "$*"; }
info() { printf '[info] %s\n' "$*"; }
warn() { printf '[warn] %s\n' "$*" >&2; }

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

  # Check liveness (best-effort from agent side; SSH-side check is authoritative)
  http_code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 15 "$PUBLIC_BASE/api/articles/$slug" 2>/dev/null || echo UNREACHABLE)
  if [ "$http_code" = "200" ]; then
    info "OK     $slug (live)"
  elif [ "$http_code" = "UNREACHABLE" ]; then
    # Sandbox can't reach public URL — mark for SSH-side check via autopublish.sh
    # autopublish-host.sh will skip already-live slugs idempotently
    info "QUEUE  $slug — approved commit, liveness check deferred to VPS (sandbox egress blocked)"
    stranded+=("$slug")
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
