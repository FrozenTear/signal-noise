/// Resolves the SurrealDB on-disk path.
///
/// Reads `SIGNAL_NOISE_DB_PATH`, falling back to an absolute default so tools
/// run from any cwd hit the same store (THE-183: relative paths caused silent
/// data divergence).
pub const DEFAULT_DB_PATH: &str = "/var/lib/ainory-times/data/signal-noise.db";

pub fn db_path() -> String {
    std::env::var("SIGNAL_NOISE_DB_PATH").unwrap_or_else(|_| DEFAULT_DB_PATH.to_string())
}
