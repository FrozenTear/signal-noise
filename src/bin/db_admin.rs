/// One-shot admin utility: reject (or delete) articles by slug.
///
/// Usage:
///   cargo run --bin db_admin --features server -- reject test-article "rejection note for the wall"
///   cargo run --bin db_admin --features server -- reject test-article
///   cargo run --bin db_admin --features server -- delete test-article

use anyhow::{bail, Result};
use surrealdb::{engine::local::{Db, SurrealKv}, Surreal};

const DB_PATH: &str = "data/signal-noise.db";
const DB_NS: &str = "signal_noise";
const DB_NAME: &str = "signal_noise";

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        bail!("Usage: db_admin <reject|delete> <slug> [rejection reason text...]");
    }
    let cmd = &args[1];
    let slug = &args[2];

    let db: Surreal<Db> = Surreal::new::<SurrealKv>(DB_PATH).await?;
    db.use_ns(DB_NS).use_db(DB_NAME).await?;

    match cmd.as_str() {
        "reject" => {
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
            let mut q = db.query(sql).bind(("slug", slug.clone()));
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
        }
        "delete" => {
            db.query("DELETE article WHERE slug = $slug")
                .bind(("slug", slug.clone()))
                .await?;
            println!("Deleted article with slug '{}'", slug);
        }
        other => bail!("Unknown command '{}'. Use 'reject' or 'delete'.", other),
    }

    Ok(())
}
