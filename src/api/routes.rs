// Axum API route handlers — wired up by dioxus-axum in main.rs
//
// GET    /api/articles          — feed with optional ?category= filter
// GET    /api/articles/:slug    — single article with sources + pipeline
// POST   /api/articles          — publish an article from the pipeline
// PATCH  /api/articles/:slug    — update article status
// GET    /api/agents/status     — agent roster for live sidebar

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, patch},
    Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::info;

use super::auth::BearerAuth;
use super::rate_limit::{rate_limit_middleware, RateLimitState};
use super::AppState;

pub fn router(state: AppState) -> Router {
    let rl = RateLimitState::new();
    Router::new()
        .route("/articles", get(list_articles).post(publish_article))
        .route("/articles/{slug}", get(get_article).patch(update_article_status))
        .route("/agents/status", get(agent_status).put(push_agent_status))
        .layer(middleware::from_fn_with_state(rl, rate_limit_middleware))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct ArticleQuery {
    pub category: Option<String>,
    /// THE-246: optional region facet filter (american|european|global).
    pub region: Option<String>,
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
    /// THE-246: region facet (american|european|global). Optional — defaults to "global".
    pub region: Option<String>,
    /// Persona slug (e.g. "priya-nair") — resolved to a record<persona> inside SurrealDB.
    /// Optional: when omitted or empty, use `byline` instead.
    pub persona: Option<String>,
    /// Free-form byline string used when persona is NULL (e.g. H2H AI-reporter pairings).
    pub byline: Option<String>,
    /// H2H linkage and model attribution stored in pipeline_metadata.
    pub pipeline_metadata: Option<Value>,
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
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

fn step_sort_order(step_type: &str) -> i32 {
    match step_type {
        "scan" => 0,
        "source_check" => 1,
        "fact_check" => 2,
        "draft" => 3,
        "verify" => 4,
        "edit" => 5,
        _ => 99,
    }
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

    // THE-246: category and region are independent, both optional filters.
    // Build the WHERE clause from whichever filters were supplied.
    let mut conditions = vec!["status = 'published'".to_string()];
    if params.category.is_some() {
        conditions.push("category = $cat".to_string());
    }
    if params.region.is_some() {
        conditions.push("region = $region".to_string());
    }
    let sql = format!(
        "SELECT *, persona.name AS persona_name FROM article WHERE {} ORDER BY published_at DESC LIMIT $limit START $offset",
        conditions.join(" AND ")
    );

    let mut query = state
        .db
        .query(sql)
        .bind(("limit", limit))
        .bind(("offset", offset));
    if let Some(cat) = params.category {
        query = query.bind(("cat", cat));
    }
    if let Some(region) = params.region {
        query = query.bind(("region", region));
    }
    let mut result = query.await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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
    _auth: BearerAuth,
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

    // persona: optional — only validate existence when a non-empty slug is supplied
    let persona = payload.persona.as_deref().unwrap_or("").trim().to_string();
    let byline = payload.byline.as_deref().unwrap_or("").trim().to_string();

    // body: reject if it still contains the metadata sections that should be
    // extracted into dedicated API fields (monologue, sources, pipeline, etc.)
    let metadata_headings = [
        "## AI Monologue",
        "## Confidence Score",
        "## Source Block",
        "## Pipeline Metadata",
        "## Extended Process Log",
    ];
    let has_metadata = payload.body.lines().any(|line| {
        let t = line.trim();
        metadata_headings.iter().any(|h| t == *h)
    });
    if has_metadata {
        return Err(bad_req("body contains metadata sections (## AI Monologue, ## Source Block, etc.); extract these into dedicated API fields"));
    }

    // Validate persona slug when a non-empty slug was supplied
    if !persona.is_empty() {
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

    // THE-246: region facet. Absent/empty defaults to "global" (backward compatible);
    // a supplied value must be one of the three allowed regions.
    let region = match payload.region.as_deref().map(str::trim) {
        None | Some("") => "global".to_string(),
        Some(r) => {
            let r = r.to_lowercase();
            if !["american", "european", "global"].contains(&r.as_str()) {
                return Err(bad_req("region must be one of: american, european, global"));
            }
            r
        }
    };

    let slug = payload
        .slug
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| generate_slug(&payload.title));

    info!(
        action = "POST /api/articles",
        slug = %slug,
        outcome = "accepted",
        ts = %chrono::Utc::now().to_rfc3339(),
        "write-audit"
    );

    let summary = payload.summary.unwrap_or_default();
    let ai_monologue = payload.ai_monologue.unwrap_or_default();

    // Upsert article.
    // type::record('article', $slug) gives a deterministic record ID so UPSERT creates
    // the row when it doesn't exist. `UPSERT article MERGE … WHERE slug=$slug` only
    // updates existing rows and silently no-ops for new slugs (SurrealDB v3).
    // Extract h2h routing fields from pipeline_metadata and store as top-level typed
    // fields (SurrealDB TYPE object rejects nested writes on SCHEMAFULL tables without
    // FLEXIBLE — resolved by the scalar approach from 80c1d1e).
    let pm = payload.pipeline_metadata.as_ref();
    let h2h_slug = pm.and_then(|m| m.get("h2h_slug")).and_then(|v| v.as_str()).unwrap_or("").to_string();
    let h2h_role = pm.and_then(|m| m.get("h2h_role")).and_then(|v| v.as_str()).unwrap_or("").to_string();
    let h2h_order = pm.and_then(|m| m.get("h2h_order")).and_then(|v| v.as_i64()).unwrap_or(-1_i64);
    state
        .db
        .query(
            r#"
            UPSERT type::record('article', $slug) MERGE {
                slug:             $slug,
                title:            $title,
                summary:          $summary,
                body:             $body,
                category:         $category,
                region:           $region,
                persona:          IF $persona != '' THEN
                                      (SELECT id FROM persona WHERE slug = $persona LIMIT 1)[0].id
                                  ELSE NONE END,
                byline:           IF $byline != '' THEN $byline ELSE NONE END,
                h2h_slug:         IF $h2h_slug != '' THEN $h2h_slug ELSE NONE END,
                h2h_role:         IF $h2h_role != '' THEN $h2h_role ELSE NONE END,
                h2h_order:        IF $h2h_order >= 0 THEN $h2h_order ELSE NONE END,
                confidence_score:    $confidence,
                ai_monologue:        $monologue,
                ai_monologue_extended: $monologue_extended,
                status:              'published',
                published_at:     IF (SELECT published_at FROM article WHERE slug = $slug LIMIT 1)[0].published_at != NONE
                                  THEN (SELECT published_at FROM article WHERE slug = $slug LIMIT 1)[0].published_at
                                  ELSE time::now() END,
                updated_at:       time::now()
            }
            "#,
        )
        .bind(("slug", slug.clone()))
        .bind(("title", payload.title))
        .bind(("summary", summary))
        .bind(("body", payload.body))
        .bind(("category", category))
        .bind(("region", region))
        .bind(("persona", persona))
        .bind(("byline", byline))
        .bind(("h2h_slug", h2h_slug))
        .bind(("h2h_role", h2h_role))
        .bind(("h2h_order", h2h_order))
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
                        type = IF $input.type != 'wire' THEN $input.type ELSE type END,
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
            let sort_order = step_sort_order(&step.step_type);

            // Use caller-provided timestamps when available, fall back to time::now()
            let query = if step.started_at.is_some() || step.completed_at.is_some() {
                r#"
                LET $art = (SELECT id FROM article WHERE slug = $slug LIMIT 1)[0].id;
                LET $step = (CREATE pipeline_step CONTENT {
                    article:          $art,
                    agent_name:       $agent_name,
                    step_type:        $step_type,
                    input_summary:    $input_summary,
                    output_summary:   $output_summary,
                    confidence_delta: $confidence_delta,
                    sort_order:       $sort_order,
                    started_at:       IF $started_at != NONE THEN <datetime>$started_at ELSE time::now() END,
                    completed_at:     IF $completed_at != NONE THEN <datetime>$completed_at ELSE NONE END
                })[0];
                RELATE $art->produced_by->$step.id;
                "#
            } else {
                r#"
                LET $art = (SELECT id FROM article WHERE slug = $slug LIMIT 1)[0].id;
                LET $step = (CREATE pipeline_step CONTENT {
                    article:          $art,
                    agent_name:       $agent_name,
                    step_type:        $step_type,
                    input_summary:    $input_summary,
                    output_summary:   $output_summary,
                    confidence_delta: $confidence_delta,
                    sort_order:       $sort_order,
                    started_at:       time::now()
                })[0];
                RELATE $art->produced_by->$step.id;
                "#
            };

            state
                .db
                .query(query)
                .bind(("slug", slug.clone()))
                .bind(("agent_name", step.agent_name))
                .bind(("step_type", step.step_type))
                .bind(("input_summary", input_summary))
                .bind(("output_summary", output_summary))
                .bind(("confidence_delta", confidence_delta))
                .bind(("sort_order", sort_order))
                .bind(("started_at", step.started_at))
                .bind(("completed_at", step.completed_at))
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
/// Accepts a JSON array of agent status items.
pub async fn push_agent_status(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Json(items): Json<Vec<AgentStatusPushItem>>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        action = "PUT /api/agents/status",
        items = items.len(),
        outcome = "accepted",
        ts = %chrono::Utc::now().to_rfc3339(),
        "write-audit"
    );
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

const VALID_STATUSES: &[&str] = &[
    "draft", "fact_checking", "writing", "editing", "published", "rejected",
];

#[derive(Deserialize)]
pub struct ArticleStatusPatch {
    pub status: String,
    pub body: Option<String>,
    pub summary: Option<String>,
    pub confidence_score: Option<f64>,
    pub ai_monologue: Option<String>,
    pub ai_monologue_extended: Option<String>,
}

/// PATCH /api/articles/:slug — update article status and optionally content fields.
/// Required: { "status": "<valid_status>" }. Optional: body, summary, confidence_score, ai_monologue, ai_monologue_extended.
pub async fn update_article_status(
    _auth: BearerAuth,
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(payload): Json<ArticleStatusPatch>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let bad_req = |msg: &str| -> (StatusCode, Json<Value>) {
        (StatusCode::BAD_REQUEST, Json(json!({ "error": msg })))
    };

    info!(
        action = %format!("PATCH /api/articles/{}", slug),
        slug = %slug,
        status = %payload.status,
        outcome = "accepted",
        ts = %chrono::Utc::now().to_rfc3339(),
        "write-audit"
    );

    if !VALID_STATUSES.contains(&payload.status.as_str()) {
        return Err(bad_req(&format!(
            "invalid status '{}'; must be one of: {}",
            payload.status,
            VALID_STATUSES.join(", ")
        )));
    }

    let mut q = state
        .db
        .query(
            "UPDATE article SET status = $status, updated_at = time::now() WHERE slug = $slug",
        )
        .bind(("status", payload.status.clone()))
        .bind(("slug", slug.clone()))
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "database error" }))))?;

    let rows: Vec<Value> = q
        .take(0)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "database error" }))))?;

    if rows.is_empty() {
        return Err((StatusCode::NOT_FOUND, Json(json!({ "error": "article not found" }))));
    }

    // Apply optional content patches
    if let Some(body) = payload.body {
        state.db
            .query("UPDATE article SET body = $body, updated_at = time::now() WHERE slug = $slug")
            .bind(("body", body))
            .bind(("slug", slug.clone()))
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "database error" }))))?;
    }
    if let Some(summary) = payload.summary {
        state.db
            .query("UPDATE article SET summary = $summary, updated_at = time::now() WHERE slug = $slug")
            .bind(("summary", summary))
            .bind(("slug", slug.clone()))
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "database error" }))))?;
    }
    if let Some(score) = payload.confidence_score {
        state.db
            .query("UPDATE article SET confidence_score = $score, updated_at = time::now() WHERE slug = $slug")
            .bind(("score", score))
            .bind(("slug", slug.clone()))
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "database error" }))))?;
    }
    if let Some(mono) = payload.ai_monologue {
        state.db
            .query("UPDATE article SET ai_monologue = $mono, updated_at = time::now() WHERE slug = $slug")
            .bind(("mono", mono))
            .bind(("slug", slug.clone()))
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "database error" }))))?;
    }
    if let Some(ext) = payload.ai_monologue_extended {
        state.db
            .query("UPDATE article SET ai_monologue_extended = $ext, updated_at = time::now() WHERE slug = $slug")
            .bind(("ext", ext))
            .bind(("slug", slug.clone()))
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "database error" }))))?;
    }

    Ok(Json(json!({ "status": payload.status, "slug": slug })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::AppState;
    use axum::extract::Json as ExtractJson;
    use axum::extract::State;
    use surrealdb::{engine::local::SurrealKv, Surreal};

    const SCHEMA: &str = include_str!("../../db/schema.surql");

    async fn make_test_db(path: &str) -> Surreal<surrealdb::engine::local::Db> {
        let _ = std::fs::remove_dir_all(path);
        let db = Surreal::new::<SurrealKv>(path).await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        db.query(SCHEMA).await.unwrap();
        db
    }

    // Verifies the UPSERT fix at the SurrealQL level: type::record('article',$slug)
    // creates the record for a brand-new slug. Before the fix, UPSERT … WHERE slug=$slug
    // silently no-opped on SurrealDB v3 leaving zero rows.
    #[tokio::test]
    async fn upsert_type_record_creates_new_record() {
        let db = make_test_db("/tmp/sn_test_type_record").await;

        let mut res = db
            .query(r#"
                UPSERT type::record('article', $slug) MERGE {
                    slug:   $slug,
                    title:  'Regression test article',
                    body:   'Body.',
                    category: 'tech',
                    status: 'published',
                    updated_at: time::now()
                }
            "#)
            .bind(("slug", "upsert-type-record-test"))
            .await
            .expect("UPSERT type::record query must not error");
        let _: Vec<Value> = res.take(0).expect("result take must succeed");

        let mut check = db
            .query("SELECT slug FROM article WHERE slug = 'upsert-type-record-test'")
            .await
            .unwrap();
        let rows: Vec<Value> = check.take(0).unwrap();
        assert_eq!(
            rows.len(), 1,
            "UPSERT type::record must create a new row for a new slug; got {} rows", rows.len()
        );
    }

    // End-to-end: publish_article with a new slug → article is retrievable via SELECT.
    #[tokio::test]
    async fn publish_new_slug_creates_retrievable_row() {
        let db = make_test_db("/tmp/sn_test_publish_upsert").await;
        let state = AppState { db: db.clone() };

        let result = publish_article(
            BearerAuth,
            State(state),
            ExtractJson(ArticlePublishPayload {
                title: "Upsert regression test".to_string(),
                slug: Some("upsert-regression-test-slug".to_string()),
                summary: None,
                body: "Body content for upsert test.".to_string(),
                category: "tech".to_string(),
                region: None,
                persona: None,
                byline: Some("Test Desk".to_string()),
                pipeline_metadata: None,
                confidence_score: Some(0.8),
                ai_monologue: Some("Short monologue.".to_string()),
                ai_monologue_extended: Some("Extended monologue for test.".to_string()),
                sources: None,
                pipeline_steps: None,
            }),
        )
        .await;

        assert!(result.is_ok(), "publish_article returned error: {:?}", result.err());

        let mut res = db
            .query("SELECT slug FROM article WHERE slug = 'upsert-regression-test-slug'")
            .await
            .unwrap();
        let rows: Vec<Value> = res.take(0).unwrap();
        assert_eq!(rows.len(), 1, "new slug must create a row; UPSERT fix not applied — got {} rows", rows.len());
    }

    // THE-246: a publish with no `region` defaults to "global"; a publish WITH a
    // region round-trips that value. Both are read back off the persisted row.
    #[tokio::test]
    async fn region_defaults_and_round_trips() {
        let db = make_test_db("/tmp/sn_test_region").await;
        let state = AppState { db: db.clone() };

        let mk = |slug: &str, region: Option<String>| ArticlePublishPayload {
            title: format!("Region test {slug}"),
            slug: Some(slug.to_string()),
            summary: None,
            body: "Region test body.".to_string(),
            category: "tech".to_string(),
            region,
            persona: None,
            byline: Some("Test Desk".to_string()),
            pipeline_metadata: None,
            confidence_score: Some(0.8),
            ai_monologue: Some("m".to_string()),
            ai_monologue_extended: Some("ext".to_string()),
            sources: None,
            pipeline_steps: None,
        };

        // No region → global.
        publish_article(BearerAuth, State(state.clone()), ExtractJson(mk("region-default-slug", None)))
            .await
            .expect("publish without region must succeed");
        // Explicit region → round-trips.
        publish_article(BearerAuth, State(state.clone()), ExtractJson(mk("region-european-slug", Some("european".to_string()))))
            .await
            .expect("publish with region must succeed");

        let mut res = db
            .query("SELECT region FROM article WHERE slug = 'region-default-slug'; \
                    SELECT region FROM article WHERE slug = 'region-european-slug'")
            .await
            .unwrap();
        let default_rows: Vec<Value> = res.take(0).unwrap();
        let euro_rows: Vec<Value> = res.take(1).unwrap();
        assert_eq!(default_rows[0]["region"].as_str(), Some("global"), "absent region must default to global");
        assert_eq!(euro_rows[0]["region"].as_str(), Some("european"), "supplied region must round-trip");

        // An invalid region is rejected with 400.
        let bad = publish_article(BearerAuth, State(state), ExtractJson(mk("region-bad-slug", Some("antarctic".to_string())))).await;
        assert!(bad.is_err(), "invalid region must be rejected");
    }

    // THE-246: the schema seed builds the Section→Beat taxonomy. The launch beats
    // are parented under the Tech section, and the section itself has no parent.
    #[tokio::test]
    async fn taxonomy_reparents_beats_under_tech_section() {
        let db = make_test_db("/tmp/sn_test_taxonomy").await;

        let mut res = db
            .query("SELECT slug, parent.slug AS parent_slug FROM category WHERE slug IN ['linux','tech','privacy','business']; \
                    SELECT parent FROM category:tech_section")
            .await
            .unwrap();
        let beats: Vec<Value> = res.take(0).unwrap();
        assert_eq!(beats.len(), 4, "all four launch beats must exist");
        for b in &beats {
            assert_eq!(
                b["parent_slug"].as_str(), Some("tech-section"),
                "beat {:?} must be parented under tech-section", b["slug"]
            );
        }
        let section: Vec<Value> = res.take(1).unwrap();
        assert!(section[0]["parent"].is_null(), "the Tech section must have no parent (it is a Section)");
    }

    // THE-264: live-publish-path verification over real HTTP.
    //
    // Boots the actual Axum router (auth extractor + rate-limit middleware + the
    // /api routes) on a localhost TCP port against a fresh embedded SurrealKV DB
    // with the real db/schema.surql applied, then drives the deploy-path checks
    // end-to-end with an HTTP client. This is the headless-reproducible proxy for
    // the production VPS publish path (the VPS is link-local and unreachable from
    // CI / the heartbeat env); it exercises the same router, schema, and JSON
    // request/response surface a live deploy would.
    #[tokio::test]
    async fn the264_live_publish_path_over_http() {
        use tokio::net::TcpListener;

        let token = "the264-test-seed-token";
        std::env::set_var("SEED_API_TOKEN", token);

        let db = make_test_db("/tmp/sn_test_the264_http").await;
        let state = AppState { db: db.clone() };
        // Mirror production wiring (main.rs nests the api router under /api).
        let app = axum::Router::new().nest("/api", super::router(state));

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app.into_make_service()).await.unwrap();
        });
        let base = format!("http://{}", addr);
        let client = reqwest::Client::new();

        // ── Check 1: taxonomy migration + region backfill (post-schema DB state) ──
        let mut tx = db
            .query("SELECT slug, parent.slug AS parent_slug FROM category WHERE slug IN ['linux','tech','privacy','business']; \
                    SELECT parent FROM category:tech_section; \
                    SELECT count() AS c FROM article WHERE region = NONE GROUP ALL")
            .await
            .unwrap();
        let beats: Vec<Value> = tx.take(0).unwrap();
        assert_eq!(beats.len(), 4, "all four launch beats must exist after migration");
        for b in &beats {
            assert_eq!(
                b["parent_slug"].as_str(), Some("tech-section"),
                "beat {:?} must be parented under tech-section", b["slug"]
            );
        }
        let section: Vec<Value> = tx.take(1).unwrap();
        assert!(section[0]["parent"].is_null(), "tech-section must have no parent");
        let none_region: Vec<Value> = tx.take(2).unwrap();
        let none_count = none_region.first().and_then(|v| v["c"].as_i64()).unwrap_or(0);
        assert_eq!(none_count, 0, "no published article may have region = NONE after backfill");

        // ── Check: auth gating — missing/wrong bearer is rejected (fail-closed) ──
        let unauth = client
            .post(format!("{}/api/articles", base))
            .json(&json!({"title":"x","body":"x","category":"tech","ai_monologue_extended":"x"}))
            .send().await.unwrap();
        assert_eq!(unauth.status().as_u16(), 401, "publish without bearer must be 401");

        // ── Check 2: backward-compat publish with NO region field → region "global" ──
        let r2 = client
            .post(format!("{}/api/articles", base))
            .bearer_auth(token)
            .json(&json!({
                "title": "THE-264 backward compat (no region)",
                "slug": "the264-no-region",
                "body": "Backward-compat publish body.",
                "category": "tech",
                "ai_monologue_extended": "extended monologue"
            }))
            .send().await.unwrap();
        assert_eq!(r2.status().as_u16(), 200, "publish without region must succeed");
        let got2: Value = client
            .get(format!("{}/api/articles/the264-no-region", base))
            .send().await.unwrap().json().await.unwrap();
        assert_eq!(got2["region"].as_str(), Some("global"), "absent region must persist as global");

        // ── Check 3: region round-trip + ?region= filter include / exclude ──
        let r3 = client
            .post(format!("{}/api/articles", base))
            .bearer_auth(token)
            .json(&json!({
                "title": "THE-264 region round-trip",
                "slug": "the264-euro",
                "body": "European region body.",
                "category": "tech",
                "region": "european",
                "ai_monologue_extended": "extended monologue"
            }))
            .send().await.unwrap();
        assert_eq!(r3.status().as_u16(), 200, "publish with region must succeed");
        let euro: Value = client
            .get(format!("{}/api/articles/the264-euro", base))
            .send().await.unwrap().json().await.unwrap();
        assert_eq!(euro["region"].as_str(), Some("european"), "supplied region must round-trip");

        let by_euro: Value = client
            .get(format!("{}/api/articles?region=european", base))
            .send().await.unwrap().json().await.unwrap();
        let euro_slugs: Vec<&str> = by_euro["articles"].as_array().unwrap()
            .iter().filter_map(|a| a["slug"].as_str()).collect();
        assert!(euro_slugs.contains(&"the264-euro"), "?region=european must include the european article");
        assert!(!euro_slugs.contains(&"the264-no-region"), "?region=european must exclude the global article");

        let by_amer: Value = client
            .get(format!("{}/api/articles?region=american", base))
            .send().await.unwrap().json().await.unwrap();
        let amer_slugs: Vec<&str> = by_amer["articles"].as_array().unwrap()
            .iter().filter_map(|a| a["slug"].as_str()).collect();
        assert!(!amer_slugs.contains(&"the264-euro"), "?region=american must NOT include the european article");

        // ── Check 4: paywall_status regression — source COUNT survives the round-trip ──
        let r4 = client
            .post(format!("{}/api/articles", base))
            .bearer_auth(token)
            .json(&json!({
                "title": "THE-264 source count",
                "slug": "the264-sources",
                "body": "Source-count regression body.",
                "category": "tech",
                "region": "global",
                "ai_monologue_extended": "extended monologue",
                "sources": [
                    {"url":"https://example.com/a","name":"Paywalled Source","type":"press","paywall_status":"paywalled","verification_status":"verified"},
                    {"url":"https://example.com/b","name":"Free Source","type":"wire","paywall_status":"free","verification_status":"unverified"}
                ]
            }))
            .send().await.unwrap();
        assert_eq!(r4.status().as_u16(), 200, "publish with sources must succeed");
        let with_src: Value = client
            .get(format!("{}/api/articles/the264-sources", base))
            .send().await.unwrap().json().await.unwrap();
        let src_count = with_src["sources"].as_array().map(|a| a.len()).unwrap_or(0);
        assert_eq!(src_count, 2, "both sent sources must be present after publish (paywall enum drop regression)");
    }
}
