#!/usr/bin/env bash
# Signal Noise — one-shot deploy to the board VPS.
#
# Build locally -> rsync artefacts -> atomic symlink swap -> systemd restart.
# No GitHub Actions, no git pull on the VPS, no container registry.
#
# Usage:
#   AINORY_VPS_HOST=194.163.163.153 ./scripts/deploy.sh
#
# Env (with defaults):
#   AINORY_VPS_HOST       VPS hostname/IP            (required)
#   AINORY_VPS_SSH_USER   SSH login user            (default: ts6dev)
#   AINORY_VPS_SSH_KEY    private key path          (default: .deploy/id_ed25519)
#   AINORY_VPS_SSH_PORT   SSH port                  (default: 22)
#   AINORY_KEEP_RELEASES  release dirs to retain    (default: 5)
#
# Prereqs on the build host: dx (Dioxus CLI), cargo, rsync, ssh.
# Prereqs on the VPS (done once by scripts/provision.sh): ainory user,
# /opt/ainory-times, /var/lib/ainory-times, the systemd unit, Caddy.
set -euo pipefail

HOST="${AINORY_VPS_HOST:?set AINORY_VPS_HOST (e.g. 194.163.163.153)}"
SSH_USER="${AINORY_VPS_SSH_USER:-ts6dev}"
SSH_KEY="${AINORY_VPS_SSH_KEY:-.deploy/id_ed25519}"
SSH_PORT="${AINORY_VPS_SSH_PORT:-22}"
KEEP="${AINORY_KEEP_RELEASES:-5}"

APP_DIR="/opt/ainory-times"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
REL_DIR="$APP_DIR/releases/$TS"
SSH=(ssh -i "$SSH_KEY" -p "$SSH_PORT" -o StrictHostKeyChecking=accept-new "$SSH_USER@$HOST")
RSYNC_RSH="ssh -i $SSH_KEY -p $SSH_PORT -o StrictHostKeyChecking=accept-new"

echo "==> building (dx bundle --release)"
dx bundle --release

# dx 0.7 emits the fullstack bundle under target/dx/<app>/release/web/.
# The tree contains the server binary plus the public/ static asset tree.
BUNDLE="target/dx/signal-noise/release/web"
if [ ! -d "$BUNDLE" ]; then
  echo "!! expected bundle dir not found: $BUNDLE" >&2
  echo "   inspect 'target/dx' and update BUNDLE in this script." >&2
  find target/dx -maxdepth 4 -type d 2>/dev/null | sed 's/^/   /' >&2 || true
  exit 1
fi

echo "==> syncing artefacts -> $SSH_USER@$HOST:$REL_DIR"
"${SSH[@]}" "mkdir -p '$REL_DIR'"
rsync -az --delete -e "$RSYNC_RSH" "$BUNDLE/"     "$SSH_USER@$HOST:$REL_DIR/"
rsync -az          -e "$RSYNC_RSH" config/        "$SSH_USER@$HOST:$REL_DIR/config/"

echo "==> swap current -> restart -> prune old releases"
"${SSH[@]}" "
  set -e
  sudo ln -snf '$REL_DIR' '$APP_DIR/current'
  sudo systemctl restart ainory-times
  ls -1dt '$APP_DIR'/releases/* | tail -n +$((KEEP + 1)) | sudo xargs -r rm -rf
"

echo "==> health check (localhost on VPS)"
"${SSH[@]}" "for i in \$(seq 1 10); do curl -fsS http://127.0.0.1:8888/ >/dev/null && { echo OK; exit 0; }; sleep 1; done; echo 'health check FAILED'; sudo systemctl status ainory-times --no-pager -l | tail -20; exit 1"

echo "==> deployed release $TS"
