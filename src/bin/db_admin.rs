/// One-shot admin utility: reject, delete, or publish standalone articles.
///
/// Publish uses the reusable seed_article path (no bespoke binaries).
///   cargo run --bin db_admin --features server -- publish the-119
///   (resolves docs/published/the-119/publish.json — materialize from THE-119#document-draft)
///
/// Other:
///   cargo run --bin db_admin --features server -- reject test-article "rejection note for the wall"
///   cargo run --bin db_admin --features server -- delete test-article

use anyhow::{bail, Context, Result};
use std::process::Command;
use surrealdb::{engine::local::{Db, SurrealKv}, Surreal};

const DB_PATH: &str = "data/signal-noise.db";
const DB_NS: &str = "signal_noise";
const DB_NAME: &str = "signal_noise";

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        bail!("Usage: db_admin <publish|reject|delete> <slug-or-path> [args...]\n  publish <slug>  -> seeds docs/published/<slug>/publish.json via reusable path\n  publish <file.json> works too");
    }
    let cmd = &args[1];
    let target = &args[2];

    match cmd.as_str() {
        "publish" => {
            let json_path = if target.contains('/') || target.ends_with(".json") {
                target.clone()
            } else {
                format!("docs/published/{}/publish.json", target)
            };
            if !std::path::Path::new(&json_path).exists() {
                bail!(
                    "No publish.json at {}.\n\
                     Materialize the approved draft (full transparency fields, pipeline_steps, sources) from the Paperclip issue document (e.g. THE-119#document-draft) into docs/published/<slug>/publish.json, then re-run.\n\
                     Once present: cargo run --bin db_admin --features server -- publish {}",
                    json_path, target
                );
            }
            println!("db_admin publish: delegating to reusable seed_article for {}", json_path);
            let status = Command::new("cargo")
                .args(["run", "--bin", "seed_article", "--features", "server", "--", &json_path])
                .status()
                .context("failed to spawn cargo for seed_article")?;
            if !status.success() {
                bail!("seed_article exited with failure while publishing {}", json_path);
            }
            println!("db_admin publish complete for target '{}'.", target);
            return Ok(());
        }
        "reject" | "delete" => {
            let slug = target;
            let db: Surreal<Db> = Surreal::new::<SurrealKv>(DB_PATH).await?;
            db.use_ns(DB_NS).use_db(DB_NAME).await?;

            if cmd == "reject" {
                let reason: Option<String> = if args.len() > 3 {
                    Some(args[3..].join(" "))
                } else {
                    None
                };
                let sql = if reason.is_some() {
                    "UPDATE article SET status = 'rejected', rejection_reason = $reason, updated_at = time::now() WHERE slug = $slug"
                } else {
                    "UPDATE article SET status = 'rejected', updated_at = time::now() WHERE slug = $slug"
                };
                let mut q = db.query(sql).bind(("slug", slug.to_string()));
                if let Some(r) = &reason {
                    q = q.bind(("reason", r.clone()));
                }
                let mut res = q.await?;
                let affected: Vec<serde_json::Value> = res.take(0).unwrap_or_default();
                if let Some(r) = reason {
                    println!("Rejected {} article(s) with slug '{}' and reason: {}", affected.len(), slug, r);
                } else {
                    println!("Rejected {} article(s) with slug '{}' (no reason provided)", affected.len(), slug);
                }
            } else {
                db.query("DELETE article WHERE slug = $slug")
                    .bind(("slug", slug.to_string()))
                    .await?;
                println!("Deleted article with slug '{}'", slug);
            }
        }
        other => bail!("Unknown command '{}'. Use 'publish', 'reject', or 'delete'.", other),
    }

    Ok(())
}
