#!/usr/bin/env bash
# Signal Noise — read-only first-touch inspection of the VPS.
# Run before provisioning to capture ground truth (OS, RAM/disk, open ports,
# existing reverse proxy, firewall, sudo). Paste output back into the deploy doc.
set -euo pipefail

HOST="${AINORY_VPS_HOST:?set AINORY_VPS_HOST}"
SSH_USER="${AINORY_VPS_SSH_USER:-ts6dev}"
SSH_KEY="${AINORY_VPS_SSH_KEY:-.deploy/id_ed25519}"
SSH_PORT="${AINORY_VPS_SSH_PORT:-22}"

ssh -i "$SSH_KEY" -p "$SSH_PORT" -o StrictHostKeyChecking=accept-new "$SSH_USER@$HOST" '
  echo "## hostnamectl";        hostnamectl 2>/dev/null
  echo "## os-release";         cat /etc/os-release 2>/dev/null
  echo "## memory / disk";      free -h; df -hT /
  echo "## cpu / uptime";       nproc; uptime
  echo "## listening sockets";  ss -tlnp 2>/dev/null | sort
  echo "## running services";   systemctl list-units --type=service --state=running --no-pager 2>/dev/null
  echo "## proxy/runtime bins"; for b in podman docker caddy nginx apache2 cargo dx; do printf "%s: " "$b"; command -v "$b" || echo "(absent)"; done
  echo "## firewall";           (sudo ufw status verbose 2>/dev/null || sudo iptables -S 2>/dev/null | head -40 || echo "(no ufw/iptables visibility)")
  echo "## existing TLS certs"; ls -1 /etc/letsencrypt/live 2>/dev/null || echo "(none)"
  echo "## sudo check";         (sudo -n true 2>/dev/null && echo "passwordless sudo: yes" || echo "passwordless sudo: NO (deploy needs it)")
'
