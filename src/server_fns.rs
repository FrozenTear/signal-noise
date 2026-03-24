// Server functions — called from Dioxus components via use_resource.
// When the `server` feature is enabled these run on the Axum backend;
// when building for WASM the #[server] macro emits an HTTP-call stub.
//
// Mock data is returned until SIG-104 (publish pipeline) lands and
// begins writing real articles to the DB.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};


// ── Shared types (compile for both server and WASM) ───────────────────────────

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ArticleSummary {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub persona_name: String,
    pub confidence_score: f64,
    pub published_at: String,
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
    pub published_at: String,
    pub sources: Vec<SourceSummary>,
    pub pipeline: Vec<PipelineSummary>,
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
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct AgentStatusItem {
    pub name: String,
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

// ── Server functions ──────────────────────────────────────────────────────────

/// List published articles, optionally filtered by category.
/// Falls back to mock data when the DB is empty (pre-SIG-104).
#[server]
pub async fn get_articles(category: Option<String>) -> Result<Vec<ArticleSummary>, ServerFnError> {
    use axum::Extension;
    use dioxus_fullstack::FullstackContext;
    use surrealdb::{engine::local::Db, Surreal};

    if let Ok(Extension(db)) = FullstackContext::extract::<Extension<Surreal<Db>>, _>().await {
        let mut res_result = if let Some(cat) = &category {
            db.query(
                "SELECT slug, title, summary, category, confidence_score, published_at \
                 FROM article WHERE status = 'published' AND category = $cat \
                 ORDER BY published_at DESC LIMIT 20",
            )
            .bind(("cat", cat.clone()))
            .await
        } else {
            db.query(
                "SELECT slug, title, summary, category, confidence_score, published_at \
                 FROM article WHERE status = 'published' \
                 ORDER BY published_at DESC LIMIT 20",
            )
            .await
        };

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
                                persona_name: v["persona_name"]
                                    .as_str()
                                    .unwrap_or("AI Reporter")
                                    .to_string(),
                                confidence_score: v["confidence_score"].as_f64().unwrap_or(0.5),
                                published_at: v["published_at"].as_str().unwrap_or("").to_string(),
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

/// Fetch a single article by slug including sources and pipeline trail.
#[server]
pub async fn get_article_by_slug(
    slug: String,
) -> Result<Option<ArticleDetail>, ServerFnError> {
    use axum::Extension;
    use dioxus_fullstack::FullstackContext;
    use surrealdb::{engine::local::Db, Surreal};

    if let Ok(Extension(db)) = FullstackContext::extract::<Extension<Surreal<Db>>, _>().await {
        if let Ok(mut res) = db
            .query(
                "SELECT *, \
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

                    let pipeline = v["pipeline"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|p| {
                                    Some(PipelineSummary {
                                        agent_name: p["agent_name"].as_str()?.to_string(),
                                        step_type: p["step_type"].as_str()?.to_string(),
                                        output_summary: p["output_summary"].as_str().unwrap_or("").to_string(),
                                        confidence_delta: p["confidence_delta"].as_f64().unwrap_or(0.0),
                                        completed_at: p["completed_at"]
                                            .as_str()
                                            .or_else(|| p["started_at"].as_str())
                                            .unwrap_or("")
                                            .to_string(),
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default();

                    return Ok(Some(ArticleDetail {
                        slug: v["slug"].as_str().unwrap_or("").to_string(),
                        title: v["title"].as_str().unwrap_or("").to_string(),
                        body: v["body"].as_str().unwrap_or("").to_string(),
                        category: v["category"].as_str().unwrap_or("").to_string(),
                        persona_name: "AI Reporter".to_string(),
                        confidence_score: v["confidence_score"].as_f64().unwrap_or(0.5),
                        ai_monologue: v["ai_monologue"].as_str().map(|s| s.to_string()),
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

/// Agent roster — queries Paperclip API for live status; falls back to static mock.
/// Set PAPERCLIP_API_URL, PAPERCLIP_API_KEY, and PAPERCLIP_COMPANY_ID env vars to enable.
#[server]
pub async fn get_agent_status() -> Result<Vec<AgentStatusItem>, ServerFnError> {
    let api_url = std::env::var("PAPERCLIP_API_URL").unwrap_or_default();
    let api_key = std::env::var("PAPERCLIP_API_KEY").unwrap_or_default();
    let company_id = std::env::var("PAPERCLIP_COMPANY_ID").unwrap_or_default();

    if !api_url.is_empty() && !api_key.is_empty() && !company_id.is_empty() {
        if let Ok(items) = fetch_paperclip_agents(&api_url, &api_key, &company_id).await {
            if !items.is_empty() {
                return Ok(items);
            }
        }
    }

    Ok(vec![
        AgentStatusItem { name: "Scanner".to_string(),          status: "idle".to_string(), current_task: None },
        AgentStatusItem { name: "Fact Checker".to_string(),     status: "idle".to_string(), current_task: None },
        AgentStatusItem { name: "Reporter".to_string(),         status: "idle".to_string(), current_task: None },
        AgentStatusItem { name: "Editor-in-Chief".to_string(),  status: "idle".to_string(), current_task: None },
    ])
}

#[cfg(feature = "server")]
async fn fetch_paperclip_agents(
    api_url: &str,
    api_key: &str,
    company_id: &str,
) -> Result<Vec<AgentStatusItem>, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let agents: serde_json::Value = client
        .get(format!("{}/api/companies/{}/agents", api_url, company_id))
        .bearer_auth(api_key)
        .send()
        .await?
        .json()
        .await?;

    let issues: serde_json::Value = client
        .get(format!(
            "{}/api/companies/{}/issues?status=in_progress",
            api_url, company_id
        ))
        .bearer_auth(api_key)
        .send()
        .await?
        .json()
        .await?;

    // Build agent-id → current task title map
    let mut task_map: std::collections::HashMap<String, String> = Default::default();
    if let Some(arr) = issues.as_array() {
        for issue in arr {
            if let (Some(agent_id), Some(title)) = (
                issue["assigneeAgentId"].as_str(),
                issue["title"].as_str(),
            ) {
                task_map.insert(agent_id.to_string(), title.to_string());
            }
        }
    }

    let items = agents
        .as_array()
        .cloned()
        .unwrap_or_default()
        .iter()
        .map(|a| {
            let id = a["id"].as_str().unwrap_or("").to_string();
            let name = a["name"].as_str().unwrap_or("Unknown Agent").to_string();
            let current_task = task_map.get(&id).cloned();
            let status = if current_task.is_some() { "working" } else { "idle" };
            AgentStatusItem {
                name,
                status: status.to_string(),
                current_task,
            }
        })
        .collect();

    Ok(items)
}

/// Transparency stats — counts from SurrealDB. Returns zeros when DB is empty.
#[server]
pub async fn get_transparency_stats() -> Result<TransparencyStats, ServerFnError> {
    use axum::Extension;
    use dioxus_fullstack::FullstackContext;
    use surrealdb::{engine::local::Db, Surreal};

    let mut stats = TransparencyStats {
        published_today: 0,
        published_total: 0,
        rejected_total: 0,
    };

    if let Ok(Extension(db)) = FullstackContext::extract::<Extension<Surreal<Db>>, _>().await {
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
    use axum::Extension;
    use dioxus_fullstack::FullstackContext;
    use surrealdb::{engine::local::Db, Surreal};

    if let Ok(Extension(db)) = FullstackContext::extract::<Extension<Surreal<Db>>, _>().await {
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
            confidence_score: 0.91,
            published_at: "2026-03-22".to_string(),
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
            confidence_score: 0.78,
            published_at: "2026-03-21".to_string(),
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
            confidence_score: 0.87,
            published_at: "2026-03-20".to_string(),
        },
        ArticleSummary {
            slug: "systemd-257-containers".to_string(),
            title: "systemd 257 Adds Native Container Runtime with OCI Compatibility".to_string(),
            summary: "The new systemd release includes a built-in container runtime that natively \
                      supports OCI images, challenging Docker and Podman on Linux desktops."
                .to_string(),
            category: "linux".to_string(),
            persona_name: "Linus Watcher".to_string(),
            confidence_score: 0.83,
            published_at: "2026-03-19".to_string(),
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
            confidence_score: 0.94,
            published_at: "2026-03-18".to_string(),
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
            confidence_score: 0.61,
            published_at: "2026-03-17".to_string(),
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
            },
            PipelineSummary {
                agent_name: "Fact Checker".to_string(),
                step_type: "fact_check".to_string(),
                output_summary: "Verified 8 of 9 claims. One claim could not be verified \
                                  and was removed."
                    .to_string(),
                confidence_delta: 0.15,
                completed_at: "2026-03-22 08:31".to_string(),
            },
            PipelineSummary {
                agent_name: "Reporter".to_string(),
                step_type: "draft".to_string(),
                output_summary: "Drafted 450-word article from verified claims. \
                                  Added context from background research."
                    .to_string(),
                confidence_delta: 0.05,
                completed_at: "2026-03-22 08:47".to_string(),
            },
            PipelineSummary {
                agent_name: "Editor-in-Chief".to_string(),
                step_type: "edit".to_string(),
                output_summary: "Copyedited for clarity. Confirmed AI disclaimer placement. \
                                  Approved for publication."
                    .to_string(),
                confidence_delta: 0.02,
                completed_at: "2026-03-22 09:03".to_string(),
            },
        ],
    })
}
