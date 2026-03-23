// Axum API route handlers — wired up by dioxus-axum in main.rs
//
// GET  /api/articles          — feed with optional ?category= filter
// GET  /api/articles/:slug    — single article with sources + pipeline
// POST /api/articles          — webhook receiver for pipeline publish events
// GET  /api/agents/status     — agent roster for live sidebar
// WS   /api/ws/agents         — live agent status (SurrealDB LIVE SELECT)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::AppState;

#[derive(Deserialize)]
pub struct ArticleQuery {
    pub category: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn list_articles(
    State(state): State<AppState>,
    Query(params): Query<ArticleQuery>,
) -> Result<Json<Value>, StatusCode> {
    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);

    let query = if let Some(cat) = params.category {
        format!(
            "SELECT * FROM article WHERE status = 'published' AND category = '{}' ORDER BY published_at DESC LIMIT {} START {}",
            cat, limit, offset
        )
    } else {
        format!(
            "SELECT * FROM article WHERE status = 'published' ORDER BY published_at DESC LIMIT {} START {}",
            limit, offset
        )
    };

    let mut result = state.db.query(query).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let articles: Vec<Value> = result.take(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({ "articles": articles })))
}

pub async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let query = format!(
        r#"
        SELECT
            *,
            ->cites->source.* AS sources,
            ->produced_by->pipeline_step.* AS pipeline
        FROM article
        WHERE slug = '{}'
        LIMIT 1
        "#,
        slug
    );

    let mut result = state.db.query(query).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let articles: Vec<Value> = result.take(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match articles.into_iter().next() {
        Some(article) => Ok(Json(article)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn publish_article(
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let _ = state;
    let _ = payload;
    // TODO: validate, upsert article from pipeline publish event
    Ok(Json(json!({ "status": "accepted" })))
}

pub async fn agent_status(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let _ = state;
    // TODO: query Paperclip API or local state for agent roster
    Ok(Json(json!({ "agents": [] })))
}
