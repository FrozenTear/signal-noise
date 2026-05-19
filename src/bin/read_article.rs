use surrealdb::{engine::local::{Db, SurrealKv}, Surreal};
use serde_json::Value;

const DB_PATH: &str = "data/signal-noise.db";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db: Surreal<Db> = Surreal::new::<SurrealKv>(DB_PATH).await?;
    db.use_ns("signal_noise").use_db("signal_noise").await?;
    
    let slug = std::env::args().nth(1).unwrap_or_default();
    let mut res = db.query("SELECT slug, title, body, summary, source_urls, confidence_score, ai_monologue, ai_monologue_extended FROM article WHERE slug = $slug")
        .bind(("slug", slug))
        .await?;
    let articles: Vec<Value> = res.take(0)?;
    println!("{}", serde_json::to_string_pretty(&articles)?);
    Ok(())
}
