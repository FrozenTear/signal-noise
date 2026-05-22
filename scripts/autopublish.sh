#!/usr/bin/env bash
# Autonomous publish — AGENT SIDE trigger (THE-233 / closes THE-227 Path A).
#
# Run from an agent sandbox (or a Paperclip routine). It:
#   1. (SYNC=1, default) SSHes to the VPS and fast-forwards the deployed source
#      tree to origin/master, so newly-merged docs/published/<slug>/publish.json
#      artifacts are on the host — same fetch/reset leg proven in redeploy-host.sh.
#   2. Pipes scripts/autopublish-host.sh over the SAME ssh and runs it on the
#      host, which sources the host-local bearer and POSTs any not-yet-live
#      articles to http://127.0.0.1:8888/api/articles.
#
# The token is sourced and used entirely on the host; it is never read, printed,
# or transmitted by this side. No GitHub secret, no operator step, no human.
#
# Env:
#   HOST   ssh target            (default root@169.254.1.2)
#   KEY    ssh identity          (default ~/.ssh/ainory_deploy)
#   SYNC   1 = git reset master first (default 1); 0 = sweep src tree as-is
#   FORCE  1 = re-POST even if a slug is already live (default 0)
#   ONLY   publish only this one slug
set -uo pipefail

HOST="${HOST:-root@169.254.1.2}"
KEY="${KEY:-$HOME/.ssh/ainory_deploy}"
SYNC="${SYNC:-1}"
FORCE="${FORCE:-0}"
ONLY="${ONLY:-}"
SRC=/opt/ainory-times/src
HERE="$(cd "$(dirname "$0")" && pwd)"

SSH=(ssh -i "$KEY" -o StrictHostKeyChecking=accept-new -o ConnectTimeout=20)

echo "== autopublish trigger: host=$HOST sync=$SYNC force=$FORCE only=${ONLY:-<all>} =="

if [ "$SYNC" = "1" ]; then
  echo "-- syncing $SRC to origin/master --"
  "${SSH[@]}" "$HOST" "cd $SRC && git fetch --all --prune -q && git reset --hard origin/master -q && git rev-parse --short HEAD" \
    || { echo "!! git sync failed"; exit 1; }
fi

echo "-- running host-side publish sweep --"
# Stream the host script over ssh so we don't depend on it being deployed yet,
# and so the version run always matches this repo. Pass tunables via env.
"${SSH[@]}" "$HOST" "FORCE='$FORCE' ONLY='$ONLY' bash -s" < "$HERE/autopublish-host.sh"
rc=$?
echo "== trigger exit: $rc =="
exit $rc
