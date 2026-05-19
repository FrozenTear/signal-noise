# gnews.io Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Integrate gnews.io API as a secondary discovery channel alongside RSS feeds to expand story candidate sourcing.

**Architecture:** Create a new `gnews` module that fetches articles from gnews.io free tier using beat-specific keyword queries. Merge gnews results with existing RSS candidates, deduplicate by headline similarity, and pass the combined list through the existing ranking and Paperclip issue creation pipeline. gnews.io tier and API key are configurable in `config/feeds.toml` to support free-to-Essential tier upgrade at launch without code changes.

**Tech Stack:** gnews.io v4 REST API, reqwest HTTP client, existing feed_rs + strsim deduplication infrastructure.

---

## File Structure

| File | Responsibility |
|------|-----------------|
| `src/gnews.rs` | gnews.io API client, query builder, response parsing |
| `src/scanner.rs` | Orchestrate RSS + gnews.io fetching, merge + deduplicate, ranking (extends existing) |
| `src/lib.rs` | Export gnews module (one-line add) |
| `config/feeds.toml` | Add [gnews] config section with API key, tier, beat queries |
| `src/bin/scanner.rs` | No changes needed (calls existing poll_feeds entrypoint) |

---

## Task 1: Add gnews Configuration to feeds.toml

**Files:**
- Modify: `config/feeds.toml` (append at end)

- [ ] **Step 1: Understand current config structure**

Read `config/feeds.toml` (already done) to see feed sections and scanner config.

- [ ] **Step 2: Add [gnews] section with API key, tier, beat-specific queries**

Append this to `config/feeds.toml`:

```toml
# ─── gnews.io Discovery Channel ───────────────────────────────────────────────

[gnews]
# API key: get from gnews.io account (start with free tier)
api_key = "GNEWS_API_KEY_HERE"
# Tier: "free" (dev) or "essential" (€50/mo, launch)
tier = "free"
# Beat-specific queries for article discovery
# Use narrow, on-topic keywords to stay within daily quota (100 req/day, 10 articles/req max)
queries = [
  { beat = "linux", q = "Linux kernel distribution open source", limit = 5 },
  { beat = "tech", q = "tech startup AI machine learning software engineering", limit = 5 },
  { beat = "privacy", q = "privacy surveillance data protection encryption", limit = 5 }
]
```

- [ ] **Step 3: Commit**

```bash
cd /home/pure/signal-noise
git add config/feeds.toml
git commit -m "config: add gnews.io integration section to feeds.toml"
```

---

## Task 2: Create gnews Module with API Client

**Files:**
- Create: `src/gnews.rs`
- Modify: `src/lib.rs` (one line to export module)

- [ ] **Step 1: Write gnews structs and API response parsing**

Create `src/gnews.rs`:

```rust
use anyhow::Result;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

/// gnews.io API response for article search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnewsSearchResponse {
    pub totalArticles: u32,
    pub articles: Vec<GnewsArticle>,
}

/// Single article from gnews.io
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnewsArticle {
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub image: Option<String>,
    pub url: String,
    pub source: GnewsSource,
    pub publishedAt: String, // ISO 8601 format
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnewsSource {
    pub name: String,
    pub url: String,
}

/// gnews.io Configuration from feeds.toml [gnews] section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnewsConfig {
    pub api_key: String,
    pub tier: String, // "free" or "essential"
    pub queries: Vec<GnewsQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnewsQuery {
    pub beat: String,
    pub q: String,
    pub limit: usize,
}

/// Fetch articles from gnews.io for a single query
pub async fn fetch_gnews_query(config: &GnewsConfig, query: &GnewsQuery) -> Result<Vec<GnewsArticle>> {
    let client = reqwest::Client::new();
    let url = "https://gnewsapi.com/api/v4/search";

    // Construct query parameters
    let lang = "en";
    let sort_by = "publishedAt"; // Most recent first

    tracing::debug!(
        "Fetching gnews.io: beat={}, q='{}', limit={}",
        query.beat,
        query.q,
        query.limit
    );

    let response = client
        .get(url)
        .query(&[
            ("q", query.q.as_str()),
            ("lang", lang),
            ("sortby", sort_by),
            ("max", &query.limit.to_string()),
            ("apikey", &config.api_key),
        ])
        .header("User-Agent", "Signal Noise Scanner/1.0")
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!(
            "gnews.io API error ({}): {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }

    let body: GnewsSearchResponse = response.json().await?;
    tracing::info!(
        "Fetched {} articles from gnews.io for beat '{}' query '{}'",
        body.articles.len(),
        query.beat,
        query.q
    );

    Ok(body.articles)
}

/// Fetch all gnews articles for configured queries
pub async fn fetch_all_gnews(config: &GnewsConfig) -> Result<Vec<(String, GnewsArticle)>> {
    let mut all_articles = Vec::new();

    for query in &config.queries {
        match fetch_gnews_query(config, query).await {
            Ok(articles) => {
                for article in articles {
                    all_articles.push((query.beat.clone(), article));
                }
            }
            Err(e) => {
                tracing::warn!("Failed to fetch gnews.io for beat '{}': {}", query.beat, e);
            }
        }
    }

    Ok(all_articles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnews_config_deserialize() {
        let toml_str = r#"
api_key = "test-key"
tier = "free"
queries = [
    { beat = "linux", q = "kernel", limit = 5 }
]
"#;
        let config: GnewsConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.tier, "free");
        assert_eq!(config.queries.len(), 1);
        assert_eq!(config.queries[0].beat, "linux");
    }
}
```

