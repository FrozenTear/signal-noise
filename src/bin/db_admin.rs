/// One-shot admin utility: reject (or delete) articles by slug.
///
/// Usage:
///   cargo run --bin db_admin --features server -- reject test-article
///   cargo run --bin db_admin --features server -- delete test-article

use anyhow::{bail, Result};
use surrealdb::{engine::local::{Db, SurrealKv}, Surreal};

const DB_NS: &str = "signal_noise";
const DB_NAME: &str = "signal_noise";

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        bail!("Usage: db_admin <reject|delete> <slug>");
    }
    let cmd = &args[1];
    let slug = &args[2];

    let db: Surreal<Db> = Surreal::new::<SurrealKv>(signal_noise::config::db_path().as_str()).await?;
    db.use_ns(DB_NS).use_db(DB_NAME).await?;

    match cmd.as_str() {
        "reject" => {
            let mut res = db
                .query("UPDATE article SET status = 'rejected', updated_at = time::now() WHERE slug = $slug")
                .bind(("slug", slug.clone()))
                .await?;
            let affected: Vec<serde_json::Value> = res.take(0)?;
            println!("Rejected {} article(s) with slug '{}'", affected.len(), slug);
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
