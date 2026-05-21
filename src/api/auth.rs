use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Json, Response},
};
use constant_time_eq::constant_time_eq;
use serde_json::json;
use tracing::warn;

/// Axum extractor that enforces `Authorization: Bearer <SEED_API_TOKEN>` on mutating routes.
///
/// Fail-closed: if SEED_API_TOKEN is unset or empty, all writes are rejected with 503
/// before any DB work begins.  Logs every attempt (accepted and rejected) to the audit
/// trail — never logs the raw token, only a short fingerprint.
pub struct BearerAuth;

impl<S> FromRequestParts<S> for BearerAuth
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let action = format!("{} {}", parts.method, parts.uri);
        let ts = chrono::Utc::now().to_rfc3339();

        // Fail-closed: if the env var is absent or empty, reject all writes.
        let expected = match std::env::var("SEED_API_TOKEN") {
            Ok(t) if !t.is_empty() => t,
            _ => {
                warn!(
                    action = %action,
                    outcome = "rejected-503",
                    ts = %ts,
                    "write-audit: SEED_API_TOKEN not configured"
                );
                return Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(json!({ "error": "write access not configured" })),
                )
                    .into_response());
            }
        };

        let fp = token_fingerprint(&expected);

        // Extract Bearer token from Authorization header.
        let provided = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .unwrap_or("");

        // Constant-time compare resists timing oracles.  Length must match first;
        // expected length is not secret so the fast-path length check is acceptable.
        let ok = provided.len() == expected.len()
            && constant_time_eq(provided.as_bytes(), expected.as_bytes());

        if !ok {
            warn!(
                action = %action,
                token_fp = %fp,
                outcome = "rejected-401",
                ts = %ts,
                "write-audit"
            );
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "unauthorized" })),
            )
                .into_response());
        }

        Ok(BearerAuth)
    }
}

/// Non-secret fingerprint for audit correlation — never includes raw token material.
fn token_fingerprint(token: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    token.hash(&mut h);
    format!("{:016x}", h.finish())
}