- [ ] **Step 2: Export gnews module from lib.rs**

Edit `src/lib.rs`:

```rust
pub mod models;
pub mod scanner;
pub mod gnews;
```

- [ ] **Step 3: Verify the module compiles**

```bash
cd /home/pure/signal-noise
cargo check
```

Expected: No errors. Module compiles with new gnews structures.

- [ ] **Step 4: Commit**

```bash
git add src/gnews.rs src/lib.rs
git commit -m "feat: add gnews.io API client module with response parsing"
```

---

## Task 3: Extend scanner.rs to Fetch from gnews.io

**Files:**
- Modify: `src/scanner.rs` (add new function, update poll_feeds orchestration)

- [ ] **Step 1: Add function to convert gnews articles to StoryCandidate**

Add this function to `src/scanner.rs`:

```rust
/// Convert a gnews.io article to a StoryCandidate
pub fn gnews_article_to_candidate(
    article: crate::gnews::GnewsArticle,
    beat: String,
    priority: String,
) -> StoryCandidate {
    let published_at = chrono::DateTime::parse_from_rfc3339(&article.publishedAt)
        .ok()
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let summary = article
        .description
        .unwrap_or_else(|| article.content.unwrap_or_default());

    StoryCandidate {
        headline: article.title,
        summary,
        source_urls: vec![article.url],
        beat,
        priority,
        published_at,
        source_feed: format!("gnews.io ({})", article.source.name),
        relevance_score: 0.0, // Will be set by rank_candidates
    }
}

/// Fetch gnews articles and convert to story candidates
pub async fn fetch_gnews_candidates(config_path: &str) -> Result<Vec<StoryCandidate>> {
    let config = load_feed_config(config_path)?;

    // Extract gnews config from the TOML (need to parse manually)
    let contents = std::fs::read_to_string(config_path)?;
    let root: toml::Table = toml::from_str(&contents)?;

    let gnews_table = match root.get("gnews") {
        Some(toml::Value::Table(t)) => t.clone(),
        _ => {
            tracing::warn!("No [gnews] config found in {}", config_path);
            return Ok(Vec::new());
        }
    };

    let gnews_config: crate::gnews::GnewsConfig = gnews_table.try_into()?;

    let mut candidates = Vec::new();
    let gnews_articles = crate::gnews::fetch_all_gnews(&gnews_config).await?;

    for (beat, article) in gnews_articles {
        let priority = "medium".to_string(); // gnews discovery is secondary tier
        let candidate = gnews_article_to_candidate(article, beat, priority);
        candidates.push(candidate);
    }

    tracing::info!(
        "Fetched {} story candidates from gnews.io",
        candidates.len()
    );

    Ok(candidates)
}
```

- [ ] **Step 2: Update poll_feeds to merge RSS + gnews candidates**

Locate the `poll_feeds` function in `src/scanner.rs` and update it:

Replace the existing implementation (keeping the function signature) with:

