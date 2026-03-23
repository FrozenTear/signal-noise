use anyhow::Result;
use chrono::Utc;
use feed_rs::parser;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedConfig {
    pub feed: Vec<FeedEntry>,
    pub scanner: ScannerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedEntry {
    pub url: String,
    pub name: String,
    pub beat: String,
    pub r#type: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerConfig {
    pub poll_interval_sec: u64,
    pub dedup_threshold: f64,
    pub max_candidates_per_run: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryCandidate {
    pub headline: String,
    pub summary: String,
    pub source_urls: Vec<String>,
    pub beat: String,
    pub priority: String,
    pub published_at: Option<chrono::DateTime<Utc>>,
    pub source_feed: String,
    pub relevance_score: f64,
}

/// Load feed configuration from TOML file
pub fn load_feed_config(path: &str) -> Result<FeedConfig> {
    let contents = fs::read_to_string(path)?;
    let config: FeedConfig = toml::from_str(&contents)?;
    Ok(config)
}

/// Fetch a single RSS feed
pub async fn fetch_feed(url: &str) -> Result<Vec<StoryCandidate>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "Signal Noise Scanner/1.0")
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await?;

    let body = response.bytes().await?;
    let feed = parser::parse(&body[..])?;

    let mut candidates = Vec::new();
    for entry in feed.entries {
        let headline = entry.title.clone().map(|t| t.content).unwrap_or_default();
        let summary = entry
            .summary
            .clone()
            .map(|s| s.content)
            .unwrap_or_else(|| {
                entry
                    .content
                    .as_ref()
                    .and_then(|c| c.body.clone())
                    .unwrap_or_default()
            });

        let source_urls: Vec<String> = entry
            .links
            .iter()
            .map(|link| link.href.clone())
            .collect();

        let published_at = entry.published.or_else(|| entry.updated);

        candidates.push(StoryCandidate {
            headline,
            summary,
            source_urls,
            beat: String::new(), // Will be filled by caller
            priority: String::new(),
            published_at,
            source_feed: String::new(), // Will be filled by caller
            relevance_score: 0.0,
        });
    }

    Ok(candidates)
}

/// Calculate string similarity using Levenshtein distance
fn string_similarity(s1: &str, s2: &str) -> f64 {
    let max_len = std::cmp::max(s1.len(), s2.len());
    if max_len == 0 {
        return 1.0;
    }

    let distance = strsim::levenshtein(s1, s2);
    1.0 - (distance as f64 / max_len as f64)
}

/// Deduplicate candidates using headline similarity
pub fn deduplicate_candidates(
    candidates: Vec<StoryCandidate>,
    threshold: f64,
) -> Vec<StoryCandidate> {
    let mut deduped = Vec::new();

    for candidate in candidates {
        let is_duplicate = deduped.iter().any(|existing: &StoryCandidate| {
            string_similarity(&candidate.headline, &existing.headline) > threshold
        });

        if !is_duplicate {
            deduped.push(candidate);
        }
    }

    deduped
}

/// Rank candidates by newsworthiness
pub fn rank_candidates(mut candidates: Vec<StoryCandidate>) -> Vec<StoryCandidate> {
    // Score based on:
    // - Priority (high=3, medium=2, low=1)
    // - Recency (favor recent articles)
    // - Summary length (more detailed is better)

    let now = Utc::now();

    for candidate in &mut candidates {
        let priority_score = match candidate.priority.as_str() {
            "high" => 3.0,
            "medium" => 2.0,
            _ => 1.0,
        };

        let recency_score = if let Some(pub_date) = candidate.published_at {
            let age_hours = (now - pub_date).num_hours() as f64;
            (1.0 / (1.0 + age_hours / 24.0)).max(0.1)
        } else {
            0.5
        };

        let summary_score = (candidate.summary.len() as f64 / 500.0).min(1.0);

        candidate.relevance_score = priority_score * 0.4 + recency_score * 0.4 + summary_score * 0.2;
    }

    candidates.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
    candidates
}

/// Poll all configured feeds and return top story candidates
pub async fn poll_feeds(
    config_path: &str,
) -> Result<Vec<StoryCandidate>> {
    let config = load_feed_config(config_path)?;

    let mut all_candidates = Vec::new();

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

    tracing::info!("Fetched {} total articles", all_candidates.len());

    // Deduplicate
    let deduped = deduplicate_candidates(all_candidates, config.scanner.dedup_threshold);
    tracing::info!("After dedup: {} articles", deduped.len());

    // Rank and limit
    let ranked = rank_candidates(deduped);
    let top = ranked
        .into_iter()
        .take(config.scanner.max_candidates_per_run)
        .collect();

    tracing::info!("Top {} candidates selected", config.scanner.max_candidates_per_run);

    Ok(top)
}
