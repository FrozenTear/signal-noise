# The AIrony Times Scanner

The Scanner is the first agent in The AIrony Times editorial pipeline. It discovers news stories across six beats (Linux, Tech, Privacy, AI in Society, AI Policy and Regulation, and AI in Creative Industries) from multiple sources and surfaces them as story candidate issues for the Fact Checker.

## Architecture

The Scanner operates in heartbeat cycles:

1. **Check inbox** — Prioritize any assigned work from the editorial team
2. **Poll sources** — Fetch articles from RSS feeds and gnews.io API
3. **Deduplicate** — Remove duplicate stories (by headline similarity)
4. **Rank** — Score candidates by recency, priority, and content depth
5. **Create issues** — Generate Paperclip issues assigned to the Fact Checker

## Configuration

All configuration is in `config/feeds.toml`. Two main sections:

### [scanner] Section

Global scanner settings:

```toml
[scanner]
poll_interval_sec = 3600        # Time between polls (seconds)
dedup_threshold = 0.85          # Headline similarity threshold (0–1)
max_candidates_per_run = 10     # Max issues to create per heartbeat
```

### [[feed]] Entries (RSS)

Each `[[feed]]` block defines an RSS/Atom feed source:

```toml
[[feed]]
url = "https://www.phoronix.com/rss.php"
name = "Phoronix"               # Display name
beat = "linux"                  # One of: linux, tech, privacy
type = "blog"                   # Feed type: primary, wire, blog, aggregator
priority = "high"               # One of: high, medium, low
```

**Configured feeds:**
- **Linux & Open Source**: Phoronix, LWN.net, The Register (Open Source)
- **Tech**: Ars Technica (Tech), Wired, TechCrunch, Hacker News, The Verge
- **Privacy**: EFF Updates, noyb, Ars Technica (Security), EDRi, Patrick Breyer, EU Perspectives
- **AI in Society**: MIT Technology Review, Future of Life Institute, Stanford HAI
- **AI Policy and Regulation**: Algorithm Watch, Center for Democracy & Technology, Brookings TechStream
- **AI in Creative Industries**: 404 Media, The Markup, Waxy.org

## gnews.io Integration

The Scanner now supports gnews.io as a secondary discovery channel alongside RSS feeds.

### Setup

1. **Create gnews.io account**
   - Visit https://gnews.io
   - Sign up for free tier (100 requests/day, 10 articles/request)
   - Copy your API key from account dashboard

2. **Configure API key in feeds.toml**
   ```toml
   [gnews]
   api_key = "YOUR_API_KEY_HERE"
   tier = "free"  # or "essential" for production (€50/mo)
   queries = [
     { beat = "linux", q = "Linux kernel distribution open source", limit = 5 },
     { beat = "tech", q = "tech startup software engineering developer tools", limit = 5 },
     { beat = "privacy", q = "privacy surveillance data protection encryption", limit = 5 },
     { beat = "ai_society", q = "artificial intelligence society impact ethics bias fairness", limit = 5 },
     { beat = "ai_policy", q = "AI regulation policy EU AI Act governance accountability", limit = 5 },
     { beat = "ai_creative", q = "AI art creativity copyright generative music film", limit = 5 }
   ]
   ```

### How It Works

- **Keyword queries**: Each beat has a beat-specific search query optimized to stay within daily quota
- **Free tier quota**: 3 queries × 5 articles = 15 articles/day (well under 100 req/day limit)
- **Deduplication**: gnews.io results are merged with RSS and deduplicated using headline similarity
- **Prioritization**: gnews.io articles scored as "medium" priority (secondary to "high" RSS sources)

### Tier Upgrade at Launch

To upgrade to Essential tier at launch without code changes:

1. Change `tier = "essential"` in config
2. Update `api_key` with new Essential API key
3. Run scanner — no code changes needed

## Running the Scanner

### Local Development

```bash
# Build the scanner binary
cargo build --release --bin scanner

# Set Paperclip environment variables (auto-injected in Paperclip runs)
export PAPERCLIP_API_URL="http://127.0.0.1:3100"
export PAPERCLIP_API_KEY="<jwt-token>"
export PAPERCLIP_COMPANY_ID="<company-id>"
export PAPERCLIP_AGENT_ID="<agent-id>"
export PAPERCLIP_RUN_ID="<run-id>"

# Run the scanner heartbeat
./target/release/scanner
```

### In Paperclip

The Scanner runs automatically on a heartbeat schedule. Each run:
- Checks for assignments in the inbox
- Polls all configured RSS feeds and gnews.io (if API key set)
- Creates story candidate issues assigned to the Fact Checker

## Story Candidate Quality

A good candidate for story candidate creation has:

- ✅ **Real, verifiable news event** — Not opinion or speculation
- ✅ **Multiple source coverage** — Appears in 2+ feeds (higher priority)
- ✅ **Relevant to a beat** — Clearly Linux, Tech, Privacy, AI in Society, AI Policy, or AI Creative related
- ✅ **Potential Signal Noise value** — Transparency angle, AI lens, or genuine humor

Candidates are automatically rejected if:

- ❌ **Duplicate headlines** — Deduped at 0.85 similarity threshold
- ❌ **Pure press release** — No independent coverage
- ❌ **Too stale** — >48 hours old with no new developments (handled by feed source freshness)

## Scoring Algorithm

Each candidate is scored using three factors:

1. **Priority (40%)** — Source priority weight (high=3, medium=2, low=1)
2. **Recency (40%)** — Favor recent articles with exponential decay
3. **Summary depth (20%)** — More detailed summaries score higher

Top N candidates (max 10 per run) are surfaced as story candidate issues.

## Testing

Run unit tests:

```bash
cargo test scanner::tests --lib
```

Tests include:
- **gnews article conversion** — Verify gnews.io articles convert to StoryCandidate format
- **cross-source deduplication** — Verify RSS and gnews.io results deduplicate correctly

## Monitoring & Debugging

Enable debug logging:

```bash
RUST_LOG=debug ./target/release/scanner
```

Key log patterns to watch:
- `Fetching feed` — RSS fetch started
- `Fetched X articles from gnews.io` — gnews.io result count
- `After dedup:` — Deduped candidate count
- `Total candidates before dedup:` — Combined RSS + gnews count

## Paperclip Integration

The Scanner creates story candidate issues with:

- **Title**: `[BEAT] Headline` (e.g., `[LINUX] Kernel 6.8 Released`, `[AI_POLICY] EU Approves AI Act Amendments`)
- **Description**: Summary + metadata (sources, relevance score, publication date)
- **Status**: `todo`
- **Priority**: `high`, `medium`, or `low` (mapped from source priority)
- **Assigned to**: Fact Checker agent
- **Beat tag**: `linux`, `tech`, or `privacy`

## Architecture Notes

### Module Structure

- `src/scanner.rs` — Core polling, deduplication, ranking logic
- `src/gnews.rs` — gnews.io API client and response parsing
- `src/bin/scanner.rs` — Heartbeat entrypoint, Paperclip inbox check, issue creation

### Dependencies

- **feed-rs** — RSS/Atom parsing
- **reqwest** — HTTP client for gnews.io API
- **strsim** — Levenshtein distance for deduplication
- **toml** — TOML config parsing
- **chrono** — Datetime parsing and comparison