```rust
/// Poll all configured feeds (RSS + gnews.io) and return top story candidates
pub async fn poll_feeds(
    config_path: &str,
) -> Result<Vec<StoryCandidate>> {
    let config = load_feed_config(config_path)?;

    let mut all_candidates = Vec::new();

    // Fetch from RSS feeds (existing code)
    for feed_entry in config.feed {
        tracing::info!("Fetching feed: {} ({})", feed_entry.name, feed_entry.url);

        match fetch_feed(&feed_entry.url).await {
            Ok(mut candidates) => {
                for candidate in &mut candidates {
                    candidate.beat = feed_entry.beat.clone();
                    candidate.priority = feed_entry.priority.clone();
                    candidate.source_feed = feed_entry.name.clone();
                }
                all_candidates.extend(candidates);
            }
            Err(e) => {
                tracing::warn!("Failed to fetch {}: {}", feed_entry.name, e);
            }
        }
    }

    tracing::info!("Fetched {} total articles from RSS", all_candidates.len());

    // Fetch from gnews.io (new code)
    match fetch_gnews_candidates(config_path).await {
        Ok(gnews_candidates) => {
            tracing::info!("Fetched {} articles from gnews.io", gnews_candidates.len());
            all_candidates.extend(gnews_candidates);
        }
        Err(e) => {
            tracing::warn!("Failed to fetch gnews.io candidates: {}", e);
        }
    }

    tracing::info!("Total candidates before dedup: {}", all_candidates.len());

    // Deduplicate (existing code)
    let deduped = deduplicate_candidates(all_candidates, config.scanner.dedup_threshold);
    tracing::info!("After dedup: {} articles", deduped.len());

    // Rank and limit (existing code)
    let ranked = rank_candidates(deduped);
    let top = ranked
        .into_iter()
        .take(config.scanner.max_candidates_per_run)
        .collect();

    tracing::info!("Top {} candidates selected", config.scanner.max_candidates_per_run);

    Ok(top)
}
```

- [ ] **Step 3: Verify scanner.rs compiles**

```bash
cd /home/pure/signal-noise
cargo check
```

Expected: No errors. gnews integration compiles.

- [ ] **Step 4: Test the type conversion manually**

Run a quick compile check on just the scanner module:

```bash
cargo check --bin scanner
```

Expected: Compiles successfully.

- [ ] **Step 5: Commit**

```bash
git add src/scanner.rs
git commit -m "feat: extend scanner to fetch and merge gnews.io results with RSS"
```

---

## Task 4: Update scanner.rs Imports and Dependencies

**Files:**
- Modify: `src/scanner.rs` (imports section)

- [ ] **Step 1: Add required imports at top of scanner.rs**

Add these imports to the top of `src/scanner.rs` (after existing imports):

```rust
use toml;
```

Verify this is added and matches the code in Task 3.

- [ ] **Step 2: Verify full build**

```bash
cd /home/pure/signal-noise
cargo build --bin scanner
```

Expected: Build succeeds with no warnings or errors.

- [ ] **Step 3: Commit**

```bash
git add src/scanner.rs
git commit -m "fix: add toml import for gnews config parsing"
```

---

## Task 5: Create Integration Tests

