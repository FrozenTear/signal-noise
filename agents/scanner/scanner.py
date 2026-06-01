#!/usr/bin/env python3
"""
Signal Noise Scanner Agent
Polls RSS feeds, deduplicates stories, and creates story candidate issues in Paperclip.
"""

import os
import sys
import json
import requests
import feedparser
import re
from datetime import datetime, timedelta
from typing import Optional, List, Dict, Tuple
from difflib import SequenceMatcher
import toml

from published_dedupe import PublishedCatalogDedupe

# Paperclip API setup
PAPERCLIP_API_URL = os.environ.get("PAPERCLIP_API_URL", "http://127.0.0.1:3100")
PAPERCLIP_API_KEY = os.environ.get("PAPERCLIP_API_KEY")
PAPERCLIP_COMPANY_ID = os.environ.get("PAPERCLIP_COMPANY_ID")
PAPERCLIP_AGENT_ID = os.environ.get("PAPERCLIP_AGENT_ID")
PAPERCLIP_RUN_ID = os.environ.get("PAPERCLIP_RUN_ID")

PROJECT_ROOT = os.environ.get("PWD", "/home/soot/github/signal-noise")
FEEDS_CONFIG = f"{PROJECT_ROOT}/config/feeds.toml"


def load_feeds_config() -> Dict:
    """Load feeds configuration from feeds.toml"""
    with open(FEEDS_CONFIG, 'r') as f:
        config = toml.load(f)
    return config


def fetch_feed(feed_url: str, timeout=10) -> List[Dict]:
    """Fetch and parse an RSS/Atom feed."""
    try:
        response = requests.get(feed_url, timeout=timeout)
        response.raise_for_status()
        parsed = feedparser.parse(response.content)

        if parsed.bozo:
            print(f"Warning: Feed parsing issues for {feed_url}: {parsed.bozo_exception}")

        entries = []
        for entry in parsed.entries[:10]:  # Limit to last 10 entries per feed
            entries.append({
                'title': entry.get('title', ''),
                'link': entry.get('link', ''),
                'summary': entry.get('summary', ''),
                'published': entry.get('published', ''),
                'author': entry.get('author', ''),
            })

        return entries
    except Exception as e:
        print(f"Error fetching feed {feed_url}: {e}")
        return []


def similarity_ratio(a: str, b: str) -> float:
    """Calculate string similarity using SequenceMatcher."""
    return SequenceMatcher(None, a.lower(), b.lower()).ratio()


def get_existing_candidates(beat: str) -> List[Dict]:
    """Fetch existing story candidates for a beat from Paperclip."""
    try:
        headers = {"Authorization": f"Bearer {PAPERCLIP_API_KEY}"}
        # Get story candidates created in the last 48 hours
        response = requests.get(
            f"{PAPERCLIP_API_URL}/api/companies/{PAPERCLIP_COMPANY_ID}/issues",
            params={
                "q": f"[{beat.upper()}]",
                "status": "todo,in_progress,done,in_review"
            },
            headers=headers,
            timeout=10
        )
        response.raise_for_status()
        return response.json() if isinstance(response.json(), list) else []
    except Exception as e:
        print(f"Error fetching existing candidates: {e}")
        return []


def validate_candidate(entry: Dict) -> Tuple[bool, Optional[str]]:
    """Validate that a candidate has all required fields for filing.

    Required fields:
    - url: non-empty http/https URL (from entry['link'])
    - headline: non-empty string (from entry['title'])
    - lead: 1-2 sentence summary (from entry['summary'])

    Returns: (is_valid, error_message)
    """
    url = entry.get('link', '').strip()
    headline = entry.get('title', '').strip()
    lead = entry.get('summary', '').strip()

    if not url:
        return False, "missing url"
    if not url.startswith(('http://', 'https://')):
        return False, "url is not http/https"
    if not headline:
        return False, "missing headline"
    if not lead:
        return False, "missing lead/summary"

    return True, None


def is_duplicate(entry: Dict, existing: List[Dict], threshold=0.85) -> Tuple[bool, Optional[str]]:
    """Check if entry is a duplicate of an existing candidate."""
    entry_title = entry.get('title', '')
    entry_url = entry.get('link', '')

    for candidate in existing:
        existing_title = candidate.get('title', '')
        existing_desc = candidate.get('description', '')

        # Exact URL match
        if entry_url and entry_url in existing_desc:
            return True, candidate.get('id')

        # Title similarity
        if similarity_ratio(entry_title, existing_title) > threshold:
            return True, candidate.get('id')

    return False, None


