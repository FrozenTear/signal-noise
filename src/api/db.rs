use anyhow::Result;
use surrealdb::{
    engine::local::{Db, SurrealKv},
    Surreal,
};

const SCHEMA_SURQL: &str = include_str!("../../db/schema.surql");
const DB_PATH: &str = "data/signal-noise.db";
const DB_NS: &str = "signal_noise";
const DB_NAME: &str = "signal_noise";

pub async fn init_db() -> Result<Surreal<Db>> {
    let db = match Surreal::new::<SurrealKv>(DB_PATH).await {
        Ok(db) => db,
        Err(e) if e.to_string().contains("already locked") || e.to_string().contains("LOCK") => {
            let lock_path = format!("{}/LOCK", DB_PATH);
            tracing::warn!("Stale SurrealDB lock detected, removing: {}", lock_path);
            std::fs::remove_file(&lock_path)?;
            Surreal::new::<SurrealKv>(DB_PATH).await?
        }
        Err(e) => return Err(e.into()),
    };
    db.use_ns(DB_NS).use_db(DB_NAME).await?;
    Ok(db)
}

pub async fn apply_schema(db: &Surreal<Db>) -> Result<()> {
    db.query(SCHEMA_SURQL).await?;
    Ok(())
}
