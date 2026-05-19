#!/bin/bash
# Signal Noise Scanner - Minimal RSS discovery with proper JSON encoding
# Fetches key feeds and creates story candidates with beat balancing

set -e

PAPERCLIP_API_URL="${PAPERCLIP_API_URL:-http://127.0.0.1:3100}"
PAPERCLIP_COMPANY_ID="${PAPERCLIP_COMPANY_ID}"
PAPERCLIP_RUN_ID="${PAPERCLIP_RUN_ID}"
PAPERCLIP_API_KEY="${PAPERCLIP_API_KEY}"
SOURCE_CHECKER_ID="${SOURCE_CHECKER_AGENT_ID}"
GOAL_ID="${PAPERCLIP_GOAL_ID}"

# Validate required environment variables
if [ -z "$PAPERCLIP_API_KEY" ] || [ -z "$PAPERCLIP_COMPANY_ID" ]; then
    echo "Error: Missing required Paperclip environment variables"
    exit 1
fi

# Feeds to scan (beat:name:url)
declare -a FEEDS=(
  # Linux beat
  "linux:Phoronix:https://www.phoronix.com/rss.php"
  "linux:LWN:https://lwn.net/headlines/rss"
  # Tech beat
  "tech:TechCrunch:https://techcrunch.com/feed"
  "tech:Hacker News:https://hnrss.org/frontpage?points=100"
  "tech:Ars Technica:https://feeds.arstechnica.com/arstechnica/gadgets"
  # Privacy beat
  "privacy:EFF:https://www.eff.org/rss/updates.xml"
  "privacy:Ars Security:https://feeds.arstechnica.com/arstechnica/security"
  # AI in Society beat
  "ai_society:MIT Technology Review:https://www.technologyreview.com/feed/"
  "ai_society:Future of Life Institute:https://futureoflife.org/feed/"
  "ai_society:Stanford HAI:https://hai.stanford.edu/news/rss"
  # AI Policy and Regulation beat
  "ai_policy:Algorithm Watch:https://algorithmwatch.org/en/feed/"
  "ai_policy:CDT:https://cdt.org/feed/"
  "ai_policy:Brookings TechStream:https://www.brookings.edu/topic/technology-innovation/feed/"
  # AI in Creative Industries beat
  "ai_creative:404 Media:https://www.404media.co/rss/"
  "ai_creative:The Markup:https://themarkup.org/feeds/rss.xml"
  "ai_creative:Waxy.org:https://waxy.org/feed/"
)

echo "======================================="
echo "Scanner Discovery Scan"
echo "API: $PAPERCLIP_API_URL"
echo "Company: $PAPERCLIP_COMPANY_ID"
echo "======================================="

created_count=0
declare -A beat_counts
beat_counts["linux"]=0
beat_counts["tech"]=0
beat_counts["privacy"]=0
beat_counts["ai_society"]=0
beat_counts["ai_policy"]=0
beat_counts["ai_creative"]=0
MAX_PER_BEAT=3

# Helper function to properly escape JSON strings
escape_json() {
    local s="$1"
    # Escape special JSON characters
    s="${s//\\/\\\\}"  # backslash
    s="${s//\"/\\\"}"  # quote
    s="${s//$'\n'/\\n}" # newline
    s="${s//$'\r'/\\r}" # carriage return
    s="${s//$'\t'/\\t}" # tab
    echo "$s"
}

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
  item_count=0
  prev_title=""
  while IFS= read -r line; do
    # Look for title tags (handle CDATA)
    if echo "$line" | grep -q '<title'; then
      # Remove XML tags and CDATA markers
      title=$(echo "$line" | sed 's/<title>//g; s/<\/title>//g; s/<!\[CDATA\[//g; s/\]\]>//g; s/^[[:space:]]*//; s/[[:space:]]*$//')

      # Decode HTML entities
      title=$(echo "$title" | sed 's/&amp;/\&/g; s/&lt;/</g; s/&gt;/>/g; s/&quot;/"/g; s/&#039;/'\''/g; s/&nbsp;/ /g')

      # Skip empty titles, feed titles, or duplicates
      if [ -z "$title" ] || [ "$title" = "$prev_title" ] || [[ "$title" =~ ^(RSS|Atom|Feed|Headlines|Updates|<html|404)$ ]]; then
        continue
      fi

      # Skip if title looks like HTML or markup
      if echo "$title" | grep -q '<.*>'; then
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

      # Try to extract link
      link=""
      if echo "$line" | grep -q '<link'; then
        link=$(echo "$line" | sed 's/<link>//g; s/<\/link>//g; s/^[[:space:]]*//; s/[[:space:]]*$//')
      fi

      # Create story candidate with proper JSON encoding
      beat_upper=$(echo "$beat" | tr '[:lower:]' '[:upper:]')
      issue_title="[$beat_upper] $title"

      # Escape values for JSON
      escaped_title=$(escape_json "$issue_title")
      escaped_description=$(escape_json "## Story Details\n- Beat: $beat\n- Source: $name\n- URL: $link\n\n## Summary\n$title")

      # Create JSON payload - use printf to ensure proper formatting
      if [ -n "$GOAL_ID" ]; then
        json_payload=$(printf '{"title":"%s","description":"%s","status":"todo","priority":"medium","assigneeAgentId":"%s","goalId":"%s"}' \
          "$escaped_title" "$escaped_description" "$SOURCE_CHECKER_ID" "$GOAL_ID")
      else
        json_payload=$(printf '{"title":"%s","description":"%s","status":"todo","priority":"medium","assigneeAgentId":"%s"}' \
          "$escaped_title" "$escaped_description" "$SOURCE_CHECKER_ID")
      fi

      # Create issue via API
      result=$(curl -s -X POST \
        "$PAPERCLIP_API_URL/api/companies/$PAPERCLIP_COMPANY_ID/issues" \
        -H "Authorization: Bearer $PAPERCLIP_API_KEY" \
        -H "X-Paperclip-Run-Id: $PAPERCLIP_RUN_ID" \
        -H "Content-Type: application/json" \
        -d "$json_payload" 2>&1)

      if echo "$result" | grep -q '"id"'; then
        created_count=$((created_count + 1))
        beat_counts[$beat]=$((beat_counts[$beat] + 1))
        issue_id=$(echo "$result" | grep -o '"identifier":"[^"]*"' | cut -d'"' -f4)
        [ -z "$issue_id" ] && issue_id=$(echo "$result" | grep -o '"id":"[^"]*"' | head -1 | cut -d'"' -f4)
        echo "    ✓ Created: $issue_id"
      else
        # Debug: show first attempt's error
        if [ "$item_count" -eq 1 ]; then
          echo "    ✗ Failed (API error: $(echo "$result" | head -c 100))"
        else
          echo "    ✗ Failed"
        fi
      fi

      item_count=$((item_count + 1))
    fi
  done <<< "$response"
done

echo "======================================="
echo "Created $created_count story candidates"
echo "Beat coverage:"
echo "  Linux:      ${beat_counts[linux]} stories"
echo "  Tech:       ${beat_counts[tech]} stories"
echo "  Privacy:    ${beat_counts[privacy]} stories"
echo "  AI Society: ${beat_counts[ai_society]} stories"
echo "  AI Policy:  ${beat_counts[ai_policy]} stories"
echo "  AI Creative:${beat_counts[ai_creative]} stories"
echo "======================================="

exit 0
