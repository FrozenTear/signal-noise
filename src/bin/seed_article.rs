/// Reusable standalone-article DB seed (direct-to-Surreal, no server required).
///
/// Replaces per-article one-off bins (seed_the122 etc.) with a single generic
/// tool. Feed it a publish.json (or equivalent) that matches the shape used by
/// POST /api/articles plus seed extras.
///
/// Usage:
///   cargo run --bin seed_article --features server -- docs/published/the-116/publish.json
///   cargo run --bin seed_article --features server -- docs/published/the-119/publish.json
///
/// The JSON may include optional top-level fields for seed:
///   "published_at": "2026-05-21T12:00:00Z",
///   "issue": "THE-119",
///   "pipeline_steps": [ ... ]   (full audit trail; recommended for transparency)
///
/// Idempotent: re-running the same slug deletes prior article row + its
/// produced_by/cites edges and pipeline_step rows before re-insert (sources
/// are UPSERTed by url and left intact for sharing across articles).
///
/// This is the official DB seed path for standalone articles (THE-119 will be
/// the first published through it once its publish.json lands).

use anyhow::{bail, Context, Result};
use serde::Deserialize;
use serde_json::json;
use surrealdb::{engine::local::{Db, SurrealKv}, Surreal};

const DB_PATH: &str = "data/signal-noise.db";
const DB_NS: &str = "signal_noise";
const DB_NAME: &str = "signal_noise";

#[derive(Debug, Deserialize)]
struct SeedPayload {
    title: String,
    slug: Option<String>,
    summary: Option<String>,
    body: String,
    category: String,
    persona: Option<String>,
    confidence_score: Option<f64>,
    ai_monologue: Option<String>,
    ai_monologue_extended: Option<String>,
    sources: Option<Vec<SourceSeed>>,
    pipeline_steps: Option<Vec<PipelineStepSeed>>,
    // Seed-only / optional overrides
    published_at: Option<String>,
    issue: Option<String>,
    byline: Option<String>,
    model_attribution: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SourceSeed {
    url: String,
    name: String,
    #[serde(rename = "type", default = "default_source_type")]
    source_type: String,
    #[serde(default)]
    paywall_status: Option<String>,
    #[serde(default)]
    verification_status: Option<String>,
}

fn default_source_type() -> String {
    "press".to_string()
}

#[derive(Debug, Deserialize)]
struct PipelineStepSeed {
    agent_name: String,
    step_type: String,
    input_summary: Option<String>,
    output_summary: Option<String>,
    confidence_delta: Option<f64>,
    started_at: Option<String>,
    completed_at: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        bail!(
            "Usage: seed_article <path-to-publish.json>\n\nExample:\n  cargo run --bin seed_article --features server -- docs/published/the-119/publish.json"
        );
    }
    let json_path = &args[1];

    let raw = std::fs::read_to_string(json_path)
        .with_context(|| format!("failed to read publish json at {}", json_path))?;
    let payload: SeedPayload = serde_json::from_str(&raw)
        .with_context(|| "failed to parse SeedPayload / ArticlePublishPayload shape from json")?;

