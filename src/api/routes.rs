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
        .route("/agents/status", get(agent_status).put(push_agent_status))
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
    pub ai_monologue_extended: Option<String>,
    pub sources: Option<Vec<SourcePayload>>,
    pub pipeline_steps: Option<Vec<PipelineStepPayload>>,
}

#[derive(Deserialize)]
pub struct SourcePayload {
    pub url: String,
    pub name: String,
    #[serde(rename = "type", default = "default_source_type")]
    pub source_type: String,
    pub paywall_status: Option<String>,
    pub verification_status: Option<String>,
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
            .query("SELECT *, persona.name AS persona_name FROM article WHERE status = 'published' AND category = $cat ORDER BY published_at DESC LIMIT $limit START $offset")
            .bind(("cat", cat))
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        state
            .db
            .query("SELECT *, persona.name AS persona_name FROM article WHERE status = 'published' ORDER BY published_at DESC LIMIT $limit START $offset")
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
                persona.name AS persona_name,
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
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let bad_req = |msg: &str| -> (StatusCode, Json<Value>) {
        (StatusCode::BAD_REQUEST, Json(json!({ "error": msg })))
    };
    let db_err = || -> (StatusCode, Json<Value>) {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "database error" })))
    };

    // Required text fields
    if payload.title.trim().is_empty() {
        return Err(bad_req("title is required"));
    }
    if payload.body.trim().is_empty() {
        return Err(bad_req("body is required"));
    }
    if payload.category.trim().is_empty() {
        return Err(bad_req("category is required"));
    }

    // ai_monologue_extended: must be provided and non-empty
    let ai_monologue_extended = match &payload.ai_monologue_extended {
        None => return Err(bad_req("ai_monologue_extended is required")),
        Some(s) if s.trim().is_empty() => return Err(bad_req("ai_monologue_extended cannot be empty")),
        Some(s) => s.clone(),
    };

    // confidence_score: if provided, must be within [0.0, 1.0]
    if let Some(score) = payload.confidence_score {
        if !(0.0_f64..=1.0_f64).contains(&score) {
            return Err(bad_req("confidence_score must be between 0.0 and 1.0"));
        }
    }
    let confidence = payload.confidence_score.unwrap_or(0.0);

    // persona: must be provided and non-empty
    let persona = match &payload.persona {
        None => return Err(bad_req("persona is required")),
        Some(s) if s.trim().is_empty() => return Err(bad_req("persona cannot be empty")),
        Some(s) => s.clone(),
    };

    // body: reject if it contains metadata headers (YAML frontmatter or ALL_CAPS: lines)
    let has_metadata = payload.body.lines().any(|line| {
        let t = line.trim();
        if t.starts_with("---") {
            return true;
        }
        if let Some(colon) = t.find(':') {
            let key = &t[..colon];
            return key.len() >= 2 && key.chars().all(|c| c.is_ascii_uppercase() || c == '_');
        }
        false
    });
    if has_metadata {
        return Err(bad_req("body contains metadata headers; strip frontmatter before publishing"));
    }

    // Validate persona slug exists in DB
    let mut persona_res = state
        .db
        .query("SELECT id FROM persona WHERE slug = $slug LIMIT 1")
        .bind(("slug", persona.clone()))
        .await
        .map_err(|_| db_err())?;
    let persona_rows: Vec<Value> = persona_res.take(0).map_err(|_| db_err())?;
    if persona_rows.is_empty() {
        return Err(bad_req(&format!("persona '{}' does not exist", persona)));
    }

    // Validate category slug exists in DB
    let category = payload.category.trim().to_string();
    let mut cat_res = state
        .db
        .query("SELECT id FROM category WHERE slug = $slug LIMIT 1")
        .bind(("slug", category.clone()))
        .await
        .map_err(|_| db_err())?;
    let cat_rows: Vec<Value> = cat_res.take(0).map_err(|_| db_err())?;
    if cat_rows.is_empty() {
        return Err(bad_req(&format!("category '{}' does not exist", category)));
    }

    let slug = payload
        .slug
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| generate_slug(&payload.title));

    let summary = payload.summary.unwrap_or_default();
    let ai_monologue = payload.ai_monologue.unwrap_or_default();

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
                confidence_score:    $confidence,
                ai_monologue:        $monologue,
                ai_monologue_extended: IF $monologue_extended != '' THEN $monologue_extended ELSE NONE END,
                status:              'published',
                published_at:     IF (SELECT published_at FROM article WHERE slug = $slug LIMIT 1)[0].published_at != NONE
                                  THEN (SELECT published_at FROM article WHERE slug = $slug LIMIT 1)[0].published_at
                                  ELSE time::now() END,
                updated_at:       time::now()
            } WHERE slug = $slug
            "#,
        )
        .bind(("slug", slug.clone()))
        .bind(("title", payload.title))
        .bind(("summary", summary))
        .bind(("body", payload.body))
        .bind(("category", category))
        .bind(("persona", persona))
        .bind(("confidence", confidence))
        .bind(("monologue", ai_monologue))
        .bind(("monologue_extended", ai_monologue_extended))
        .await
        .map_err(|_| db_err())?;

    // On re-publish, clean up old cites edges and pipeline_step records so we don't duplicate.
    state
        .db
        .query(
            r#"
            LET $art = (SELECT id FROM article WHERE slug = $slug LIMIT 1)[0].id;
            DELETE cites WHERE in = $art;
            LET $old_steps = (SELECT ->produced_by->pipeline_step AS steps FROM $art)[0].steps;
            DELETE produced_by WHERE in = $art;
            FOR $s IN $old_steps { DELETE $s; };
            "#,
        )
        .bind(("slug", slug.clone()))
        .await
        .map_err(|_| db_err())?;

    // Upsert each source and create article->cites->source edges.
    // All ID resolution happens inside SurrealDB — no Value passing across the boundary.
    if let Some(sources) = payload.sources {
        for source in sources {
            let paywall = source.paywall_status.unwrap_or_else(|| "unknown".to_string());
            let verification = source.verification_status.unwrap_or_else(|| "unknown".to_string());

            state
                .db
                .query(
                    r#"
                    LET $art = (SELECT id FROM article WHERE slug = $slug LIMIT 1)[0].id;
                    LET $src = (INSERT INTO source {
                        url:                 $url,
                        name:                $name,
                        type:                $stype,
                        paywall_status:      $paywall,
                        verification_status: $verification
                    } ON DUPLICATE KEY UPDATE
                        name = $input.name,
                        paywall_status = IF $input.paywall_status != 'unknown' THEN $input.paywall_status ELSE paywall_status END,
                        verification_status = IF $input.verification_status != 'unknown' THEN $input.verification_status ELSE verification_status END
                    )[0];
                    RELATE $art->cites->$src.id;
                    "#,
                )
                .bind(("slug", slug.clone()))
                .bind(("url", source.url))
                .bind(("name", source.name))
                .bind(("stype", source.source_type))
                .bind(("paywall", paywall))
                .bind(("verification", verification))
                .await
                .map_err(|_| db_err())?;
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
                .map_err(|_| db_err())?;
        }
    }

    Ok(Json(json!({ "status": "published", "slug": slug })))
}

