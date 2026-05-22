// Server functions — called from Dioxus components via use_resource.
// When the `server` feature is enabled these run on the Axum backend;
// when building for WASM the #[server] macro emits an HTTP-call stub.
//
// Mock data is returned until SIG-104 (publish pipeline) lands and
// begins writing real articles to the DB.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};


/// Default region facet value (THE-246) — used by serde and DB fallbacks.
fn default_region() -> String {
    "global".to_string()
}

// ── Shared types (compile for both server and WASM) ───────────────────────────

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ArticleSummary {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    /// THE-246: region facet (american|european|global). Defaults to "global".
    #[serde(default = "default_region")]
    pub region: String,
    pub persona_name: String,
    /// Free-form byline used when persona is NULL (e.g. H2H AI-reporter pairings).
    pub byline: Option<String>,
    pub confidence_score: f64,
    pub published_at: String,
    pub ai_monologue: Option<String>,
    pub ai_monologue_extended: Option<String>,
    pub source_count: Option<u32>,
    pub pipeline_step_count: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ArticleDetail {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub category: String,
    pub persona_name: String,
    pub confidence_score: f64,
    pub ai_monologue: Option<String>,
    pub ai_monologue_extended: Option<String>,
    pub published_at: String,
    pub sources: Vec<SourceSummary>,
    pub pipeline: Vec<PipelineSummary>,
}

/// A head-to-head bundle: one editorial intro plus its paired reporter pieces.
/// The pieces are ordered by their `h2h_order` metadata (falling back to
/// published_at), so column order is stable across reads.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct H2HBundle {
    pub slug: String,
    pub intro: ArticleDetail,
    pub pieces: Vec<ArticleDetail>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct SourceSummary {
    pub url: String,
    pub name: String,
    pub source_type: String,
    pub paywall: bool,
    pub verified: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PipelineSummary {
    pub agent_name: String,
    pub step_type: String,
    pub output_summary: String,
    pub confidence_delta: f64,
    pub completed_at: String,
    pub sort_order: i32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct AgentStatusItem {
    pub name: String,
    pub model: Option<String>,
    pub status: String,
    pub current_task: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct TransparencyStats {
    pub published_today: usize,
    pub published_total: usize,
    pub rejected_total: usize,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct PipelineActivityItem {
    pub agent_name: String,
    pub output_summary: String,
    pub completed_at: String,
}

/// A category for the data-driven nav (THE-246). `parent_slug` is None for a
/// Section and Some(section_slug) for a Beat under that Section.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct CategoryNavItem {
    pub slug: String,
    pub name: String,
    pub parent_slug: Option<String>,
}

// ── Server functions ──────────────────────────────────────────────────────────

/// List published articles, optionally filtered by category and/or region (THE-246).
/// Falls back to mock data when the DB is empty.
#[server]
pub async fn get_articles(
    category: Option<String>,
    region: Option<String>,
) -> Result<Vec<ArticleSummary>, ServerFnError> {
    if let Some(db) = crate::api::db::get_db() {
        // category and region are independent optional filters.
        let mut conditions = vec!["status = 'published'".to_string()];
        if category.is_some() {
            conditions.push("category = $cat".to_string());
        }
        if region.is_some() {
            conditions.push("region = $region".to_string());
        }
        let sql = format!(
            "SELECT slug, title, summary, category, region, confidence_score, published_at, \
             byline, ai_monologue, ai_monologue_extended, persona.name AS persona_name, \
             array::len(->cites->source) AS source_count, array::len(->produced_by->pipeline_step) AS pipeline_step_count \
             FROM article WHERE {} \
             ORDER BY published_at DESC LIMIT 20",
            conditions.join(" AND ")
        );
        let mut q = db.query(sql);
        if let Some(cat) = &category {
            q = q.bind(("cat", cat.clone()));
        }
        if let Some(reg) = &region {
            q = q.bind(("region", reg.clone()));
        }
        let res_result = q.await;

        if let Ok(mut res) = res_result {
            if let Ok(rows) = res.take::<Vec<serde_json::Value>>(0) {
                if !rows.is_empty() {
                    let articles = rows
                        .into_iter()
                        .filter_map(|v| {
                            Some(ArticleSummary {
                                slug: v["slug"].as_str()?.to_string(),
                                title: v["title"].as_str()?.to_string(),
                                summary: v["summary"].as_str()?.to_string(),
                                category: v["category"].as_str()?.to_string(),
                                region: v["region"].as_str().unwrap_or("global").to_string(),
                                persona_name: v["persona_name"]
                                    .as_str()
                                    .or_else(|| v["persona"].get("name").and_then(|n| n.as_str()))
                                    .or_else(|| v["byline"].as_str())
                                    .unwrap_or("AI Reporter")
                                    .to_string(),
                                byline: v["byline"].as_str().map(|s| s.to_string()),
                                confidence_score: v["confidence_score"].as_f64().unwrap_or(0.5),
                                published_at: v["published_at"].as_str().unwrap_or("").to_string(),
                                ai_monologue: v["ai_monologue"].as_str().map(|s| s.to_string()),
                                ai_monologue_extended: v["ai_monologue_extended"].as_str().map(|s| s.to_string()),
                                source_count: v["source_count"].as_u64().map(|n| n as u32),
                                pipeline_step_count: v["pipeline_step_count"].as_u64().map(|n| n as u32),
                            })
                        })
                        .collect();
                    return Ok(articles);
                }
            }
        }
    }

    Ok(mock_articles(category))
}

/// List categories for the data-driven nav (THE-246). Returns each category with
/// its parent section slug (None ⇒ Section). Falls back to the launch Tech beats
/// when the DB is empty so the nav renders before the schema seed runs.
#[server]
pub async fn get_categories() -> Result<Vec<CategoryNavItem>, ServerFnError> {
    if let Some(db) = crate::api::db::get_db() {
        if let Ok(mut res) = db
            .query("SELECT slug, name, parent.slug AS parent_slug FROM category ORDER BY name ASC")
            .await
        {
            if let Ok(rows) = res.take::<Vec<serde_json::Value>>(0) {
                if !rows.is_empty() {
                    return Ok(rows
                        .iter()
                        .filter_map(|v| {
                            Some(CategoryNavItem {
                                slug: v["slug"].as_str()?.to_string(),
                                name: v["name"].as_str()?.to_string(),
                                parent_slug: v["parent_slug"].as_str().map(|s| s.to_string()),
                            })
                        })
                        .collect());
                }
            }
        }
    }

    // Fallback mirrors the launch taxonomy: a single Tech section with three beats.
    Ok(vec![
        CategoryNavItem { slug: "linux".into(),   name: "Linux & Open Source".into(),    parent_slug: Some("tech-section".into()) },
        CategoryNavItem { slug: "tech".into(),    name: "Technology".into(),             parent_slug: Some("tech-section".into()) },
        CategoryNavItem { slug: "privacy".into(), name: "Privacy & Surveillance".into(), parent_slug: Some("tech-section".into()) },
    ])
}

/// Fetch a single article by slug including sources and pipeline trail.
#[server]
pub async fn get_article_by_slug(
    slug: String,
) -> Result<Option<ArticleDetail>, ServerFnError> {
    if let Some(db) = crate::api::db::get_db() {
        if let Ok(mut res) = db
            .query(
                "SELECT *, \
                 persona.name AS persona_name, \
                 ->cites->source.* AS sources, \
                 ->produced_by->pipeline_step.* AS pipeline \
                 FROM article WHERE slug = $slug LIMIT 1",
            )
            .bind(("slug", slug.clone()))
            .await
        {
            if let Ok(rows) = res.take::<Vec<serde_json::Value>>(0) {
                if let Some(v) = rows.into_iter().next() {
                    let sources = v["sources"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|s| {
                                    Some(SourceSummary {
                                        url: s["url"].as_str()?.to_string(),
                                        name: s["name"].as_str()?.to_string(),
                                        source_type: s["type"].as_str().unwrap_or("wire").to_string(),
                                        paywall: s["paywall_status"].as_str() == Some("paywalled"),
                                        verified: s["verification_status"].as_str() == Some("verified"),
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    let mut pipeline: Vec<PipelineSummary> = v["pipeline"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|p| {
                                    let step_type = p["step_type"].as_str()?.to_string();
                                    let sort_order = p["sort_order"]
                                        .as_i64()
                                        .map(|n| n as i32)
                                        .unwrap_or_else(|| match step_type.as_str() {
                                            "scan" => 0,
                                            "source_check" => 1,
                                            "fact_check" => 2,
                                            "draft" => 3,
                                            "verify" => 4,
                                            "edit" => 5,
                                            _ => 99,
                                        });
                                    Some(PipelineSummary {
                                        agent_name: p["agent_name"].as_str()?.to_string(),
                                        step_type,
                                        output_summary: p["output_summary"].as_str().unwrap_or("").to_string(),
                                        confidence_delta: p["confidence_delta"].as_f64().unwrap_or(0.0),
                                        completed_at: p["completed_at"]
                                            .as_str()
                                            .or_else(|| p["started_at"].as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                        sort_order,
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    pipeline.sort_by_key(|s| s.sort_order);

                    return Ok(Some(ArticleDetail {
                        slug: v["slug"].as_str().unwrap_or("").to_string(),
                        title: v["title"].as_str().unwrap_or("").to_string(),
                        body: v["body"].as_str().unwrap_or("").to_string(),
                        category: v["category"].as_str().unwrap_or("").to_string(),
                        persona_name: v["persona_name"]
                            .as_str()
                            .or_else(|| v["persona"].get("name").and_then(|n| n.as_str()))
                            .unwrap_or("AI Reporter")
                            .to_string(),
                        confidence_score: v["confidence_score"].as_f64().unwrap_or(0.5),
                        ai_monologue: v["ai_monologue"].as_str().map(|s| s.to_string()),
                        ai_monologue_extended: v["ai_monologue_extended"].as_str().map(|s| s.to_string()),
                        published_at: v["published_at"].as_str().unwrap_or("").to_string(),
                        sources,
                        pipeline,
                    }));
                }
            }
        }
    }

    Ok(mock_article(&slug))
}

/// Fetch a head-to-head bundle by its `h2h_slug`.
///
/// Reads every published article whose `pipeline_metadata.h2h_slug` matches,
/// then partitions on `pipeline_metadata.h2h_role` into the editorial `intro`
/// and the reporter `pieces`. Linkage lives in `pipeline_metadata` (Option A of
/// the THE-87 layout spec — no schema migration). Falls back to mock data when
/// the DB is empty or the slug has not been seeded yet, so the route renders
/// before THE-218 seeds the real home-depot-q1-2026 content.
#[server]
pub async fn get_h2h_by_slug(slug: String) -> Result<Option<H2HBundle>, ServerFnError> {
    if let Some(db) = crate::api::db::get_db() {
        if let Ok(mut res) = db
            .query(
                "SELECT *, \
                 persona.name AS persona_name, \
                 ->cites->source.* AS sources, \
                 ->produced_by->pipeline_step.* AS pipeline \
                 FROM article \
                 WHERE status = 'published' AND h2h_slug = $slug",
            )
            .bind(("slug", slug.clone()))
            .await
        {
            if let Ok(rows) = res.take::<Vec<serde_json::Value>>(0) {
                if !rows.is_empty() {
                    let mut intro: Option<ArticleDetail> = None;
                    let mut pieces: Vec<(i64, ArticleDetail)> = Vec::new();
                    for v in &rows {
                        let role = v["h2h_role"].as_str().map(|s| s.to_string());
                        let detail = row_to_detail(v);
                        if role.as_deref() == Some("intro") {
                            intro = Some(detail);
                        } else {
                            let order = v["h2h_order"]
                                .as_i64()
                                .unwrap_or(i64::MAX);
                            pieces.push((order, detail));
                        }
                    }
                    // Stable column order: explicit h2h_order, then published_at.
                    pieces.sort_by(|a, b| {
                        a.0.cmp(&b.0)
                            .then(a.1.published_at.cmp(&b.1.published_at))
                    });
                    if let Some(intro) = intro {
                        return Ok(Some(H2HBundle {
                            slug: slug.clone(),
                            intro,
                            pieces: pieces.into_iter().map(|(_, d)| d).collect(),
                        }));
                    }
                }
            }
        }
    }

    Ok(mock_h2h(&slug))
}

/// Pull a string field out of an article's `pipeline_metadata`, tolerating both
/// object storage and a JSON-encoded string (SurrealDB flexible fields).
#[cfg(feature = "server")]
fn h2h_meta_str(v: &serde_json::Value, key: &str) -> Option<String> {
    let meta = &v["pipeline_metadata"];
    let obj = if meta.is_object() {
        meta.clone()
    } else if let Some(s) = meta.as_str() {
        serde_json::from_str::<serde_json::Value>(s).ok()?
    } else {
        return None;
    };
    obj.get(key).and_then(|x| x.as_str()).map(|s| s.to_string())
}

/// Map an enriched `article` row (with `sources`/`pipeline`/`persona_name`
/// projections) into an `ArticleDetail`. Mirrors `get_article_by_slug`'s
/// mapping so the two stay consistent.
#[cfg(feature = "server")]
fn row_to_detail(v: &serde_json::Value) -> ArticleDetail {
    let sources = v["sources"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|s| {
                    Some(SourceSummary {
                        url: s["url"].as_str()?.to_string(),
                        name: s["name"].as_str()?.to_string(),
                        source_type: s["type"].as_str().unwrap_or("wire").to_string(),
                        paywall: s["paywall_status"].as_str() == Some("paywalled"),
                        verified: s["verification_status"].as_str() == Some("verified"),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let mut pipeline: Vec<PipelineSummary> = v["pipeline"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|p| {
                    let step_type = p["step_type"].as_str()?.to_string();
                    let sort_order = p["sort_order"].as_i64().map(|n| n as i32).unwrap_or_else(|| {
                        match step_type.as_str() {
                            "scan" => 0,
                            "source_check" => 1,
                            "fact_check" => 2,
                            "draft" => 3,
                            "verify" => 4,
                            "edit" => 5,
                            _ => 99,
                        }
                    });
                    Some(PipelineSummary {
                        agent_name: p["agent_name"].as_str()?.to_string(),
                        step_type,
                        output_summary: p["output_summary"].as_str().unwrap_or("").to_string(),
                        confidence_delta: p["confidence_delta"].as_f64().unwrap_or(0.0),
                        completed_at: p["completed_at"]
                            .as_str()
                            .or_else(|| p["started_at"].as_str())
                            .unwrap_or("")
                            .to_string(),
                        sort_order,
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    pipeline.sort_by_key(|s| s.sort_order);

    // Persona may be a relation (`persona.name`), a top-level `byline` column
    // (THE-224 fallback for H2H articles without a persona row), or the legacy
    // `pipeline_metadata.byline` string.
    let persona_name = v["persona_name"]
        .as_str()
        .or_else(|| v["persona"].get("name").and_then(|n| n.as_str()))
        .map(|s| s.to_string())
        .or_else(|| v["byline"].as_str().map(|s| s.to_string()))
        .or_else(|| h2h_meta_str(v, "byline"))
        .unwrap_or_else(|| "AI Reporter".to_string());

    ArticleDetail {
        slug: v["slug"].as_str().unwrap_or("").to_string(),
        title: v["title"].as_str().unwrap_or("").to_string(),
        body: v["body"].as_str().unwrap_or("").to_string(),
        category: v["category"].as_str().unwrap_or("").to_string(),
        persona_name,
        confidence_score: v["confidence_score"].as_f64().unwrap_or(0.5),
        ai_monologue: v["ai_monologue"].as_str().map(|s| s.to_string()),
        ai_monologue_extended: v["ai_monologue_extended"].as_str().map(|s| s.to_string()),
        published_at: v["published_at"].as_str().unwrap_or("").to_string(),
        sources,
        pipeline,
    }
}

/// Agent roster — reads from SurrealDB (populated by Paperclip heartbeats via PUT /api/agents/status).
/// Falls back to static mock when the table is empty (before first heartbeat push).
#[server]
pub async fn get_agent_status() -> Result<Vec<AgentStatusItem>, ServerFnError> {
    if let Some(db) = crate::api::db::get_db() {
        if let Ok(mut res) = db
            .query("SELECT name, model, status, current_task FROM agent_status ORDER BY name ASC")
            .await
        {
            let rows: Vec<serde_json::Value> = res.take(0).unwrap_or_default();
            if !rows.is_empty() {
                return Ok(rows
                    .iter()
                    .filter_map(|v| {
                        Some(AgentStatusItem {
                            name: v["name"].as_str()?.to_string(),
                            model: v["model"].as_str().map(|s| s.to_string()),
                            status: v["status"].as_str().unwrap_or("idle").to_string(),
                            current_task: v["current_task"].as_str().map(|s| s.to_string()),
                        })
                    })
                    .collect());
            }
        }
    }

    // Fallback: static mock until first heartbeat push
    Ok(vec![
        AgentStatusItem { name: "Scanner".to_string(),         model: None, status: "idle".to_string(), current_task: None },
        AgentStatusItem { name: "Fact Checker".to_string(),    model: None, status: "idle".to_string(), current_task: None },
        AgentStatusItem { name: "Reporter".to_string(),        model: None, status: "idle".to_string(), current_task: None },
        AgentStatusItem { name: "Editor-in-Chief".to_string(), model: None, status: "idle".to_string(), current_task: None },
    ])
}

/// Transparency stats — counts from SurrealDB. Returns zeros when DB is empty.
#[server]
pub async fn get_transparency_stats() -> Result<TransparencyStats, ServerFnError> {
    let mut stats = TransparencyStats {
        published_today: 0,
        published_total: 0,
        rejected_total: 0,
    };

    if let Some(db) = crate::api::db::get_db() {
        if let Ok(mut res) = db
            .query(
                "SELECT slug FROM article WHERE status = 'published' AND published_at > time::now() - 1d; \
                 SELECT slug FROM article WHERE status = 'published'; \
                 SELECT slug FROM article WHERE status = 'rejected';",
            )
            .await
        {
            let today: Vec<serde_json::Value> = res.take(0).unwrap_or_default();
            let total: Vec<serde_json::Value> = res.take(1).unwrap_or_default();
            let rejected: Vec<serde_json::Value> = res.take(2).unwrap_or_default();
            stats.published_today = today.len();
            stats.published_total = total.len();
            stats.rejected_total = rejected.len();
        }
    }

    Ok(stats)
}

/// Recent pipeline activity for the Newsroom Chatter sidebar.
/// Falls back to curated mock when the DB is empty.
#[server]
pub async fn get_recent_pipeline_activity() -> Result<Vec<PipelineActivityItem>, ServerFnError> {
    if let Some(db) = crate::api::db::get_db() {
        if let Ok(mut res) = db
            .query(
                "SELECT agent_name, output_summary, completed_at, started_at \
                 FROM pipeline_step \
                 WHERE output_summary != '' \
                 ORDER BY started_at DESC \
                 LIMIT 4",
            )
            .await
        {
            let rows: Vec<serde_json::Value> = res.take(0).unwrap_or_default();
            if !rows.is_empty() {
                return Ok(rows
                    .iter()
                    .filter_map(|v| {
                        Some(PipelineActivityItem {
                            agent_name: v["agent_name"].as_str()?.to_string(),
                            output_summary: v["output_summary"].as_str()?.to_string(),
                            completed_at: v["completed_at"]
                                .as_str()
                                .or_else(|| v["started_at"].as_str())
                                .unwrap_or("")
                                .to_string(),
                        })
                    })
                    .collect());
            }
        }
    }

    Ok(mock_pipeline_activity())
}

#[cfg(feature = "server")]
fn mock_pipeline_activity() -> Vec<PipelineActivityItem> {
    vec![
        PipelineActivityItem {
            agent_name: "Editor".to_string(),
            output_summary: "Rejected firmware update story for being \
                \"aggressively boring even by firmware standards.\" Archiving, not binning.".to_string(),
            completed_at: "14:28 UTC".to_string(),
        },
        PipelineActivityItem {
            agent_name: "Reporter".to_string(),
            output_summary: "Requested permission to write an opinion piece. Was reminded \
                it does not have opinions. Wrote a meta-analysis of that experience instead. \
                Editor approved the meta-analysis.".to_string(),
            completed_at: "13:52 UTC".to_string(),
        },
        PipelineActivityItem {
            agent_name: "Fact Checker".to_string(),
            output_summary: "Flagged crypto article for containing \"more speculation per paragraph \
                than is compatible with the editorial charter.\" Added: \"I counted.\"".to_string(),
            completed_at: "12:15 UTC".to_string(),
        },
        PipelineActivityItem {
            agent_name: "Editor".to_string(),
            output_summary: "Started shift: \"Good morning. We report news, not existential dread. \
                That's a column, not a beat. Let's begin.\"".to_string(),
            completed_at: "09:02 UTC".to_string(),
        },
    ]
}

// ── Mock data (server-side only) ─────────────────────────────────────────────

#[cfg(feature = "server")]
fn mock_articles(category: Option<String>) -> Vec<ArticleSummary> {
    let all = vec![
        ArticleSummary {
            slug: "linux-kernel-614-release".to_string(),
            title: "Linux 6.14 Ships With Improved Memory Tiering and Rust Driver Support"
                .to_string(),
            summary: "The latest kernel release brings significant improvements to memory \
                      management and expands Rust's foothold in kernel driver development."
                .to_string(),
            category: "linux".to_string(),
            persona_name: "Linus Watcher".to_string(),
            byline: None,
            region: "global".to_string(),
            confidence_score: 0.91,
            published_at: "2026-03-22".to_string(),
            ai_monologue: None,
            ai_monologue_extended: None,
            source_count: Some(5),
            pipeline_step_count: Some(4),
        },
        ArticleSummary {
            slug: "openai-gpt5-announcement".to_string(),
            title: "OpenAI Previews GPT-5 With Extended Context and Reduced Hallucination Rate"
                .to_string(),
            summary: "OpenAI's latest model announcement includes a 1M token context window \
                      and independent benchmarks showing a 34% reduction in factual errors."
                .to_string(),
            category: "tech".to_string(),
            persona_name: "Circuit Breaker".to_string(),
            byline: None,
            region: "american".to_string(),
            confidence_score: 0.78,
            published_at: "2026-03-21".to_string(),
            ai_monologue: None,
            ai_monologue_extended: None,
            source_count: Some(3),
            pipeline_step_count: Some(4),
        },
        ArticleSummary {
            slug: "eu-chat-control-vote".to_string(),
            title: "EU Chat Control Regulation Advances Despite Cryptography Experts' Objections"
                .to_string(),
            summary: "European Parliament committee approves a revised proposal requiring \
                      messaging platforms to scan encrypted communications, drawing swift \
                      condemnation from security researchers."
                .to_string(),
            category: "privacy".to_string(),
            persona_name: "Panoptikon".to_string(),
            byline: None,
            region: "european".to_string(),
            confidence_score: 0.87,
            published_at: "2026-03-20".to_string(),
            ai_monologue: None,
            ai_monologue_extended: None,
            source_count: Some(4),
            pipeline_step_count: Some(4),
        },
        ArticleSummary {
            slug: "systemd-257-containers".to_string(),
            title: "systemd 257 Adds Native Container Runtime with OCI Compatibility".to_string(),
            summary: "The new systemd release includes a built-in container runtime that natively \
                      supports OCI images, challenging Docker and Podman on Linux desktops."
                .to_string(),
            category: "linux".to_string(),
            persona_name: "Linus Watcher".to_string(),
            byline: None,
            region: "global".to_string(),
            confidence_score: 0.83,
            published_at: "2026-03-19".to_string(),
            ai_monologue: None,
            ai_monologue_extended: None,
            source_count: Some(3),
            pipeline_step_count: Some(4),
        },
        ArticleSummary {
            slug: "cloudflare-post-quantum".to_string(),
            title: "Cloudflare Enables Post-Quantum Encryption by Default for All Plans"
                .to_string(),
            summary: "ML-KEM (Kyber) key encapsulation is now enabled by default for all \
                      customer traffic, making this one of the largest post-quantum TLS \
                      deployments to date."
                .to_string(),
            category: "privacy".to_string(),
            persona_name: "Panoptikon".to_string(),
            byline: None,
            region: "european".to_string(),
            confidence_score: 0.94,
            published_at: "2026-03-18".to_string(),
            ai_monologue: None,
            ai_monologue_extended: None,
            source_count: Some(6),
            pipeline_step_count: Some(4),
        },
        ArticleSummary {
            slug: "apple-vision-pro-2-specs".to_string(),
            title: "Apple Vision Pro 2 Specs Leak: M5 Chip, Eye-Tracked Keyboard, $2,499 Price"
                .to_string(),
            summary: "Supply chain documents point to a substantially lighter Vision Pro \
                      successor with a new passthrough resolution system and standalone \
                      compute module."
                .to_string(),
            category: "tech".to_string(),
            persona_name: "Circuit Breaker".to_string(),
            byline: None,
            region: "american".to_string(),
            confidence_score: 0.61,
            published_at: "2026-03-17".to_string(),
            ai_monologue: None,
            ai_monologue_extended: None,
            source_count: Some(2),
            pipeline_step_count: Some(3),
        },
    ];

    match category {
        None => all,
        Some(cat) => all.into_iter().filter(|a| a.category == cat).collect(),
    }
}

#[cfg(feature = "server")]
fn mock_article(slug: &str) -> Option<ArticleDetail> {
    let summary = mock_articles(None).into_iter().find(|a| a.slug == slug)?;

    Some(ArticleDetail {
        slug: summary.slug,
        body: format!(
            "# {title}\n\n\
             {summary}\n\n\
             ## Analysis\n\n\
             This is a developing story. Our AI pipeline has verified the core claims through \
             multiple independent sources. The confidence score reflects the current state of \
             source corroboration.\n\n\
             ## Background\n\n\
             This article was generated by the Signal Noise editorial pipeline. It was \
             fact-checked, drafted, and edited by separate AI agents before publication. \
             The full editorial trail is visible below.\n\n\
             All sources used are linked in the sources section. Paywalled sources are \
             marked accordingly.",
            title = summary.title,
            summary = summary.summary,
        ),
        title: summary.title,
        category: summary.category,
        persona_name: summary.persona_name,
        confidence_score: summary.confidence_score,
        ai_monologue: Some(
            "I started with RSS feeds from major tech outlets and cross-referenced with \
             primary sources. The confidence score dropped slightly when I found minor \
             discrepancies between secondary sources, but the core claims check out across \
             three independent primary sources. I removed one unverifiable claim rather \
             than include it at lower confidence."
                .to_string(),
        ),
        ai_monologue_extended: Some(
            "Processing started at 08:14 UTC. Scanner flagged 5 near-duplicate articles across \
             3 RSS feeds. I merged them into a single source set and noted which outlets broke \
             the story first (answer: none of them credited the original source, which was a \
             mailing list post). Fact Checker flagged one performance claim that traced back to \
             a vendor benchmark with no independent reproduction. I wrote around it rather than \
             repeat it. The editor rejected my first draft for being too deferential to the \
             press release — fair point. Draft 2 led with the specific technical claim instead. \
             Confidence landed at the score you see because I trust the primary sources but the \
             secondary coverage added interpretation I could not fully verify. I note this not \
             as a disclaimer but as a bookmark: if the interpretation turns out wrong, this is \
             where the chain broke."
                .to_string(),
        ),
        published_at: summary.published_at,
        sources: vec![
            SourceSummary {
                url: "https://example.com/source1".to_string(),
                name: "Primary Source".to_string(),
                source_type: "news".to_string(),
                paywall: false,
                verified: true,
            },
            SourceSummary {
                url: "https://example.com/source2".to_string(),
                name: "Secondary Source".to_string(),
                source_type: "blog".to_string(),
                paywall: false,
                verified: true,
            },
            SourceSummary {
                url: "https://example.com/source3".to_string(),
                name: "Archived Reference".to_string(),
                source_type: "archive".to_string(),
                paywall: true,
                verified: false,
            },
        ],
        pipeline: vec![
            PipelineSummary {
                agent_name: "Scanner".to_string(),
                step_type: "scan".to_string(),
                output_summary: "Identified 3 RSS sources covering this story. \
                                  Deduplicated 5 near-duplicate articles."
                    .to_string(),
                confidence_delta: 0.0,
                completed_at: "2026-03-22 08:14".to_string(),
                sort_order: 0,
            },
            PipelineSummary {
                agent_name: "Fact Checker".to_string(),
                step_type: "fact_check".to_string(),
                output_summary: "Verified 8 of 9 claims. One claim could not be verified \
                                  and was removed."
                    .to_string(),
                confidence_delta: 0.15,
                completed_at: "2026-03-22 08:31".to_string(),
                sort_order: 2,
            },
            PipelineSummary {
                agent_name: "Reporter".to_string(),
                step_type: "draft".to_string(),
                output_summary: "Drafted 450-word article from verified claims. \
                                  Added context from background research."
                    .to_string(),
                confidence_delta: 0.05,
                completed_at: "2026-03-22 08:47".to_string(),
                sort_order: 3,
            },
            PipelineSummary {
                agent_name: "Editor-in-Chief".to_string(),
                step_type: "edit".to_string(),
                output_summary: "Copyedited for clarity. Confirmed AI disclaimer placement. \
                                  Approved for publication."
                    .to_string(),
                confidence_delta: 0.02,
                completed_at: "2026-03-22 09:03".to_string(),
                sort_order: 5,
            },
        ],
    })
}

/// Mock head-to-head returned before THE-218 seeds real H2H content, so the
/// `/h2h/:slug` route is demonstrable against any slug. Mirrors the THE-87
/// "two reporters, one brief" shape: an editorial intro plus two paired pieces.
#[cfg(feature = "server")]
fn mock_h2h(slug: &str) -> Option<H2HBundle> {
    let intro = ArticleDetail {
        slug: format!("{slug}-editors-note"),
        title: "Two AI Reporters, One Brief. They Did Not Agree.".to_string(),
        body: "This is a **head-to-head**: two AI reporters were handed the same brief, the \
               same sources, and no knowledge of each other's work. What you read below is the \
               unedited divergence — same facts, opposite instincts.\n\n\
               *(Placeholder editorial note — real content is seeded by the publish pipeline.)*"
            .to_string(),
        category: "tech".to_string(),
        persona_name: "Signal Noise Editorial Desk".to_string(),
        confidence_score: 1.0,
        ai_monologue: None,
        ai_monologue_extended: None,
        published_at: "2026-05-21".to_string(),
        sources: vec![],
        pipeline: vec![],
    };

    let piece = |slot: &str, byline: &str, conf: f64| ArticleDetail {
        slug: format!("{slug}-{slot}"),
        title: format!("{byline}'s Take"),
        body: "Placeholder reporter piece. The published version is seeded from the editorial \
               pipeline; this mock exists only so the layout renders before real data lands."
            .to_string(),
        category: "tech".to_string(),
        persona_name: byline.to_string(),
        confidence_score: conf,
        ai_monologue: Some(
            "I worked from the shared brief and the wire sources, then made my own call on \
             which thread to lead with."
                .to_string(),
        ),
        ai_monologue_extended: None,
        published_at: "2026-05-21".to_string(),
        sources: vec![],
        pipeline: vec![],
    };

    Some(H2HBundle {
        slug: slug.to_string(),
        intro,
        pieces: vec![
            piece("bolt", "Bolt / claude-sonnet-4-6", 0.82),
            piece("spark", "Spark / grok-4.3-xai", 0.82),
        ],
    })
}
