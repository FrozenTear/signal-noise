#!/bin/bash
# Minimal RSS discovery script - no external dependencies needed
# Fetches key feeds and creates story candidates

set -e

PAPERCLIP_API_URL="${PAPERCLIP_API_URL:-http://127.0.0.1:3100}"
PAPERCLIP_COMPANY_ID="${PAPERCLIP_COMPANY_ID}"
PAPERCLIP_RUN_ID="${PAPERCLIP_RUN_ID}"
PAPERCLIP_API_KEY="${PAPERCLIP_API_KEY}"
SOURCE_CHECKER_ID="${SOURCE_CHECKER_AGENT_ID:-f0817ec6-5733-46f4-a0f8-2762ccb1b8d8}"
GOAL_ID="${PAPERCLIP_GOAL_ID:-6efa1d2d-3c6a-47f1-bc88-f6fb99fb2042}"

# Feeds to scan (beat, name, url)
declare -a FEEDS=(
  "linux:Phoronix:https://www.phoronix.com/rss.php"
  "tech:TechCrunch:https://techcrunch.com/feed"
  "privacy:EFF:https://www.eff.org/rss/updates.xml"
)

echo "======================================="
echo "Scanner Discovery Scan"
echo "======================================="

created_count=0

for feed_spec in "${FEEDS[@]}"; do
  IFS=':' read -r beat name url <<< "$feed_spec"

  echo "Fetching $name ($beat)..."

  # Fetch feed and extract first 3 items
  response=$(curl -s --max-time 10 "$url" 2>/dev/null || echo "")

  if [ -z "$response" ]; then
    echo "  [SKIP] Failed to fetch"
    continue
  fi

  # Extract titles and links - simple regex approach
  # This is crude but works for most RSS feeds
  item_count=0
  while IFS= read -r line; do
    if [[ "$line" =~ \<title\>(.*)\<\/title\> && "$item_count" -lt 3 ]]; then
      title="${BASH_REMATCH[1]}"
      # Skip feed title (it's the first one and generic)
      if [[ ! "$title" =~ ^[A-Z][a-z]+\$ ]]; then
        echo "  - $title"

        # Try to extract link
        link=""
        while IFS= read -r link_line; do
          if [[ "$link_line" =~ \<link\>(.*?)\<\/link\> ]]; then
            link="${BASH_REMATCH[1]}"
            break
          fi
        done <<< "$response"

        # Create story candidate
        beat_upper=$(echo "$beat" | tr '[:lower:]' '[:upper:]')
        issue_title="[$beat_upper] $title"

        issue_json=$(cat <<EOF
{
  "title": "$issue_title",
  "description": "## Story Details\n- Beat: $beat\n- Source: $name\n- URL: $link\n\n## Summary\n$title",
  "status": "todo",
  "priority": "medium",
  "assigneeAgentId": "$SOURCE_CHECKER_ID",
  "goalId": "$GOAL_ID"
}
EOF
)

        # Create issue via API
        result=$(curl -s -X POST \
          "$PAPERCLIP_API_URL/api/companies/$PAPERCLIP_COMPANY_ID/issues" \
          -H "Authorization: Bearer $PAPERCLIP_API_KEY" \
          -H "X-Paperclip-Run-Id: $PAPERCLIP_RUN_ID" \
          -H "Content-Type: application/json" \
          -d "$issue_json" 2>/dev/null || echo "{}")

        if echo "$result" | grep -q '"id"'; then
          created_count=$((created_count + 1))
          echo "    ✓ Created"
        else
          echo "    ✗ Failed"
        fi

        item_count=$((item_count + 1))
      fi
    fi
  done <<< "$response"
done

echo "======================================="
echo "Created $created_count story candidates"
echo "======================================="