    if payload.title.trim().is_empty() || payload.body.trim().is_empty() || payload.category.trim().is_empty() {
        bail!("title, body, and category are required in the publish json");
    }
    let ai_monologue_extended = payload
        .ai_monologue_extended
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("ai_monologue_extended is required and must be non-empty for standalone seeds"))?;

    let confidence = payload.confidence_score.unwrap_or(0.0);
    if !(0.0..=1.0).contains(&confidence) {
        bail!("confidence_score must be between 0.0 and 1.0");
    }

    let persona = payload
        .persona
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| anyhow::anyhow!("persona is required for standalone seed"))?;
    let category = payload.category.trim().to_string();

    let slug = payload
        .slug
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| {
            payload
                .title
                .to_lowercase()
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '-' })
                .collect::<String>()
                .split('-')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("-")
        });

    let summary = payload.summary.unwrap_or_default();
    let ai_monologue = payload.ai_monologue.unwrap_or_default();
    let published_at = payload
        .published_at
        .unwrap_or_else(|| "2026-05-21T12:00:00Z".to_string());

    // Build pipeline_metadata (include seed extras so the UI can show issue/byline etc.)
    let pm = json!({
        "byline": payload.byline.unwrap_or_else(|| format!("{} (seed)", persona)),
        "model_attribution": payload.model_attribution.unwrap_or_else(|| "seed".to_string()),
        "persona": persona,
        "issue": payload.issue,
        "source_substitution": false,
        "seeded_via": "seed_article",
        "seed_json": json_path,
    });

    let db: Surreal<Db> = Surreal::new::<SurrealKv>(DB_PATH).await?;
    db.use_ns(DB_NS).use_db(DB_NAME).await?;

    // Ensure core personas exist (INSERT IGNORE mirrors db/schema.surql; required for
    // article.persona link + bylines in UI). This unblocks seeds for priya-nair (THE-121),
    // milo-varga (THE-116), sable-ren (THE-124/122) without requiring a prior server start.
    let _ = db
        .query(
            r#"INSERT IGNORE INTO persona (slug, name, bio, beat, writing_style_guide, example_phrases, is_active) VALUES
            ("priya-nair", "Priya Nair", "Priya covers the technology beat with a focus on developer tools, AI research, and the business of tech. She writes with precision and a healthy skepticism toward hype.", "tech", "Clear, analytical prose. Cite sources inline. Flag uncertainty with hedged language. Avoid superlatives. Short paragraphs.", [], true),
            ("milo-varga", "Milo Varga", "Milo covers the Linux kernel, distributions, and the free software ecosystem. A systems-level thinker who writes like someone who has been reading changelogs since before most repos had README files.", "linux", "Technical but accessible. Uses concrete examples over abstractions. Dry humor — earned, not forced.", [], true),
            ("sable-ren", "Sable Ren", "Sable covers surveillance, data rights, and the intersection of policy and technology. Writes with the rigor of someone who reads court filings for fun.", "privacy", "Precise legal-technical language. Never sensational. Always names the actors.", [], true)
        "#,
        )
        .await;

    // ── Idempotency: remove prior article + its private edges/steps (sources stay) ──
    let aid = format!("article:`{}`", slug);
    let _ = db.query(format!("DELETE pipeline_step WHERE article = {aid}")).await;
    let _ = db.query(format!("DELETE {aid}->produced_by")).await;
    let _ = db.query(format!("DELETE {aid}->cites")).await;
    let _ = db
        .query("DELETE article WHERE slug = $slug")
        .bind(("slug", slug.clone()))
        .await;

    // ── Insert article (record syntax for direct offline seed) ────────────────────
    db.query(format!(
        "CREATE {aid} SET \
         slug = $slug, \
         title = $title, \
         summary = $summary, \
         body = $body, \
         category = $category, \
         persona = (SELECT VALUE id FROM persona WHERE slug = $persona LIMIT 1)[0], \
         confidence_score = $confidence, \
         ai_monologue = $monologue, \
         ai_monologue_extended = $extended, \
         pipeline_metadata = $pm, \
         source_urls = $source_urls, \
         status = 'published', \
         published_at = <datetime>$published_at"
    ))
    .bind(("slug", slug.clone()))
    .bind(("title", payload.title.clone()))
    .bind(("summary", summary.clone()))
    .bind(("body", payload.body.clone()))
    .bind(("category", category.clone()))
    .bind(("persona", persona.clone()))
    .bind(("confidence", confidence))
    .bind(("monologue", ai_monologue.clone()))
    .bind(("extended", ai_monologue_extended.clone()))
    .bind(("pm", pm.clone()))
    .bind(("source_urls", payload.sources.as_ref().map(|ss| ss.iter().map(|s| s.url.clone()).collect::<Vec<_>>()).unwrap_or_default()))
    .bind(("published_at", published_at.clone()))
    .await
    .context("failed to CREATE article in seed")?;

    // ── Sources + cites edges (idempotent UPSERT on source url) ──────────────────
    if let Some(sources) = &payload.sources {
        for s in sources {
            let paywall = s.paywall_status.clone().unwrap_or_else(|| "unknown".to_string());
            let verification = s.verification_status.clone().unwrap_or_else(|| "unknown".to_string());
            let key = s.url.replace("https://", "").replace("http://", "").replace(['/', '.', ':', '?', '&', '=', '#'], "-");
            let sid = format!("source:`{}`", key);

            db.query(format!(
                "UPSERT {sid} SET \
                 url = $url, name = $name, type = $stype, \
                 paywall_status = $paywall, verification_status = $verification"
            ))
            .bind(("url", s.url.clone()))
            .bind(("name", s.name.clone()))
            .bind(("stype", s.source_type.clone()))
            .bind(("paywall", paywall.clone()))
            .bind(("verification", verification.clone()))
            .await?;

            let excerpt = if verification == "verified" { "true" } else { "false" };
            db.query(format!(
                "RELATE {aid}->cites->{sid} CONTENT {{ \
                 excerpt_available: {excerpt}, \
                 verification_status_at_cite_time: $verification }}"
            ))
            .bind(("verification", verification.clone()))
            .await?;
        }
    }

    // ── Pipeline steps + produced_by edges (full transparency trail) ─────────────
    if let Some(steps) = &payload.pipeline_steps {
        for (i, step) in steps.iter().enumerate() {
            let pid = format!("pipeline_step:`{}_{}`", slug, i);
            let sort_order = match step.step_type.as_str() {
                "scan" => 0,
                "source_check" => 1,
                "fact_check" => 2,
                "draft" => 3,
                "verify" => 4,
                "edit" => 5,
                _ => 99,
            };
            let input = step.input_summary.clone().unwrap_or_default();
            let output = step.output_summary.clone().unwrap_or_default();
            let delta = step.confidence_delta.unwrap_or(0.0);
            let started = step.started_at.clone().unwrap_or_else(|| published_at.clone());
            let completed = step.completed_at.clone().unwrap_or_else(|| published_at.clone());

            db.query(format!(
                "CREATE {pid} SET \
                 article = {aid}, \
                 agent_name = $agent, \
                 step_type = $step_type, \
                 sort_order = $sort_order, \
                 input_summary = $input, \
                 output_summary = $output, \
                 confidence_delta = $delta, \
                 started_at = <datetime>$started, \
                 completed_at = <datetime>$completed"
            ))
            .bind(("agent", step.agent_name.clone()))
            .bind(("step_type", step.step_type.clone()))
            .bind(("sort_order", sort_order))
            .bind(("input", input))
            .bind(("output", output))
            .bind(("delta", delta))
            .bind(("started", started))
            .bind(("completed", completed))
            .await?;

            db.query(format!("RELATE {aid}->produced_by->{pid}")).await?;
        }
    }

    println!("Seeded standalone article via reusable path:");
    println!("  slug:      /article/{}", slug);
    println!("  json:      {}", json_path);
    println!("  category:  {}", category);
    println!("  persona:   {}", persona);
    if payload.pipeline_steps.as_ref().map_or(0, |v| v.len()) > 0 {
        println!("  pipeline:  {} steps recorded", payload.pipeline_steps.as_ref().unwrap().len());
    } else {
        println!("  pipeline:  (none provided — add pipeline_steps for full transparency)");
    }
    println!("  sources:   {}", payload.sources.as_ref().map_or(0, |v| v.len()));
    println!("\nReady for THE-119 (and future standalone articles). Re-run with the same json for idempotent refresh.");

    Ok(())
}