#!/usr/bin/env bash
# Signal Noise — one-time VPS provisioning (idempotent), BUILD-ON-HOST model.
#
# Reflects the LIVE box (THE-129): Rocky Linux 10, reached as **root** over the
# Paperclip host bridge (host.containers.internal / 169.254.1.2). We are already
# root, so there is NO sudo and NO apt — package management is dnf.
#
# The Paperclip container is Debian 13 / glibc 2.41; the host is glibc 2.39, so a
# container-built binary will NOT run on the host. The toolchain therefore lives
# on the HOST and the release is built there (see scripts/deploy.sh). This script
# only lays down host build deps, the service user/dirs, Caddy, and — critically —
# **appends** the news site block to the existing /etc/caddy/Caddyfile. It must
# never overwrite that file: it already serves `scuffedcrew.no -> :3100` (the
# Paperclip control plane) and clobbering it would kill the control plane.
#
# Run AFTER inspect.sh, BEFORE the first deploy.sh. Safe to re-run.
#
# Usage:
#   ./scripts/provision.sh                                        # defaults below
#   AINORY_VPS_SSH_HOST=194.163.163.153 ./scripts/provision.sh    # public IP instead
set -euo pipefail

# Live reality: root@host.containers.internal. 194.163.163.153 is the same box's
# public IP if you are provisioning from outside the Paperclip container.
HOST="${AINORY_VPS_SSH_HOST:-169.254.1.2}"
SSH_USER="${AINORY_VPS_SSH_USER:-root}"
SSH_KEY="${AINORY_VPS_SSH_KEY:-.deploy/id_ed25519}"
SSH_PORT="${AINORY_VPS_SSH_PORT:-22}"
DOMAIN="${AINORY_DOMAIN:-news.scuffedcrew.no}"
SSH=(ssh -i "$SSH_KEY" -p "$SSH_PORT" -o StrictHostKeyChecking=accept-new "$SSH_USER@$HOST")
SCP=(scp -i "$SSH_KEY" -P "$SSH_PORT" -o StrictHostKeyChecking=accept-new)

echo "==> uploading systemd unit to /tmp on $SSH_USER@$HOST"
"${SCP[@]}" deploy/ainory-times.service "$SSH_USER@$HOST:/tmp/ainory-times.service"

echo "==> provisioning host build deps + service layout on $SSH_USER@$HOST"
"${SSH[@]}" "DOMAIN='$DOMAIN' bash -seuo pipefail" <<'REMOTE'
export HOME=/root CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup

echo "--> [1/6] build deps: pkgconf + gcc + rustup + wasm target + dioxus-cli (dx)"
command -v pkg-config >/dev/null 2>&1 || dnf install -y pkgconf-pkg-config
command -v gcc        >/dev/null 2>&1 || dnf install -y gcc
if [ ! -x /root/.cargo/bin/cargo ]; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal --default-toolchain stable
fi
# shellcheck disable=SC1091
source /root/.cargo/env
rustup target add wasm32-unknown-unknown
if ! /root/.cargo/bin/dx --version 2>/dev/null | grep -q '0.7'; then
  cargo install dioxus-cli@0.7.3 --locked
fi
/root/.cargo/bin/dx --version

echo "--> [2/6] service user + directories"
id ainory >/dev/null 2>&1 || useradd --system --no-create-home --shell /sbin/nologin ainory
mkdir -p /opt/ainory-times/src /opt/ainory-times/releases /var/lib/ainory-times/data
chown -R ainory:ainory /var/lib/ainory-times /opt/ainory-times

echo "--> [3/6] Caddy (Rocky/dnf via COPR @caddy/caddy)"
if ! command -v caddy >/dev/null 2>&1; then
  dnf install -y dnf-plugins-core
  dnf copr enable -y @caddy/caddy
  dnf install -y caddy
  systemctl enable --now caddy
fi
caddy version

echo "--> [4/6] install systemd unit (enabled, not started; deploy.sh starts it)"
install -m0644 /tmp/ainory-times.service /etc/systemd/system/ainory-times.service
rm -f /tmp/ainory-times.service
systemctl daemon-reload
systemctl enable ainory-times
echo "    ExecStart -> $(grep ^ExecStart= /etc/systemd/system/ainory-times.service)"

echo "--> [5/6] APPEND news site block to /etc/caddy/Caddyfile (never overwrite)"
CADDY=/etc/caddy/Caddyfile
touch "$CADDY"
if ! grep -q "$DOMAIN" "$CADDY"; then
  cp -a "$CADDY" "${CADDY}.bak.$(date -u +%Y%m%dT%H%M%SZ)"
  cat >> "$CADDY" <<BLOCK

$DOMAIN {
    encode zstd gzip
    reverse_proxy 127.0.0.1:8888
}
BLOCK
  if caddy validate --config "$CADDY" --adapter caddyfile; then
    systemctl reload caddy
    echo "    Caddy reloaded with $DOMAIN -> 127.0.0.1:8888"
  else
    echo "!! caddy validate FAILED — restoring backup, leaving control plane intact" >&2
    cp -a "$(ls -1t ${CADDY}.bak.* | head -1)" "$CADDY"
    exit 1
  fi
else
  echo "    $DOMAIN already present in Caddyfile; left untouched"
fi

echo "--> [6/6] provisioned. Unit installed + enabled; Caddy fronting $DOMAIN."
echo "    Run scripts/deploy.sh to build on host, stage a release, and start."
REMOTE

echo "==> done. next: ./scripts/deploy.sh"
