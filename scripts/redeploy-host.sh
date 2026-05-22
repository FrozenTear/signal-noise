#!/usr/bin/env bash
# Build-on-host redeploy of the bearer-gated signal-noise binary (THE-198).
#
# Runs AS ROOT ON THE VPS (invoked over SSH by .github/workflows/deploy.yml, or
# manually by an operator). It performs the documented flow:
#   git fetch/checkout master  ->  dx bundle --release  ->  stage release  ->
#   inject SEED_API_TOKEN into the unit  ->  flip `current` symlink  ->
#   daemon-reload + restart  ->  health-gate with auto-rollback  ->
#   verify the bearer gate (503 no-token / 401|403 wrong-token / 200 valid UPSERT).
#
# SEED_API_TOKEN is sourced from the environment (the workflow passes it from the
# GH Actions secret of the same name). The token is written into the systemd unit
# as `Environment=SEED_API_TOKEN=...`, so the gate fails-closed (503) until set and
# the same secret is shared with the seed runner path (deploy-seed.yml).
set -uo pipefail

ROOT=/opt/ainory-times
SRC="$ROOT/src"
UNIT=/etc/systemd/system/ainory-times.service
BASE_LOCAL="http://127.0.0.1:8888"
export HOME=/root
[ -f /root/.cargo/env ] && source /root/.cargo/env

die() { echo "!! $*" >&2; exit 1; }
say() { printf '%s\n' "== $* =="; }

: "${SEED_API_TOKEN:?SEED_API_TOKEN must be set (gate would otherwise stay fail-closed/503)}"

say "sync master in $SRC"
cd "$SRC" || die "no $SRC"
git fetch --all --prune || die "git fetch failed"
git checkout master || die "git checkout master failed"
git reset --hard origin/master || die "git reset failed"
SHA="$(git rev-parse --short HEAD)"
echo "checked out master @ $SHA"

say "build (dx bundle --release --platform web)"
/root/.cargo/bin/dx bundle --release --platform web || die "BUILD FAILED"

say "locate bundle"
BUNDLE="$(find "$SRC/target/dx" -maxdepth 5 -type d -path '*release/web' 2>/dev/null | head -1)"
echo "BUNDLE=$BUNDLE"
[ -n "$BUNDLE" ] && [ -d "$BUNDLE" ] || die "bundle dir missing"

say "stage new release"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
REL="$ROOT/releases/$TS"
mkdir -p "$REL"
cp -a "$BUNDLE/." "$REL/"
mkdir -p "$REL/config"; cp -a "$SRC/config/." "$REL/config/" 2>/dev/null || true
BIN=""
for f in "$REL"/*; do [ -f "$f" ] && file "$f" | grep -q ELF && { BIN="$f"; break; }; done
echo "BIN=$BIN"
[ -n "$BIN" ] || { ls -la "$REL"; die "no ELF in release"; }
chmod +x "$BIN"
echo "$SHA" > "$ROOT/DEPLOYED_SHA"

say "update unit: ExecStart + SEED_API_TOKEN env"
sed -i "s|^ExecStart=.*|ExecStart=$BIN|" "$UNIT"
# idempotently set the SEED_API_TOKEN Environment line: drop any existing one,
# then insert a fresh line just before the [Install] section.
sed -i '/^Environment=SEED_API_TOKEN=/d' "$UNIT"
sed -i "/^\[Install\]/i Environment=SEED_API_TOKEN=${SEED_API_TOKEN}" "$UNIT"
grep -qE '^Environment=SEED_API_TOKEN=' "$UNIT" || die "failed to set SEED_API_TOKEN in unit"
chown -R ainory:ainory "$ROOT" /var/lib/ainory-times 2>/dev/null || true

say "swap + restart"
PREV="$(readlink -f "$ROOT/current" || true)"
echo "$PREV" > "$ROOT/PREV_RELEASE"
ln -snf "$REL" "$ROOT/current"
systemctl daemon-reload
systemctl restart ainory-times
sleep 4

say "health gate"
OK=no
for i in $(seq 1 25); do curl -fsS "$BASE_LOCAL/" >/dev/null 2>&1 && { OK=yes; break; }; sleep 2; done
echo "HEALTH=$OK"
if [ "$OK" != yes ]; then
  echo "!! health failed — rolling back to $PREV"
  [ -n "$PREV" ] && ln -snf "$PREV" "$ROOT/current" && systemctl restart ainory-times
  journalctl -u ainory-times --no-pager -n 40
  exit 1
fi

say "verify bearer gate (THE-159 / THE-198 acceptance)"
code_no=$(curl -s -o /dev/null -w '%{http_code}' -X POST "$BASE_LOCAL/api/articles" -H 'Content-Type: application/json' -d '{}')
code_wrong=$(curl -s -o /dev/null -w '%{http_code}' -X POST "$BASE_LOCAL/api/articles" -H 'Authorization: Bearer wrong-token-deadbeef' -H 'Content-Type: application/json' -d '{}')
# THE-284: the foundation now validates `category` against the seeded `category`
# table, so the old "meta" probe category 400s. Use a real seeded beat slug.
code_valid=$(curl -s -o /dev/null -w '%{http_code}' -X POST "$BASE_LOCAL/api/articles" -H "Authorization: Bearer ${SEED_API_TOKEN}" -H 'Content-Type: application/json' \
  -d '{"title":"deploy gate probe","slug":"deploy-gate-probe","body":"gate verification upsert","category":"business","ai_monologue_extended":"automated post-deploy gate check"}')
echo "POST {} no-bearer    -> $code_no   (expect 401 or 503)"
echo "POST {} wrong-bearer -> $code_wrong (expect 401 or 403)"
echo "POST valid-bearer    -> $code_valid (expect 200/201)"

fail=0
case "$code_no" in 401|503) ;; *) echo "!! gate NOT enforced without token (got $code_no)"; fail=1;; esac
case "$code_wrong" in 401|403) ;; *) echo "!! wrong token not rejected"; fail=1;; esac
case "$code_valid" in 200|201) ;; *) echo "!! valid token UPSERT did not succeed"; fail=1;; esac

if [ "$fail" -ne 0 ]; then
  echo "!! GATE VERIFICATION FAILED — rolling back to $PREV"
  [ -n "$PREV" ] && ln -snf "$PREV" "$ROOT/current" && systemctl daemon-reload && systemctl restart ainory-times
  exit 1
fi

# THE-284: the publish UPSERT sets status='published', and the foundation's
# data-driven nav/home feed lists every published article regardless of category.
# Reset the probe to 'rejected' so the gate check never leaks "deploy gate probe"
# into the live feed. Best-effort: a non-200 here doesn't fail the deploy.
reset_code=$(curl -s -o /dev/null -w '%{http_code}' -X PATCH "$BASE_LOCAL/api/articles/deploy-gate-probe" \
  -H "Authorization: Bearer ${SEED_API_TOKEN}" -H 'Content-Type: application/json' -d '{"status":"rejected"}')
echo "PATCH probe -> rejected -> $reset_code (expect 200)"

# prune old releases (keep 5)
ls -1dt "$ROOT"/releases/* | tail -n +6 | xargs -r rm -rf
say "REDEPLOY DONE rel=$REL sha=$SHA gate=verified($code_no/$code_wrong/$code_valid)"
