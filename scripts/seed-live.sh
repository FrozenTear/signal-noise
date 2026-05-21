#!/usr/bin/env bash
# seed-live.sh — HTTP-based article seeder for a running signal-noise instance.
#
# POSTs docs/published/<slug>/publish.json to the API.
# Requires SEED_API_TOKEN set in env — never commit the token.
#
# Usage:
#   SEED_API_TOKEN=<token> ./scripts/seed-live.sh <slug>
#   SEED_API_TOKEN=<token> SIGNAL_NOISE_URL=https://example.com ./scripts/seed-live.sh <slug>

set -euo pipefail

SLUG="${1:-}"
if [[ -z "$SLUG" ]]; then
    echo "Usage: $0 <slug>" >&2
    exit 1
fi

PUBLISH_JSON="docs/published/${SLUG}/publish.json"
if [[ ! -f "$PUBLISH_JSON" ]]; then
    echo "ERROR: $PUBLISH_JSON not found" >&2
    exit 1
fi

if [[ -z "${SEED_API_TOKEN:-}" ]]; then
    echo "ERROR: SEED_API_TOKEN is not set" >&2
    exit 1
fi

SIGNAL_NOISE_URL="${SIGNAL_NOISE_URL:-http://localhost:8888}"
API_URL="${SIGNAL_NOISE_URL}/api/articles"

echo "Seeding '${SLUG}' → ${API_URL} ..."
response=$(curl -sf \
    -X POST \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer ${SEED_API_TOKEN}" \
    --data-binary "@${PUBLISH_JSON}" \
    "${API_URL}")

echo "Response: ${response}"
echo "Done."
