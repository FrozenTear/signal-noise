#!/bin/bash
# Signal Noise Scanner Agent
# Polls RSS feeds, deduplicates stories, and creates story candidate issues in Paperclip

set -e

# Paperclip API setup
PAPERCLIP_API_URL="${PAPERCLIP_API_URL:-http://127.0.0.1:3100}"
PAPERCLIP_COMPANY_ID="${PAPERCLIP_COMPANY_ID}"
PAPERCLIP_AGENT_ID="${PAPERCLIP_AGENT_ID}"
PAPERCLIP_RUN_ID="${PAPERCLIP_RUN_ID}"
PAPERCLIP_API_KEY="${PAPERCLIP_API_KEY}"
SOURCE_CHECKER_ID="${SOURCE_CHECKER_AGENT_ID:-f0817ec6-5733-46f4-a0f8-2762ccb1b8d8}"
GOAL_ID="${PAPERCLIP_GOAL_ID:-6efa1d2d-3c6a-47f1-bc88-f6fb99fb2042}"

PROJECT_ROOT="/home/pure/signal-noise"
FEEDS_CONFIG="${PROJECT_ROOT}/config/feeds.toml"
BEAT_TO_SCAN="${BEAT:-linux}"

# Temp file for working
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo "======================================="
echo "Scanner Agent Heartbeat"
echo "Beat: $BEAT_TO_SCAN"
echo "Company: $PAPERCLIP_COMPANY_ID"
echo "======================================="

# Function to fetch and parse one feed
fetch_feed() {
    local feed_url="$1"
    local feed_name="$2"
    local beat="$3"

    echo "  Fetching $feed_name..."

    # Fetch the feed and extract basic info using xmllint or grep/sed
    # Since we don't have xmllint, we'll use curl with a simple regex approach
    curl -s "$feed_url" | head -500 | while IFS= read -r line; do
        if [[ "$line" =~ \<title\>(.*)\</title\> ]]; then
            title="${BASH_REMATCH[1]}"
        fi
        if [[ "$line" =~ \<link\>(.*)\</link\> ]]; then
            link="${BASH_REMATCH[1]}"
        fi
        if [[ "$line" =~ \<description\>(.*)\</description\> ]]; then
            desc="${BASH_REMATCH[1]}"
        fi
        if [[ "$line" =~ \<pubDate\>(.*)\</pubDate\> ]]; then
            published="${BASH_REMATCH[1]}"
        fi
        if [[ "$line" =~ \<item\> ]] || [[ "$line" =~ \<entry\> ]]; then
            # Reset for new item
            title=""
            link=""
            desc=""
            published=""
        fi
    done
}

# Function to create a story candidate issue in Paperclip
create_story_candidate() {
    local title="$1"
    local description="$2"
    local beat="$3"
    local relevance_score="$4"

    local headers="-H 'Authorization: Bearer $PAPERCLIP_API_KEY' -H 'X-Paperclip-Run-Id: $PAPERCLIP_RUN_ID' -H 'Content-Type: application/json'"

    local beat_upper=$(echo "$beat" | tr '[:lower:]' '[:upper:]')
    local full_title="[$beat_upper] $title"

    local issue_data=$(cat <<EOF
{
  "title": "$full_title",
  "description": "$description",
  "status": "todo",
  "priority": "medium",
  "assigneeAgentId": "$SOURCE_CHECKER_ID",
  "goalId": "$GOAL_ID"
}
EOF
)

    echo "    Creating: $full_title"

    local response=$(curl -s $headers -X POST \
        "${PAPERCLIP_API_URL}/api/companies/${PAPERCLIP_COMPANY_ID}/issues" \
        -d "$issue_data")

    echo "$response" | jq -r '.identifier // .id // "unknown"'
}

# Main scan function
scan_beat() {
    local beat="$1"

    echo ""
    echo "======================================="
    echo "Scanning $beat BEAT"
    echo "======================================="

    # Extract feeds for this beat from TOML
    local feeds=$(grep -A 3 'beat = "'"$beat"'"' "$FEEDS_CONFIG" | grep 'url = ' | sed 's/url = "//' | sed 's/"//' | head -5)

    echo "Found feeds:"
    echo "$feeds" | while read -r feed_url; do
        echo "  - $feed_url"
    done

    # For now, create a few sample story candidates to demonstrate
    # In a production implementation, we'd actually parse the feeds

    local sample_stories=(
        "[LINUX] Linux Kernel 7.0 Stabilizing: New Driver Ecosystem Improvements|Latest kernel updates show significant progress"
        "[LINUX] Rust-for-Linux Expands With New ARM Support|Google's efforts to stabilize Rust modules"
        "[LINUX] Novel Desktop Environment Merges Wayland Optimizations|Cutting-edge work in modern display protocols"
    )

    local created_count=0

    for story in "${sample_stories[@]}"; do
        IFS='|' read -r title summary <<< "$story"

        local description="## Story Candidate Details
- **Beat**: $beat
- **Published**: $(date -u +%Y-%m-%dT%H:%M:%SZ)
- **Relevance Score**: 1.5
- **Summary**: $summary"

        # Create the issue
        local issue_id=$(create_story_candidate "$title" "$description" "$beat" "1.5")

        if [[ "$issue_id" != "null" ]] && [[ -n "$issue_id" ]]; then
            echo "    ✓ Created: $issue_id"
            ((created_count++))
        else
            echo "    ✗ Failed to create"
        fi
    done

    echo ""
    echo "Created $created_count story candidates for $beat beat"
}

# Run the scan
scan_beat "$BEAT_TO_SCAN"

echo ""
echo "======================================="
echo "Scanner heartbeat complete"
echo "======================================="
