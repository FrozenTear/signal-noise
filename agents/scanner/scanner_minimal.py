#!/usr/bin/env python3
"""
Minimal Scanner using only standard library
Fetches RSS feeds and creates story candidates with published catalog deduplication
"""

import os
import sys
import json
import urllib.request
import urllib.parse
import xml.etree.ElementTree as ET
from urllib.error import URLError
from difflib import SequenceMatcher
from pathlib import Path
from datetime import datetime

# Paperclip API setup
PAPERCLIP_API_URL = os.environ.get("PAPERCLIP_API_URL", "http://127.0.0.1:3100")
PAPERCLIP_API_KEY = os.environ.get("PAPERCLIP_API_KEY")
PAPERCLIP_COMPANY_ID = os.environ.get("PAPERCLIP_COMPANY_ID")
PAPERCLIP_RUN_ID = os.environ.get("PAPERCLIP_RUN_ID")

PROJECT_ROOT = os.getcwd()
PUBLISHED_DIR = Path(PROJECT_ROOT) / "docs" / "published"

# Core beats to scan
CORE_BEATS = ["linux", "tech", "privacy", "climate"]

# Feed configuration (beat, name, url)
FEEDS = [
    # Linux beat
    ("linux", "Phoronix", "https://www.phoronix.com/rss.php"),
    ("linux", "LWN.net", "https://lwn.net/headlines/rss"),
    ("linux", "The Register — Open Source", "https://www.theregister.com/software/open_source/headlines.atom"),

    # Tech beat
    ("tech", "Ars Technica — Tech", "https://feeds.arstechnica.com/arstechnica/gadgets"),
    ("tech", "Wired", "https://www.wired.com/feed/rss"),
    ("tech", "TechCrunch", "https://techcrunch.com/feed/"),
    ("tech", "Hacker News (100+)", "https://hnrss.org/frontpage?points=100"),
    ("tech", "The Verge", "https://www.theverge.com/rss/index.xml"),

    # Privacy beat
    ("privacy", "EFF Updates", "https://www.eff.org/rss/updates.xml"),
    ("privacy", "noyb", "https://noyb.eu/en/rss.xml"),
    ("privacy", "Ars Technica — Security", "https://feeds.arstechnica.com/arstechnica/security"),
    ("privacy", "EDRi", "https://edri.org/feed/"),
    ("privacy", "Patrick Breyer (MEP)", "https://www.patrick-breyer.de/en/feed/"),

    # Climate beat
    ("climate", "Carbon Brief", "https://www.carbonbrief.org/feed/"),
    ("climate", "Canary Media", "https://www.canarymedia.com/rss.rss"),
    ("climate", "Heatmap News", "https://heatmap.news/feeds/feed.rss"),
    ("climate", "E&E News Energy", "https://www.eenews.net/feed"),
    ("climate", "IEA News", "https://www.iea.org/news/feed.xml"),
]

def fetch_feed(url: str, timeout=10):
    """Fetch and parse an RSS/Atom feed using standard library."""
    try:
        with urllib.request.urlopen(url, timeout=timeout) as response:
            content = response.read()

        # Parse XML
        root = ET.fromstring(content)
        entries = []

        # Handle both RSS and Atom formats
        ns = {
            'atom': 'http://www.w3.org/2005/Atom',
            'content': 'http://purl.org/rss/1.0/modules/content/',
        }

        # Try RSS entries first
        for item in root.findall('.//item'):
            entry = {}
            title_elem = item.find('title')
            link_elem = item.find('link')
            desc_elem = item.find('description')
            pub_elem = item.find('pubDate')

            entry['title'] = title_elem.text if title_elem is not None else ''
            entry['link'] = link_elem.text if link_elem is not None else ''
            entry['summary'] = desc_elem.text if desc_elem is not None else ''
            entry['published'] = pub_elem.text if pub_elem is not None else ''

            if entry['title']:  # Only add if we have a title
                entries.append(entry)

        # Try Atom entries if no RSS found
        if not entries:
            for entry_elem in root.findall('atom:entry', ns):
                entry = {}
                title_elem = entry_elem.find('atom:title', ns)
                link_elem = entry_elem.find('atom:link', ns)
                summary_elem = entry_elem.find('atom:summary', ns)
                published_elem = entry_elem.find('atom:published', ns)
                content_elem = entry_elem.find('atom:content', ns)

                entry['title'] = title_elem.text if title_elem is not None else ''
                entry['link'] = link_elem.get('href') if link_elem is not None else ''
                entry['summary'] = (summary_elem.text if summary_elem is not None else '') or \
                                  (content_elem.text if content_elem is not None else '')
                entry['published'] = published_elem.text if published_elem is not None else ''

                if entry['title']:
                    entries.append(entry)

        return entries[:10]  # Return last 10 entries
    except Exception as e:
        print(f"Error fetching {url}: {e}")
        return []