def check_published_catalog_duplicate(
    entry: Dict,
    dedupe: PublishedCatalogDedupe
) -> Tuple[bool, Optional[str], dict]:
    """
    Check if entry duplicates a published article.

    Args:
        entry: The candidate entry (with 'title', 'summary' fields)
        dedupe: PublishedCatalogDedupe instance

    Returns:
        Tuple of (is_duplicate, published_slug, debug_info)
    """
    headline = entry.get('title', '')
    summary = entry.get('summary', '')

    # Log the match keys for debugging
    full_key, stripped_key = dedupe.log_match_keys(headline, summary)

    # Check against published catalog
    is_dup, matched_slug, debug_info = dedupe.check_published_duplicate(
        headline,
        summary
    )

    if is_dup:
        print(f"  Published catalog match: {matched_slug}")
        print(f"    Match keys - Full: {full_key}, Stripped: {stripped_key}")
        print(f"    Confidence: {debug_info['confidence']}, Type: {debug_info['match_type']}")

    return is_dup, matched_slug, debug_info


def rank_candidates(entries: List[Dict], beat: str) -> List[Tuple[Dict, float]]:
    """Rank candidates by newsworthiness and comedyvalue."""
    scored = []

    for entry in entries:
        score = 1.0
        title = entry.get('title', '')
        summary = entry.get('summary', '')
        combined = (title + " " + summary).lower()

        # Boost for technical depth
        if any(word in combined for word in ['kernel', 'module', 'driver', 'api', 'protocol', 'architecture']):
            score += 0.3

        # Boost for recent developments
        if any(word in combined for word in ['new', 'announced', 'released', 'debut', 'launch', 'opens']):
            score += 0.2

        # Boost for multi-source potential (mentioned across feeds)
        if any(word in combined for word in ['major', 'significant', 'critical', 'breaking']):
            score += 0.15

        # Boost for comedy potential
        if any(word in combined for word in ['unexpected', 'chaos', 'drama', 'governance', 'war']):
            score += 0.2

        # Reduce for press releases / marketing
        if any(word in combined for word in ['press release', 'announces', 'proud', 'excited', 'pleased']):
            score -= 0.2

        # Reduce for opinion pieces
        if any(word in combined for word in ['opinion', 'analysis', 'thought', 'perspective']):
            score -= 0.1

        scored.append((entry, score))

    # Sort by score descending
    scored.sort(key=lambda x: x[1], reverse=True)
    return scored


def create_story_candidate(entry: Dict, beat: str, relevance_score: float) -> Optional[str]:
    """Create a story candidate issue in Paperclip."""
    try:
        # Get the Source Checker agent ID (must be a valid UUID)
        SOURCE_CHECKER_ID = os.environ.get("SOURCE_CHECKER_AGENT_ID", "f0817ec6-5733-46f4-a0f8-2762ccb1b8d8")

        title = f"[{beat.upper()}] {entry['title']}"

        # Format the description with story details
        description = f"""## Story Candidate Details
- **Source**: {entry.get('author', 'Unknown')}
- **Published**: {entry.get('published', 'Unknown')}
- **Relevance Score**: {relevance_score:.2f}
- **Source URLs**:
  - {entry.get('link', '')}

## Summary
{entry.get('summary', '')[:500]}
"""

        headers = {
            "Authorization": f"Bearer {PAPERCLIP_API_KEY}",
            "X-Paperclip-Run-Id": PAPERCLIP_RUN_ID
        }

        issue_data = {
            "title": title,
            "description": description,
            "status": "todo",
            "priority": "medium",
            "assigneeAgentId": SOURCE_CHECKER_ID,
            "goalId": os.environ.get("PAPERCLIP_GOAL_ID")  # Set in Paperclip
        }

        response = requests.post(
            f"{PAPERCLIP_API_URL}/api/companies/{PAPERCLIP_COMPANY_ID}/issues",
            json=issue_data,
            headers=headers,
            timeout=10
        )
        response.raise_for_status()
        result = response.json()
        return result.get('id') or result.get('identifier')
    except Exception as e:
        print(f"Error creating story candidate: {e}")
        return None


