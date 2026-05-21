#!/usr/bin/env bash
# Signal Noise — seed an approved standalone article into the LIVE embedded DB.
#
# This is the offline `stop -> seed_article -> restart` sequence run over SSH.
# It is the counterpart to deploy.sh and shares its env contract, so it can run
# from anywhere that can reach the VPS (a co-located Podman container OR a
# GitHub Actions runner — see .github/workflows/deploy-seed.yml and docs/DEPLOY.md).
#
# Why a dedicated script:
#  * The embedded SurrealKV store is SINGLE-WRITER. seed_article opens the same
#    data/signal-noise.db file as the service, so the service MUST be stopped
#    first (the unit uses Restart=always, so only an explicit `systemctl stop`
#    keeps it down — exactly what we want for the seed window).
#  * The DB path is relative (`data/signal-noise.db`), resolved against cwd. The
#    service runs with WorkingDirectory=/var/lib/ainory-times, so the seed binary
#    MUST run with that same cwd to hit the live DB and not a stray one.
#  * Files under /var/lib/ainory-times/data are owned by `ainory`; the seed must
#    run as `ainory` so the service can reopen the DB after restart.
#
# Usage:  ./scripts/seed.sh <slug>          # e.g. ./scripts/seed.sh the-119
# Env (same defaults as deploy.sh — target the co-located container by default):
#   AINORY_VPS_SSH_HOST   host to ssh to   (default: 169.254.1.2)
#   AINORY_VPS_SSH_USER   ssh user         (default: root)
#   AINORY_VPS_SSH_KEY    private key      (default: .deploy/id_ed25519)
#   AINORY_VPS_SSH_PORT   ssh port         (default: 22)
set -euo pipefail

SLUG="${1:-}"
[ -n "$SLUG" ] || { echo "Usage: $0 <slug>   (e.g. $0 the-119)"; exit 2; }

SSH_HOST="${AINORY_VPS_SSH_HOST:-169.254.1.2}"
SSH_USER="${AINORY_VPS_SSH_USER:-root}"
SSH_KEY="${AINORY_VPS_SSH_KEY:-.deploy/id_ed25519}"
SSH_PORT="${AINORY_VPS_SSH_PORT:-22}"
APP_DIR="/opt/ainory-times"
DATA_DIR="/var/lib/ainory-times"
SSH=(ssh -i "$SSH_KEY" -p "$SSH_PORT" -o StrictHostKeyChecking=accept-new "$SSH_USER@$SSH_HOST")

# The publish.json must already be present in the shipped source tree on the host
# (deploy.sh tar-pipes the whole repo, docs/published/ included). Seeding does not
# require a fresh deploy, but the source tree must contain the slug's publish.json.
echo "==> seeding '$SLUG' on $SSH_USER@$SSH_HOST (stop -> seed_article -> restart)"
"${SSH[@]}" bash -s -- "$APP_DIR" "$DATA_DIR" "$SLUG" <<'REMOTE'
set -euo pipefail
APP_DIR="$1"; DATA_DIR="$2"; SLUG="$3"
export PATH="/root/.cargo/bin:$PATH"
SRC="$APP_DIR/src"
JSON="$SRC/docs/published/$SLUG/publish.json"

[ -f "$JSON" ] || { echo "!! $JSON not found on host — run deploy.sh first to ship the source tree"; exit 1; }

echo "==> building seed_article on host"
cd "$SRC"
cargo build --release --features server --bin seed_article
BIN="$SRC/target/release/seed_article"
[ -x "$BIN" ] || { echo "!! seed_article binary missing at $BIN"; exit 1; }

echo "==> stopping ainory-times for single-writer seed window"
systemctl stop ainory-times

set +e
# Run the seed AS the service user, with cwd = the persistent data dir so the
# relative DB path (data/signal-noise.db) resolves to the LIVE store.
if id ainory >/dev/null 2>&1; then
  runuser -u ainory -- bash -c "cd '$DATA_DIR' && '$BIN' '$JSON'"
else
  ( cd "$DATA_DIR" && "$BIN" "$JSON" )
fi
SEED_RC=$?
set -e

# Always restart the service, even if the seed failed, so the site never stays down.
echo "==> restarting ainory-times"
# Belt-and-braces: keep the data dir owned by the service user.
id ainory >/dev/null 2>&1 && chown -R ainory:ainory "$DATA_DIR/data" 2>/dev/null || true
systemctl start ainory-times

if [ "$SEED_RC" -ne 0 ]; then
  echo "!! seed_article failed (rc=$SEED_RC) — service restarted, DB unchanged for this slug"
  exit "$SEED_RC"
fi

for i in $(seq 1 15); do
  if curl -fsS "http://127.0.0.1:8888/api/articles" >/dev/null; then
    echo "==> health OK; seeded '$SLUG'"; exit 0
  fi
  sleep 1
done
echo "!! post-seed health check FAILED"; systemctl status ainory-times --no-pager -l | tail -20; exit 1
REMOTE

echo "==> seed complete: $SLUG"
