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

// ── Server functions ──────────────────────────────────────────────────────────

/// List published articles, optionally filtered by category.
/// Falls back to mock data when the DB is empty (pre-SIG-104).
#[server]
pub async fn get_articles(category: Option<String>) -> Result<Vec<ArticleSummary>, ServerFnError> {
    use axum::Extension;
    use dioxus_fullstack::FullstackContext;
    use surrealdb::{engine::local::Db, Surreal};

    if let Ok(Extension(db)) = FullstackContext::extract::<Extension<Surreal<Db>>, _>().await {
        let query = match &category {
            Some(cat) => format!(
                "SELECT slug, title, summary, category, confidence_score, published_at \
                 FROM article WHERE status = 'published' AND category = '{}' \
                 ORDER BY published_at DESC LIMIT 20",
                cat
            ),
            None => "SELECT slug, title, summary, category, confidence_score, published_at \
                     FROM article WHERE status = 'published' \
                     ORDER BY published_at DESC LIMIT 20"
                .to_string(),
        };

        if let Ok(mut res) = db.query(query).await {
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
        let query = format!(
            "SELECT slug, title, body, category, confidence_score, ai_monologue, published_at \
             FROM article WHERE slug = '{}' LIMIT 1",
            slug
        );

        if let Ok(mut res) = db.query(query).await {
            if let Ok(rows) = res.take::<Vec<serde_json::Value>>(0) {
                if let Some(v) = rows.into_iter().next() {
                    return Ok(Some(ArticleDetail {
                        slug: v["slug"].as_str().unwrap_or("").to_string(),
                        title: v["title"].as_str().unwrap_or("").to_string(),
                        body: v["body"].as_str().unwrap_or("").to_string(),
                        category: v["category"].as_str().unwrap_or("").to_string(),
                        persona_name: "AI Reporter".to_string(),
                        confidence_score: v["confidence_score"].as_f64().unwrap_or(0.5),
                        ai_monologue: v["ai_monologue"].as_str().map(|s| s.to_string()),
                        published_at: v["published_at"].as_str().unwrap_or("").to_string(),
                        sources: vec![],
                        pipeline: vec![],
                    }));
                }
            }
        }
    }

    Ok(mock_article(&slug))
}

/// Agent roster — currently returns a static list.
/// Will be wired to Paperclip API once the integration is in place.
#[server]
pub async fn get_agent_status() -> Result<Vec<AgentStatusItem>, ServerFnError> {
    Ok(vec![
        AgentStatusItem {
            name: "Scanner".to_string(),
            status: "idle".to_string(),
            current_task: None,
        },
        AgentStatusItem {
            name: "Fact Checker".to_string(),
            status: "idle".to_string(),
            current_task: None,
        },
        AgentStatusItem {
            name: "Reporter".to_string(),
            status: "idle".to_string(),
            current_task: None,
        },
        AgentStatusItem {
            name: "Editor-in-Chief".to_string(),
            status: "idle".to_string(),
            current_task: None,
        },
    ])
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
