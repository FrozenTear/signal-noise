#!/usr/bin/env bash
# Signal Noise — deploy to the board VPS (build ON the host).
#
# Reality of this box (verified 2026-05-21, see docs/DEPLOY.md):
#  * The deploy environment is a Podman container co-located ON the VPS. Reach
#    the host through host.containers.internal -> 169.254.1.2:22 (the public IP
#    194.163.163.153:22 is refused from inside the container).
#  * Host is Rocky Linux 10 (glibc 2.39). The container is Debian (glibc 2.41),
#    so a binary built in the container will NOT run on the host. We therefore
#    tar-pipe the source to the host and BUILD THERE.
#  * Caddy is already installed and serving other sites; provisioning only
#    APPENDS our vhost (see scripts/provision.sh) — never overwrites.
#
# Usage:  ./scripts/deploy.sh
# Env (defaults target this box):
#   AINORY_VPS_SSH_HOST   host to ssh to        (default: 169.254.1.2)
#   AINORY_VPS_SSH_USER   ssh user              (default: root)
#   AINORY_VPS_SSH_KEY    private key           (default: .deploy/id_ed25519)
#   AINORY_VPS_SSH_PORT   ssh port              (default: 22)
#   AINORY_KEEP_RELEASES  release dirs to keep  (default: 5)
set -euo pipefail

SSH_HOST="${AINORY_VPS_SSH_HOST:-169.254.1.2}"
SSH_USER="${AINORY_VPS_SSH_USER:-root}"
SSH_KEY="${AINORY_VPS_SSH_KEY:-.deploy/id_ed25519}"
SSH_PORT="${AINORY_VPS_SSH_PORT:-22}"
KEEP="${AINORY_KEEP_RELEASES:-5}"
APP_DIR="/opt/ainory-times"
SSH=(ssh -i "$SSH_KEY" -p "$SSH_PORT" -o StrictHostKeyChecking=accept-new "$SSH_USER@$SSH_HOST")

echo "==> shipping source -> $SSH_USER@$SSH_HOST:$APP_DIR/src (tar over ssh; rsync absent in container)"
tar --exclude=./target --exclude=./.git --exclude=./.deploy --exclude=./dist \
    --exclude=./.dioxus -czf - . \
  | "${SSH[@]}" "mkdir -p '$APP_DIR/src' && tar -xzf - -C '$APP_DIR/src'"

echo "==> building on host + staging release"
"${SSH[@]}" bash -s -- "$APP_DIR" "$KEEP" <<'REMOTE'
set -euo pipefail
APP_DIR="$1"; KEEP="$2"
export PATH="/root/.cargo/bin:$PATH"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
REL="$APP_DIR/releases/$TS"

cd "$APP_DIR/src"
dx bundle --release

# Locate the bundled web tree (server binary + public/ assets). dx 0.7 emits it
# under target/dx/signal-noise/release/web/.
WEB="$(find target/dx -type d -name web -path '*release*' | head -1)"
[ -n "$WEB" ] || { echo "!! could not find dx bundle web dir under target/dx"; find target/dx -maxdepth 4 -type d; exit 1; }

mkdir -p "$REL"
cp -a "$WEB"/. "$REL"/
cp -a config "$REL"/config 2>/dev/null || true
# Sanity: the service binary must be present and executable.
[ -x "$REL/signal-noise" ] || { echo "!! $REL/signal-noise missing/not executable"; ls -la "$REL"; exit 1; }

id ainory >/dev/null 2>&1 && chown -R ainory:ainory "$REL"
ln -snf "$REL" "$APP_DIR/current"
systemctl restart ainory-times
ls -1dt "$APP_DIR"/releases/* | tail -n +$((KEEP + 1)) | xargs -r rm -rf

for i in $(seq 1 15); do
  curl -fsS http://127.0.0.1:8888/ >/dev/null && { echo "==> health OK ($TS)"; exit 0; }
  sleep 1
done
echo "!! health check FAILED"; systemctl status ainory-times --no-pager -l | tail -20; exit 1
REMOTE

echo "==> deployed."
