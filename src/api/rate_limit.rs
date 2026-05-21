// In-memory fixed-window rate limiter for mutating /api/* routes.
//
// Two independent buckets:
//   - Per client IP  : 30 mutating requests / 60 s
//   - Per bearer token (fingerprint only): 60 mutating requests / 60 s
//
// GET requests are never throttled — call next.run(req) immediately.
// Rejected requests return 429 with Retry-After: 60 and emit a write-audit log
// line in the same format used by auth.rs / routes.rs.

use axum::{
    extract::{Request, State},
    http::{HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tracing::warn;

struct Window {
    count: u32,
    reset_at: Instant,
}

/// Shared state for the rate-limit middleware. Clone is cheap — all mutable
/// data lives behind Arc<Mutex<_>>.
#[derive(Clone)]
pub struct RateLimitState {
    ip_buckets: Arc<Mutex<HashMap<String, Window>>>,
    token_buckets: Arc<Mutex<HashMap<String, Window>>>,
    ip_limit: u32,
    token_limit: u32,
    window: Duration,
    // Caps prevent memory-exhaustion DoS from attacker-controlled keys
    // (spoofed IPs, random Bearer tokens). When a map hits the ceiling,
    // expired entries are swept before admitting new keys.
    ip_max_buckets: usize,
    token_max_buckets: usize,
}

impl RateLimitState {
    pub fn new() -> Self {
        Self {
            ip_buckets: Arc::new(Mutex::new(HashMap::new())),
            token_buckets: Arc::new(Mutex::new(HashMap::new())),
            ip_limit: 30,
            token_limit: 60,
            window: Duration::from_secs(60),
            ip_max_buckets: 10_000,
            token_max_buckets: 20_000,
        }
    }

    /// Returns true if the key is within its rate-limit window, false if the
    /// request should be rejected.  Holds the Mutex only for the duration of a
    /// HashMap lookup + integer increment — no .await while locked.
    fn check(
        &self,
        buckets: &Arc<Mutex<HashMap<String, Window>>>,
        key: &str,
        limit: u32,
        max_buckets: usize,
    ) -> bool {
        let mut map = buckets.lock().expect("rate-limit mutex poisoned");
        let now = Instant::now();
        let window = self.window;

        // Evict expired windows when the map reaches the cap. Without this,
        // an attacker can exhaust memory by sending requests with random
        // Bearer tokens (hashed before auth) or spoofed IPs.
        if map.len() >= max_buckets {
            map.retain(|_, w| now < w.reset_at);
        }

        let entry = map.entry(key.to_string()).or_insert_with(|| Window {
            count: 0,
            reset_at: now + window,
        });
        if now >= entry.reset_at {
            entry.count = 1;
            entry.reset_at = now + window;
        } else {
            entry.count += 1;
        }
        entry.count <= limit
    }
}

/// Best-effort client IP extraction. Uses the **rightmost** X-Forwarded-For
/// entry (the IP Caddy directly observed on the incoming connection), falls
/// back to X-Real-Ip, and finally returns "unknown" so the bucket always
/// exists (prevents bypass by omitting headers).
///
/// Caddy appends to XFF, so leftmost entries are attacker-controlled and
/// must not be trusted for rate-limiting.
fn client_ip(req: &Request) -> String {
    if let Some(xff) = req.headers().get("x-forwarded-for") {
        if let Ok(val) = xff.to_str() {
            if let Some(ip) = val.split(',').last().map(str::trim) {
                if !ip.is_empty() {
                    return ip.to_string();
                }
            }
        }
    }
    if let Some(xri) = req.headers().get("x-real-ip") {
        if let Ok(val) = xri.to_str() {
            return val.trim().to_string();
        }
    }
    "unknown".to_string()
}

/// Non-secret fingerprint of the Bearer token — same algorithm as auth.rs so
/// audit logs can be correlated without exposing raw token material.
fn bearer_token_fp(req: &Request) -> Option<String> {
    req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|token| {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            token.hash(&mut h);
            format!("{:016x}", h.finish())
        })
}

fn is_mutating(method: &Method) -> bool {
    matches!(method, &Method::POST | &Method::PUT | &Method::PATCH | &Method::DELETE)
}

fn rate_limited_response(msg: &str) -> Response {
    let mut res = (StatusCode::TOO_MANY_REQUESTS, Json(json!({ "error": msg }))).into_response();
    res.headers_mut()
        .insert("Retry-After", HeaderValue::from_static("60"));
    res
}

/// Axum middleware: rate-limits mutating (POST/PUT/PATCH/DELETE) requests by
/// both client IP and bearer token fingerprint.  GET is passed through
/// immediately with no counter update.
pub async fn rate_limit_middleware(
    State(rl): State<RateLimitState>,
    req: Request,
    next: Next,
) -> Response {
    if !is_mutating(req.method()) {
        return next.run(req).await;
    }

    let ip = client_ip(&req);
    let token_fp = bearer_token_fp(&req);
    let action = format!("{} {}", req.method(), req.uri());
    let ts = chrono::Utc::now().to_rfc3339();

    if !rl.check(&rl.ip_buckets, &ip, rl.ip_limit, rl.ip_max_buckets) {
        warn!(
            action = %action,
            ip = %ip,
            outcome = "rate-limited-429-ip",
            ts = %ts,
            "write-audit"
        );
        return rate_limited_response("rate limit exceeded");
    }

    if let Some(fp) = token_fp {
        if !rl.check(&rl.token_buckets, &fp, rl.token_limit, rl.token_max_buckets) {
            warn!(
                action = %action,
                token_fp = %fp,
                outcome = "rate-limited-429-token",
                ts = %ts,
                "write-audit"
            );
            return rate_limited_response("rate limit exceeded");
        }
    }

    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Method;

    fn make_state() -> RateLimitState {
        RateLimitState::new()
    }

    #[test]
    fn rightmost_xff_is_used() {
        // Attacker sends a spoofed leftmost entry; Caddy appends the real IP.
        // We must rate-limit on the real (rightmost) IP, not the spoofed one.
        let spoofed = "1.2.3.4";
        let real = "9.9.9.9";
        let xff = format!("{}, {}", spoofed, real);
        let req = axum::http::Request::builder()
            .method(Method::POST)
            .header("x-forwarded-for", &xff)
            .body(axum::body::Body::empty())
            .unwrap();
        assert_eq!(client_ip(&req), real, "must use rightmost XFF entry");
    }

    #[test]
    fn single_xff_entry_accepted() {
        let req = axum::http::Request::builder()
            .method(Method::POST)
            .header("x-forwarded-for", "5.6.7.8")
            .body(axum::body::Body::empty())
            .unwrap();
        assert_eq!(client_ip(&req), "5.6.7.8");
    }

    #[test]
    fn eviction_bounds_ip_map() {
        let rl = RateLimitState {
            ip_max_buckets: 3,
            ..RateLimitState::new()
        };
        // Fill to cap with keys whose windows are already expired (reset_at in the past).
        {
            let mut map = rl.ip_buckets.lock().unwrap();
            for i in 0..3u32 {
                map.insert(
                    format!("10.0.0.{}", i),
                    Window {
                        count: 1,
                        reset_at: Instant::now() - Duration::from_secs(1),
                    },
                );
            }
        }
        // A new key triggers eviction; expired entries are swept, map stays bounded.
        assert!(rl.check(&rl.ip_buckets, "192.168.1.1", 30, 3));
        let map = rl.ip_buckets.lock().unwrap();
        assert!(map.len() <= 3, "map must not grow past cap after eviction");
    }

    #[test]
    fn ip_limit_enforced() {
        let rl = make_state();
        for _ in 0..30 {
            assert!(rl.check(&rl.ip_buckets, "1.2.3.4", 30, 10_000));
        }
        assert!(!rl.check(&rl.ip_buckets, "1.2.3.4", 30, 10_000), "31st request must be rejected");
    }
}