def scan_beat(beat: str, feeds_config: Dict) -> List[str]:
    """Scan feeds for a specific beat and create story candidates."""
    print(f"\n{'='*60}")
    print(f"Scanning {beat.upper()} beat...")
    print(f"{'='*60}")

    # Get feeds for this beat
    beat_feeds = [f for f in feeds_config.get('feed', []) if f.get('beat') == beat]
    print(f"Found {len(beat_feeds)} feeds for {beat} beat")

    all_entries = []

    # Fetch all entries from feeds
    for feed_config in beat_feeds:
        feed_url = feed_config.get('url')
        feed_name = feed_config.get('name')
        print(f"  Fetching {feed_name}...")

        entries = fetch_feed(feed_url)
        for entry in entries:
            entry['source_feed'] = feed_name
            entry['feed_type'] = feed_config.get('type')
        all_entries.extend(entries)

    print(f"Total entries fetched: {len(all_entries)}")

    # Get existing candidates to check for duplicates
    existing = get_existing_candidates(beat)
    print(f"Found {len(existing)} existing candidates in pipeline")

    # Initialize published catalog deduplicator
    dedupe = PublishedCatalogDedupe()

    # Validate required fields
    validated_entries = []
    invalid_count = 0
    for entry in all_entries:
        is_valid, error = validate_candidate(entry)
        if not is_valid:
            invalid_count += 1
        else:
            validated_entries.append(entry)

    if invalid_count > 0:
        print(f"Dropped {invalid_count} candidates with missing required fields (url/headline/lead)")
    print(f"Candidates after validation: {len(validated_entries)}")

    # Deduplicate against pipeline candidates
    filtered_entries = []
    for entry in validated_entries:
        is_dup, dup_id = is_duplicate(entry, existing)
        if not is_dup:
            filtered_entries.append(entry)

    print(f"Candidates after pipeline deduplication: {len(filtered_entries)}")

    # Deduplicate against published catalog
    catalog_filtered_entries = []
    published_dup_count = 0
    for entry in filtered_entries:
        is_dup, published_slug, debug_info = check_published_catalog_duplicate(entry, dedupe)
        if is_dup:
            published_dup_count += 1
        else:
            catalog_filtered_entries.append(entry)

    if published_dup_count > 0:
        print(f"Dropped {published_dup_count} candidates that duplicate published articles")
    print(f"Candidates after published catalog deduplication: {len(catalog_filtered_entries)}")

    # Rank remaining candidates
    ranked = rank_candidates(catalog_filtered_entries, beat)

    # Take top candidates (per-beat cap takes priority over global cap)
    max_per_beat = feeds_config.get('scanner', {}).get('max_candidates_per_beat', 3)
    top_candidates = ranked[:max_per_beat]

    print(f"Top {len(top_candidates)} candidates to create:")

    created_ids = []
    for entry, score in top_candidates:
        print(f"  - {score:.2f}: {entry['title'][:60]}...")
        issue_id = create_story_candidate(entry, beat, score)
        if issue_id:
            created_ids.append(issue_id)
            print(f"    ✓ Created: {issue_id}")
        else:
            print(f"    ✗ Failed to create issue")

    return created_ids


def main():
    """Main Scanner heartbeat entrypoint."""
    if not all([PAPERCLIP_API_KEY, PAPERCLIP_COMPANY_ID, PAPERCLIP_AGENT_ID]):
        print("Error: Missing Paperclip environment variables")
        sys.exit(1)

    # Load configuration
    config = load_feeds_config()

    # Determine which beats to scan
    # If BEAT environment variable is set, only scan that beat (for focused scans)
    beat_to_scan = os.environ.get("BEAT", "").lower()

    if beat_to_scan:
        beats = [beat_to_scan]
    else:
        # Default: scan all configured beats
        beats_set = set()
        for feed in config.get('feed', []):
            beats_set.add(feed.get('beat'))
        beats = sorted(beats_set)

    print(f"Scanner Agent Heartbeat")
    print(f"Scanning beats: {', '.join(beats)}")
    print(f"Company: {PAPERCLIP_COMPANY_ID}")
    print(f"Run ID: {PAPERCLIP_RUN_ID}")

    all_created = []
    for beat in beats:
        created = scan_beat(beat, config)
        all_created.extend(created)

    print(f"\n{'='*60}")
    print(f"Summary: Created {len(all_created)} new story candidate issues")
    if all_created:
        print(f"Issues: {', '.join(all_created)}")
    print(f"{'='*60}\n")

    return len(all_created)


if __name__ == "__main__":
    main()
