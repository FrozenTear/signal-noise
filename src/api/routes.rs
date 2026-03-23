// Axum API route handlers — wired up by dioxus-axum in main.rs
//
// GET  /api/articles          — feed with optional ?category= filter
// GET  /api/articles/:slug    — single article with sources + pipeline
// POST /api/articles          — publish an article from the pipeline
// GET  /api/agents/status     — agent roster for live sidebar

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/articles", get(list_articles).post(publish_article))
        .route("/articles/{slug}", get(get_article))
        .route("/agents/status", get(agent_status))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct ArticleQuery {
    pub category: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

#[derive(Deserialize)]
pub struct ArticlePublishPayload {
    pub title: String,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub body: String,
    pub category: String,
    /// Persona slug (e.g. "priya-nair") — resolved to a record<persona> inside SurrealDB
    pub persona: Option<String>,
    pub confidence_score: Option<f64>,
    pub ai_monologue: Option<String>,
    pub sources: Option<Vec<SourcePayload>>,
    pub pipeline_steps: Option<Vec<PipelineStepPayload>>,
}

#[derive(Deserialize)]
pub struct SourcePayload {
    pub url: String,
    pub name: String,
    #[serde(rename = "type", default = "default_source_type")]
    pub source_type: String,
}

fn default_source_type() -> String {
    "wire".to_string()
}

#[derive(Deserialize)]
pub struct PipelineStepPayload {
    pub agent_name: String,
    pub step_type: String,
    pub input_summary: Option<String>,
    pub output_summary: Option<String>,
    pub confidence_delta: Option<f64>,
}

fn generate_slug(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub async fn list_articles(
    State(state): State<AppState>,
    Query(params): Query<ArticleQuery>,
) -> Result<Json<Value>, StatusCode> {
    let limit = params.limit.unwrap_or(20);
    let offset = params.offset.unwrap_or(0);

    let mut result = if let Some(cat) = params.category {
        state
            .db
            .query("SELECT * FROM article WHERE status = 'published' AND category = $cat ORDER BY published_at DESC LIMIT $limit START $offset")
            .bind(("cat", cat))
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        state
            .db
            .query("SELECT * FROM article WHERE status = 'published' ORDER BY published_at DESC LIMIT $limit START $offset")
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let articles: Vec<Value> = result.take(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(json!({ "articles": articles })))
}

pub async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let mut result = state
        .db
        .query(
            r#"
            SELECT
                *,
                ->cites->source.* AS sources,
                ->produced_by->pipeline_step.* AS pipeline
            FROM article
            WHERE slug = $slug
            LIMIT 1
            "#,
        )
        .bind(("slug", slug))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let articles: Vec<Value> = result.take(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match articles.into_iter().next() {
        Some(article) => Ok(Json(article)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn publish_article(
    State(state): State<AppState>,
    Json(payload): Json<ArticlePublishPayload>,
) -> Result<Json<Value>, StatusCode> {
    // Validate required fields
    if payload.title.trim().is_empty()
        || payload.body.trim().is_empty()
        || payload.category.trim().is_empty()
    {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }

    let slug = payload
        .slug
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| generate_slug(&payload.title));

    let summary = payload.summary.unwrap_or_default();
    let confidence = payload.confidence_score.unwrap_or(0.0);
    let ai_monologue = payload.ai_monologue.unwrap_or_default();
    let persona = payload.persona.unwrap_or_default();

    // Upsert article.
    // persona is resolved to a record<persona> inside SurrealDB via sub-select.
    // UPSERT MERGE preserves existing fields (e.g. created_at) on re-publish;
    // schema DEFAULTs fill pipeline_metadata, source_urls, created_at on first insert.
    state
        .db
        .query(
            r#"
            UPSERT article MERGE {
                slug:             $slug,
                title:            $title,
                summary:          $summary,
                body:             $body,
                category:         $category,
                persona:          IF $persona != '' THEN
                                      (SELECT id FROM persona WHERE slug = $persona LIMIT 1)[0].id
                                  ELSE NONE END,
                confidence_score: $confidence,
                ai_monologue:     $monologue,
                status:           'published',
                published_at:     time::now(),
                updated_at:       time::now()
            } WHERE slug = $slug
            "#,
        )
        .bind(("slug", slug.clone()))
        .bind(("title", payload.title))
        .bind(("summary", summary))
        .bind(("body", payload.body))
        .bind(("category", payload.category))
        .bind(("persona", persona))
        .bind(("confidence", confidence))
        .bind(("monologue", ai_monologue))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Upsert each source and create article->cites->source edges.
    // All ID resolution happens inside SurrealDB — no Value passing across the boundary.
    if let Some(sources) = payload.sources {
        for source in sources {
            state
                .db
                .query(
                    r#"
                    LET $art = (SELECT id FROM article WHERE slug = $slug LIMIT 1)[0].id;
                    LET $src = (INSERT INTO source {
                        url:                 $url,
                        name:                $name,
                        type:                $stype,
                        paywall_status:      'unknown',
                        verification_status: 'unknown'
                    } ON DUPLICATE KEY UPDATE name = $input.name)[0];
                    RELATE $art->cites->$src.id;
                    "#,
                )
                .bind(("slug", slug.clone()))
                .bind(("url", source.url))
                .bind(("name", source.name))
                .bind(("stype", source.source_type))
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    // Create pipeline_step records and article->produced_by->pipeline_step edges.
    if let Some(steps) = payload.pipeline_steps {
        for step in steps {
            let input_summary = step.input_summary.unwrap_or_default();
            let output_summary = step.output_summary.unwrap_or_default();
            let confidence_delta = step.confidence_delta.unwrap_or(0.0);

            state
                .db
                .query(
                    r#"
                    LET $art = (SELECT id FROM article WHERE slug = $slug LIMIT 1)[0].id;
                    LET $step = (CREATE pipeline_step CONTENT {
                        article:          $art,
                        agent_name:       $agent_name,
                        step_type:        $step_type,
                        input_summary:    $input_summary,
                        output_summary:   $output_summary,
                        confidence_delta: $confidence_delta,
                        started_at:       time::now()
                    })[0];
                    RELATE $art->produced_by->$step.id;
                    "#,
                )
                .bind(("slug", slug.clone()))
                .bind(("agent_name", step.agent_name))
                .bind(("step_type", step.step_type))
                .bind(("input_summary", input_summary))
                .bind(("output_summary", output_summary))
                .bind(("confidence_delta", confidence_delta))
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    Ok(Json(json!({ "status": "published", "slug": slug })))
}

pub async fn agent_status(
    State(_state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "agents": [
            {
                "id": "scanner",
                "name": "Scanner",
                "role": "Finds and ingests news from RSS feeds and gnews.io",
                "status": "active",
                "beat": "all"
            },
            {
                "id": "founding-engineer",
                "name": "Founding Engineer",
                "role": "Backend infrastructure, database, and pipeline plumbing",
                "status": "active",
                "beat": "engineering"
            }
        ]
    })))
}
