use anyhow::Result;
use serde::{Deserialize, Serialize};

/// gnews.io API response for article search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GnewsSearchResponse {
    #[serde(rename = "totalArticles")]
    pub total_articles: u32,
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
    #[serde(rename = "publishedAt")]
    pub published_at: String, // ISO 8601 format
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
        body.total_articles,
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
