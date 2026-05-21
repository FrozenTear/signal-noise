#!/usr/bin/env bash
# Signal Noise — roll back to the previous release dir.
# Flips /opt/ainory-times/current one release back and restarts the service.
set -euo pipefail

HOST="${AINORY_VPS_HOST:?set AINORY_VPS_HOST}"
SSH_USER="${AINORY_VPS_SSH_USER:-ts6dev}"
SSH_KEY="${AINORY_VPS_SSH_KEY:-.deploy/id_ed25519}"
SSH_PORT="${AINORY_VPS_SSH_PORT:-22}"
SSH=(ssh -i "$SSH_KEY" -p "$SSH_PORT" -o StrictHostKeyChecking=accept-new "$SSH_USER@$HOST")

"${SSH[@]}" '
  set -e
  APP_DIR=/opt/ainory-times
  CUR=$(readlink -f "$APP_DIR/current" || true)
  PREV=$(ls -1dt "$APP_DIR"/releases/* | grep -vx "$CUR" | head -1)
  [ -n "$PREV" ] || { echo "no previous release to roll back to"; exit 1; }
  echo "rolling back: $CUR -> $PREV"
  sudo ln -snf "$PREV" "$APP_DIR/current"
  sudo systemctl restart ainory-times
  for i in $(seq 1 10); do curl -fsS http://127.0.0.1:8888/ >/dev/null && { echo OK; exit 0; }; sleep 1; done
  echo "health check FAILED after rollback"; exit 1
'