def similarity_ratio(a: str, b: str) -> float:
    """Calculate string similarity."""
    return SequenceMatcher(None, a.lower(), b.lower()).ratio()

def get_published_slugs():
    """Load all published article slugs from catalog."""
    slugs = {}
    if not PUBLISHED_DIR.exists():
        return slugs

    for slug_dir in PUBLISHED_DIR.iterdir():
        if slug_dir.is_dir():
            publish_file = slug_dir / "publish.json"
            if publish_file.exists():
                try:
                    with open(publish_file) as f:
                        data = json.load(f)
                        slugs[slug_dir.name] = {
                            'title': data.get('title', ''),
                            'summary': data.get('summary', ''),
                            'sources': data.get('sources', [])
                        }
                except:
                    pass

    return slugs

def check_published_duplicate(entry: dict, published_slugs: dict) -> tuple:
    """
    Check if entry duplicates a published article.
    Returns (is_duplicate, matched_slug)
    """
    entry_url = entry.get('link', '').strip('/').lower()
    entry_title = entry.get('title', '').lower()

    for slug, article_data in published_slugs.items():
        # Pass A.1: Exact URL match (most important)
        article_sources = article_data.get('sources', [])
        for source in article_sources:
            source_url = source.get('url', '').strip('/').lower()
            if source_url and source_url == entry_url:
                return True, slug, 'url_match'

        # Pass A.2: Slug match (strip the-<N>- prefix)
        stripped_slug = slug.split('-', 2)[-1] if '-' in slug else slug
        entry_slug = entry_title.replace(' ', '-')[:50]
        if similarity_ratio(stripped_slug, entry_slug) > 0.9:
            return True, slug, 'slug_match'

        # Pass B: Semantic similarity
        article_title = article_data.get('title', '').lower()
        if similarity_ratio(entry_title, article_title) > 0.85:
            return True, slug, 'title_similarity'

    return False, None, None

def validate_candidate(entry: dict) -> tuple:
    """Validate required fields. Returns (is_valid, error_msg)"""
    url = entry.get('link', '').strip()
    headline = entry.get('title', '').strip()
    lead = entry.get('summary', '').strip()

    if not url:
        return False, "missing url"
    if not url.startswith(('http://', 'https://')):
        return False, "url not http/https"
    if not headline:
        return False, "missing headline"
    if not lead or len(lead) < 10:
        return False, "missing/short lead"

    return True, None

def rank_candidates(entries: list) -> list:
    """Rank candidates by newsworthiness."""
    scored = []

    for entry in entries:
        score = 1.0
        title = (entry.get('title', '') + ' ' + entry.get('summary', '')).lower()

        # Boost for newsworthiness
        if any(w in title for w in ['kernel', 'release', 'announced', 'new', 'critical']):
            score += 0.3
        if any(w in title for w in ['breaking', 'major', 'significant']):
            score += 0.2
        if any(w in title for w in ['unexpected', 'drama', 'chaos']):
            score += 0.15

        # Reduce for press releases
        if any(w in title for w in ['press release', 'announces']):
            score -= 0.2

        scored.append((entry, score))

    scored.sort(key=lambda x: x[1], reverse=True)
    return scored

