use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<RecordId>,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub category: String,
    /// Region facet (THE-246) — orthogonal to category. Defaults to Global.
    #[serde(default)]
    pub region: Region,
    pub persona: Option<RecordId>,
    pub confidence_score: f64,
    pub ai_monologue: Option<String>,
    pub pipeline_metadata: serde_json::Value,
    pub source_urls: Vec<String>,
    pub status: ArticleStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Region facet (THE-246). Orthogonal to category; default Global.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    American,
    European,
    #[default]
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ArticleStatus {
    Draft,
    FactChecking,
    Writing,
    Editing,
    Published,
    Rejected,
}
