#!/bin/bash
# Minimal RSS discovery script - no external dependencies needed
# Fetches key feeds and creates story candidates with beat balancing

set -e

PAPERCLIP_API_URL="${PAPERCLIP_API_URL:-http://127.0.0.1:3100}"
PAPERCLIP_COMPANY_ID="${PAPERCLIP_COMPANY_ID}"
PAPERCLIP_RUN_ID="${PAPERCLIP_RUN_ID}"
PAPERCLIP_API_KEY="${PAPERCLIP_API_KEY}"
SOURCE_CHECKER_ID="${SOURCE_CHECKER_AGENT_ID:-f0817ec6-5733-46f4-a0f8-2762ccb1b8d8}"
GOAL_ID="${PAPERCLIP_GOAL_ID:-6efa1d2d-3c6a-47f1-bc88-f6fb99fb2042}"

# Feeds to scan (beat, name, url)
# Primary feeds + fallbacks for better beat coverage
declare -a FEEDS=(
  # Linux beat
  "linux:Phoronix:https://www.phoronix.com/rss.php"
  "linux:LWN:https://lwn.net/headlines/rss"
  # Tech beat (multiple sources for reliability)
  "tech:TechCrunch:https://techcrunch.com/feed"
  "tech:Hacker News:https://hnrss.org/frontpage?points=100"
  "tech:Ars Technica:https://feeds.arstechnica.com/arstechnica/gadgets"
  # Privacy beat
  "privacy:EFF:https://www.eff.org/rss/updates.xml"
  "privacy:Ars Security:https://feeds.arstechnica.com/arstechnica/security"
)

echo "======================================="
echo "Scanner Discovery Scan"
echo "======================================="

created_count=0
declare -A beat_counts
beat_counts["linux"]=0
beat_counts["tech"]=0
beat_counts["privacy"]=0
MAX_PER_BEAT=3

for feed_spec in "${FEEDS[@]}"; do
  IFS=':' read -r beat name url <<< "$feed_spec"

  # Skip beat if we've already got enough stories for this heartbeat
  if [ "${beat_counts[$beat]}" -ge "$MAX_PER_BEAT" ]; then
    echo "  [SKIP] $name - beat already has $MAX_PER_BEAT stories"
    continue
  fi

  echo "Fetching $name ($beat)..."

  # Fetch feed with timeout and retry
  response=""
  for attempt in 1 2; do
    response=$(curl -s --max-time 8 --connect-timeout 5 "$url" 2>/dev/null || echo "")
    [ -n "$response" ] && break
    [ "$attempt" -lt 2 ] && sleep 1
  done

  if [ -z "$response" ]; then
    echo "  [SKIP] Failed to fetch"
    continue
  fi

  # Extract titles - simple line-based approach
  # Process each line looking for title tags
  item_count=0
  prev_title=""
  while IFS= read -r line; do
    # Look for title tags (handle CDATA)
    if echo "$line" | grep -q '<title'; then
      # Remove XML tags and CDATA markers
      title=$(echo "$line" | sed 's/<title>//g; s/<\/title>//g; s/<!\[CDATA\[//g; s/\]\]>//g; s/^[[:space:]]*//; s/[[:space:]]*$//')

      # Skip empty titles, feed titles, or duplicates
      if [ -z "$title" ] || [ "$title" = "$prev_title" ] || [[ "$title" =~ ^(RSS|Atom|Feed|Headlines|Updates)$ ]]; then
        continue
      fi

      # Skip generic single-word titles
      word_count=$(echo "$title" | wc -w)
      if [ "$word_count" -lt 3 ]; then
        continue
      fi

      # Only take up to 3 per beat
      if [ "$item_count" -ge 3 ]; then
        break
      fi

      echo "  - $title"
      prev_title="$title"

      # Try to extract link from same or next line
      link=""
      if echo "$line" | grep -q '<link'; then
        link=$(echo "$line" | sed 's/<link>//g; s/<\/link>//g; s/^[[:space:]]*//; s/[[:space:]]*$//')
      fi

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
        beat_counts[$beat]=$((beat_counts[$beat] + 1))
        echo "    ✓ Created"
      else
        echo "    ✗ Failed"
      fi

      item_count=$((item_count + 1))
    fi
  done <<< "$response"
done

echo "======================================="
echo "Created $created_count story candidates"
echo "Beat coverage:"
echo "  Linux:  ${beat_counts[linux]} stories"
echo "  Tech:   ${beat_counts[tech]} stories"
echo "  Privacy: ${beat_counts[privacy]} stories"
echo "======================================="