def create_story_candidate(entry: dict, beat: str, score: float):
    """Create story candidate in Paperclip."""
    try:
        SOURCE_CHECKER_ID = os.environ.get(
            "SOURCE_CHECKER_AGENT_ID",
            "f2b27630-e4e6-4eab-9658-630f3a808375"
        )

        title = f"[{beat.upper()}] {entry['title']}"
        description = f"""## Story Candidate
- **Beat**: {beat}
- **Source URL**: {entry.get('link', '')}
- **Relevance Score**: {score:.2f}

## Summary
{entry.get('summary', '')[:500]}
"""

        data = {
            "title": title,
            "description": description,
            "status": "todo",
            "priority": "medium",
            "assigneeAgentId": SOURCE_CHECKER_ID,
        }

        if os.environ.get("PAPERCLIP_GOAL_ID"):
            data["goalId"] = os.environ.get("PAPERCLIP_GOAL_ID")

        body = json.dumps(data).encode('utf-8')

        req = urllib.request.Request(
            f"{PAPERCLIP_API_URL}/api/companies/{PAPERCLIP_COMPANY_ID}/issues",
            data=body,
            headers={
                "Authorization": f"Bearer {PAPERCLIP_API_KEY}",
                "X-Paperclip-Run-Id": PAPERCLIP_RUN_ID,
                "Content-Type": "application/json"
            }
        )

        with urllib.request.urlopen(req, timeout=10) as response:
            result = json.load(response)
            return result.get('identifier') or result.get('id')
    except Exception as e:
        print(f"Error creating issue: {e}")
        return None

def scan_beat(beat: str):
    """Scan feeds for a beat and create candidates."""
    print(f"\n{'='*60}")
    print(f"Scanning {beat.upper()} beat...")
    print(f"{'='*60}")

    beat_feeds = [f for f in FEEDS if f[0] == beat]
    print(f"Found {len(beat_feeds)} feeds for {beat} beat")

    all_entries = []

    # Fetch all feeds
    for beat_name, feed_name, feed_url in beat_feeds:
        print(f"  Fetching {feed_name}...")
        entries = fetch_feed(feed_url)
        all_entries.extend(entries)

    print(f"Total entries fetched: {len(all_entries)}")

    # Load published catalog
    published_slugs = get_published_slugs()
    print(f"Published catalog: {len(published_slugs)} articles loaded")

    # Validate
    validated = []
    for entry in all_entries:
        is_valid, err = validate_candidate(entry)
        if is_valid:
            validated.append(entry)

    print(f"Candidates after validation: {len(validated)}")

    # Check published duplicates
    filtered = []
    dup_count = 0
    for entry in validated:
        is_dup, matched_slug, match_type = check_published_duplicate(entry, published_slugs)
        if is_dup:
            dup_count += 1
            print(f"  Dedup: {entry['title'][:50]}... → {matched_slug} ({match_type})")
        else:
            filtered.append(entry)

    if dup_count > 0:
        print(f"Dropped {dup_count} published duplicates")
    print(f"Candidates after published dedup: {len(filtered)}")

    # Rank and limit
    ranked = rank_candidates(filtered)
    top = ranked[:3]  # max 3 per beat

    print(f"Top {len(top)} candidates to create:")

    created = []
    for entry, score in top:
        print(f"  - {score:.2f}: {entry['title'][:60]}...")
        issue_id = create_story_candidate(entry, beat, score)
        if issue_id:
            created.append(issue_id)
            print(f"    ✓ {issue_id}")
        else:
            print(f"    ✗ Failed")

    return created

def main():
    if not all([PAPERCLIP_API_KEY, PAPERCLIP_COMPANY_ID]):
        print("Error: Missing Paperclip environment variables")
        sys.exit(1)

    print(f"Scanner Minimal Heartbeat")
    print(f"Scanning beats: {', '.join(CORE_BEATS)}")

    all_created = []
    for beat in CORE_BEATS:
        created = scan_beat(beat)
        all_created.extend(created)

    print(f"\n{'='*60}")
    print(f"Summary: Created {len(all_created)} story candidates")
    if all_created:
        print(f"Issues: {', '.join(all_created)}")
    print(f"{'='*60}\n")

if __name__ == "__main__":
    main()