/// GET /api/agents/status — read agent status from DB (pushed by Paperclip heartbeats).
pub async fn agent_status(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let mut result = state
        .db
        .query("SELECT agent_id, name, status, current_task, updated_at FROM agent_status ORDER BY name ASC")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let agents: Vec<Value> = result.take(0).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(json!({ "agents": agents })))
}

#[derive(Deserialize)]
pub struct AgentStatusPushItem {
    pub agent_id: String,
    pub name: String,
    pub model: Option<String>,
    pub status: String,
    pub current_task: Option<String>,
}

/// PUT /api/agents/status — upsert agent statuses (called by Paperclip heartbeats).
/// Accepts a JSON array of agent status items. Localhost-only by convention.
pub async fn push_agent_status(
    State(state): State<AppState>,
    Json(items): Json<Vec<AgentStatusPushItem>>,
) -> Result<Json<Value>, StatusCode> {
    for item in items {
        let task = item.current_task.unwrap_or_default();
        let model = item.model.unwrap_or_default();
        state
            .db
            .query(
                r#"
                UPSERT agent_status MERGE {
                    agent_id:     $agent_id,
                    name:         $name,
                    model:        IF $model != '' THEN $model ELSE NONE END,
                    status:       $status,
                    current_task: IF $task != '' THEN $task ELSE NONE END,
                    updated_at:   time::now()
                } WHERE agent_id = $agent_id
                "#,
            )
            .bind(("agent_id", item.agent_id))
            .bind(("name", item.name))
            .bind(("model", model))
            .bind(("status", item.status))
            .bind(("task", task))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(json!({ "status": "ok" })))
}
