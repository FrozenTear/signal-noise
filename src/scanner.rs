use anyhow::Result;
use chrono::Utc;
use feed_rs::parser;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

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

/// Convert a gnews.io article to a StoryCandidate
pub fn gnews_article_to_candidate(
    article: crate::gnews::GnewsArticle,
    beat: String,
    priority: String,
) -> StoryCandidate {
    let published_at = chrono::DateTime::parse_from_rfc3339(&article.published_at)
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
    let _config = load_feed_config(config_path)?;

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

/// Poll all configured feeds and return top story candidates
pub async fn poll_feeds(
    config_path: &str,
) -> Result<Vec<StoryCandidate>> {
    let config = load_feed_config(config_path)?;

    let mut all_candidates = Vec::new();

    // Fetch from RSS feeds
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

    // Fetch from gnews.io
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
            published_at: "2026-03-23T08:00:00Z".to_string(),
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
            headline: "Linux Kernel 6.8 Released with Major Security Updates".to_string(),
            summary: "Important security patches released".to_string(),
            source_urls: vec!["https://rss.example.com/1".to_string()],
            beat: "linux".to_string(),
            priority: "high".to_string(),
            published_at: None,
            source_feed: "Phoronix".to_string(),
            relevance_score: 0.0,
        };

        let gnews_candidate = StoryCandidate {
            headline: "Linux Kernel 6.8 Released with Major Security Updates".to_string(),
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

        // Should deduplicate these as identical headlines from different sources
        assert_eq!(deduped.len(), 1, "Identical headlines from different sources should be deduplicated");
    }
}
