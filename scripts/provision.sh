#!/usr/bin/env bash
# Signal Noise — one-time VPS provisioning (idempotent).
# Creates the ainory service user + dirs, installs Caddy, drops the systemd
# unit and Caddyfile. Run AFTER inspect.sh, BEFORE the first deploy.sh.
#
# Requires passwordless (or interactive) sudo for AINORY_VPS_SSH_USER.
#
# Usage:
#   AINORY_VPS_HOST=194.163.163.153 AINORY_DOMAIN=news.example.com ./scripts/provision.sh
#   # omit AINORY_DOMAIN for IP-only HTTP (no TLS) v1.
set -euo pipefail

HOST="${AINORY_VPS_HOST:?set AINORY_VPS_HOST}"
SSH_USER="${AINORY_VPS_SSH_USER:-ts6dev}"
SSH_KEY="${AINORY_VPS_SSH_KEY:-.deploy/id_ed25519}"
SSH_PORT="${AINORY_VPS_SSH_PORT:-22}"
DOMAIN="${AINORY_DOMAIN:-}"
SSH=(ssh -i "$SSH_KEY" -p "$SSH_PORT" -o StrictHostKeyChecking=accept-new "$SSH_USER@$HOST")
SCP=(scp -i "$SSH_KEY" -P "$SSH_PORT" -o StrictHostKeyChecking=accept-new)

echo "==> uploading unit + Caddyfile to /tmp"
"${SCP[@]}" deploy/ainory-times.service "$SSH_USER@$HOST:/tmp/ainory-times.service"

# Render Caddyfile: real domain (auto-TLS) if provided, else :80 IP-only.
if [ -n "$DOMAIN" ]; then
  printf '%s {\n    encode zstd gzip\n    reverse_proxy 127.0.0.1:8888\n}\n' "$DOMAIN" > /tmp/Caddyfile.rendered
else
  printf ':80 {\n    encode zstd gzip\n    reverse_proxy 127.0.0.1:8888\n}\n' > /tmp/Caddyfile.rendered
fi
"${SCP[@]}" /tmp/Caddyfile.rendered "$SSH_USER@$HOST:/tmp/Caddyfile"

echo "==> provisioning on VPS"
"${SSH[@]}" '
  set -e
  # service user (no login, no home clutter)
  id ainory >/dev/null 2>&1 || sudo useradd --system --no-create-home --shell /usr/sbin/nologin ainory
  sudo mkdir -p /opt/ainory-times/releases /var/lib/ainory-times/data
  sudo chown -R ainory:ainory /var/lib/ainory-times
  sudo chown -R ainory:ainory /opt/ainory-times

  # Caddy (Debian/Ubuntu); skip if already present
  if ! command -v caddy >/dev/null 2>&1; then
    sudo apt-get update -y
    sudo apt-get install -y debian-keyring debian-archive-keyring apt-transport-https curl
    curl -1sLf "https://dl.cloudsmith.io/public/caddy/stable/gpg.key" | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
    curl -1sLf "https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt" | sudo tee /etc/apt/sources.list.d/caddy-stable.list >/dev/null
    sudo apt-get update -y
    sudo apt-get install -y caddy
  fi

  sudo mv /tmp/ainory-times.service /etc/systemd/system/ainory-times.service
  sudo mv /tmp/Caddyfile /etc/caddy/Caddyfile
  sudo systemctl daemon-reload
  sudo systemctl enable ainory-times
  sudo systemctl reload caddy 2>/dev/null || sudo systemctl restart caddy
  echo "provisioned. ainory-times unit installed (not started until first deploy)."
'
echo "==> done. next: AINORY_VPS_HOST=$HOST ./scripts/deploy.sh"