**Files:**
- Create: `src/scanner/tests.rs` (or create tests module if doesn't exist)

- [ ] **Step 1: Write test for gnews article conversion**

Add or create test file with:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnews_article_to_candidate() {
        let article = crate::gnews::GnewsArticle {
            title: "Linux Kernel 6.8 Released".to_string(),
            description: Some("New features and improvements".to_string()),
            content: None,
            image: None,
            url: "https://example.com/article".to_string(),
            source: crate::gnews::GnewsSource {
                name: "TechNews".to_string(),
                url: "https://example.com".to_string(),
            },
            publishedAt: "2026-03-23T08:00:00Z".to_string(),
        };

        let candidate = gnews_article_to_candidate(
            article,
            "linux".to_string(),
            "medium".to_string(),
        );

        assert_eq!(candidate.headline, "Linux Kernel 6.8 Released");
        assert_eq!(candidate.beat, "linux");
        assert_eq!(candidate.priority, "medium");
        assert!(candidate.source_feed.contains("gnews.io"));
        assert_eq!(candidate.source_urls.len(), 1);
    }

    #[test]
    fn test_deduplication_rss_vs_gnews() {
        let rss_candidate = StoryCandidate {
            headline: "Breaking: Major Linux Update".to_string(),
            summary: "Important security patches released".to_string(),
            source_urls: vec!["https://rss.example.com/1".to_string()],
            beat: "linux".to_string(),
            priority: "high".to_string(),
            published_at: None,
            source_feed: "Phoronix".to_string(),
            relevance_score: 0.0,
        };

        let gnews_candidate = StoryCandidate {
            headline: "Breaking: Major Linux Update Released".to_string(),
            summary: "Important patches from kernel team".to_string(),
            source_urls: vec!["https://gnews.example.com/1".to_string()],
            beat: "linux".to_string(),
            priority: "medium".to_string(),
            published_at: None,
            source_feed: "gnews.io (TechNews)".to_string(),
            relevance_score: 0.0,
        };

        let candidates = vec![rss_candidate, gnews_candidate];
        let deduped = deduplicate_candidates(candidates, 0.85);

        // Should deduplicate these as the same story despite different sources
        assert_eq!(deduped.len(), 1, "Similar headlines should be deduplicated");
    }
}
```

- [ ] **Step 2: Run tests to verify they pass**

```bash
cd /home/pure/signal-noise
cargo test scanner::tests
```

Expected: Both tests pass.

- [ ] **Step 3: Commit**

```bash
git add src/scanner.rs
git commit -m "test: add gnews article conversion and cross-source deduplication tests"
```

---

## Task 6: Validation & Documentation

**Files:**
- Verify: Everything compiles and runs
- Document: Config requirements

- [ ] **Step 1: Full build and test**

```bash
cd /home/pure/signal-noise
cargo build --release --bin scanner
cargo test
```

Expected: All tests pass, binary builds successfully.

- [ ] **Step 2: Verify scanner.rs exports are accessible**

In `src/bin/scanner.rs`, verify it can call the updated poll_feeds:

```bash
grep -n "poll_feeds" src/bin/scanner.rs
```

Expected: Finds the `poll_feeds(config_path).await?` call. No changes needed.

- [ ] **Step 3: Document gnews.io API key setup requirement**

Create or append to `docs/SCANNER.md`:

```markdown
## gnews.io Integration

The Scanner now supports gnews.io as a secondary discovery channel alongside RSS feeds.

### Configuration

1. Get a free API key from [gnews.io](https://gnews.io)
   - Free tier: 100 requests/day, 10 articles per request
   - Essential tier: €50/month (upgrade at launch time)

2. Update `config/feeds.toml` [gnews] section:
   ```toml
   [gnews]
   api_key = "YOUR_API_KEY_HERE"
   tier = "free"  # or "essential" for production
   queries = [
     { beat = "linux", q = "Linux kernel distribution open source", limit = 5 },
     { beat = "tech", q = "tech startup AI machine learning software engineering", limit = 5 },
     { beat = "privacy", q = "privacy surveillance data protection encryption", limit = 5 }
   ]
   ```

3. Run the scanner as usual — it automatically fetches from both RSS and gnews.io:
   ```bash
   ./target/release/scanner
   ```

### Implementation Details

- **Deduplication:** gnews.io results are merged with RSS and deduplicated using headline similarity (0.85 threshold)
- **Prioritization:** gnews.io articles are scored as "medium" priority (secondary to RSS "high" sources)
- **Quotas:** Free tier limited to 100 req/day across all beat queries. The default 3 queries × 5 articles = 15 articles/day is well within quota.

### Upgrade Path

When ready to launch with Essential tier:
1. Change `tier = "essential"` in config
2. Update `api_key` with Essential tier key
3. No code changes needed
```

- [ ] **Step 4: Commit documentation**

```bash
git add docs/SCANNER.md
git commit -m "docs: add gnews.io integration setup and configuration guide"
```

---

## Final Checklist

- [ ] All tasks completed
- [ ] Scanner binary compiles without warnings
- [ ] Tests pass (`cargo test`)
- [ ] `config/feeds.toml` has [gnews] section with placeholder API key
- [ ] `src/gnews.rs` created with API client
- [ ] `src/scanner.rs` extended with gnews integration
- [ ] `src/lib.rs` exports gnews module
- [ ] Documentation updated
- [ ] All commits created with clear messages

---

## Execution Notes

**Important:** The gnews.io API key in `config/feeds.toml` must be obtained from https://gnews.io before running the scanner. The placeholder `GNEWS_API_KEY_HERE` will cause API errors.

To test locally:
1. Sign up for gnews.io free tier
2. Copy API key into config
3. Run `cargo build --release --bin scanner` and `./target/release/scanner` in a Paperclip-aware environment

**Rollback:** All changes are isolated to the new gnews module and scanner extensions. If issues arise, the RSS polling path remains unchanged.
