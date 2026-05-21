#!/usr/bin/env bash
# Signal Noise — read-only first-touch inspection of the VPS host.
# Defaults target this box: ssh root@169.254.1.2 (host.containers.internal from
# the co-located deploy container). Override with AINORY_VPS_SSH_* if needed.
set -euo pipefail

SSH_HOST="${AINORY_VPS_SSH_HOST:-169.254.1.2}"
SSH_USER="${AINORY_VPS_SSH_USER:-root}"
SSH_KEY="${AINORY_VPS_SSH_KEY:-.deploy/id_ed25519}"
SSH_PORT="${AINORY_VPS_SSH_PORT:-22}"

ssh -i "$SSH_KEY" -p "$SSH_PORT" -o StrictHostKeyChecking=accept-new "$SSH_USER@$SSH_HOST" '
  echo "## hostnamectl";        hostnamectl 2>/dev/null
  echo "## os-release";         cat /etc/os-release 2>/dev/null
  echo "## memory / disk";      free -h; df -hT /
  echo "## cpu / uptime";       nproc; uptime
  echo "## listening sockets";  ss -tlnp 2>/dev/null | sort
  echo "## running services";   systemctl list-units --type=service --state=running --no-pager 2>/dev/null
  echo "## proxy/runtime bins"; for b in podman docker caddy nginx httpd cargo dx; do printf "%s: " "$b"; command -v "$b" || echo "(absent)"; done
  echo "## firewall (firewalld on Rocky)"; (firewall-cmd --list-all 2>/dev/null || ufw status verbose 2>/dev/null || iptables -S 2>/dev/null | head -40 || echo "(no visibility)")
  echo "## existing Caddy vhosts"; grep -E "^[a-z0-9.:_-]+ \{" /etc/caddy/Caddyfile 2>/dev/null || echo "(no /etc/caddy/Caddyfile)"
  echo "## existing TLS certs"; ls -1 /var/lib/caddy/.local/share/caddy/certificates 2>/dev/null || ls -1 /etc/letsencrypt/live 2>/dev/null || echo "(none visible)"
'
