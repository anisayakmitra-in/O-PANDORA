//! Pandora Runtime API ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â local HTTP server exposing ExecutionController.
//!
//! Start with: pandora serve
//! Endpoints:
//!   GET  /health
//!   POST /execute       ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â run a task or plan
//!   GET  /sessions       ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â list sessions
//!   GET  /sessions/{id}  ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â get session detail
//!   GET  /explain/{id}   ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â explain a session
//!   GET  /graph/{id}     ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â render provenance graph
//!   GET  /artifacts/{id} ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â list artifacts
//!   GET  /providers      ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â provider health
//!
//! This is the foundation for MCP, IDE integration, and fleet workers.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub mod client;
pub mod delivery;
pub mod protocol;
use std::sync::Arc;

/// Check Bearer token. If PANDORA_API_TOKEN is set, auth is mandatory.
/// If not set, the API runs in insecure mode only when explicitly enabled.
fn require_auth(headers: &axum::http::HeaderMap) -> bool {
    if env_flag_enabled("PANDORA_INSECURE") {
        return true;
    }

    let expected = match std::env::var("PANDORA_API_TOKEN") {
        Ok(token) if !token.is_empty() => token,
        _ => return false,
    };
    let auth = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok());
    auth.and_then(|value| value.strip_prefix("Bearer "))
        .is_some_and(|token| constant_time_compare(token, &expected))
}

fn execution_timeout() -> std::time::Duration {
    let seconds = std::env::var("PANDORA_EXECUTION_TIMEOUT_SECONDS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| (1..=86_400).contains(value))
        .unwrap_or(1_800);
    std::time::Duration::from_secs(seconds)
}

fn next_execution_id() -> String {
    pandora_types::runtime_context::ExecutionId::new().0
}

fn env_flag_enabled(name: &str) -> bool {
    matches!(
        std::env::var(name).ok().as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("YES")
    )
}

/// Constant-time string comparison to prevent timing attacks.
fn constant_time_compare(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let max_len = a_bytes.len().max(b_bytes.len());
    let mut diff: u8 = 0;
    for i in 0..max_len {
        let a_byte = a_bytes.get(i).copied().unwrap_or(0);
        let b_byte = b_bytes.get(i).copied().unwrap_or(0);
        diff |= a_byte ^ b_byte;
        diff |= (a_bytes.len() as u8) ^ (b_bytes.len() as u8);
    }
    diff == 0
}
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

struct AuthState {
    paired_tokens: Mutex<HashMap<String, Instant>>,
    pair_attempts: Mutex<PairRate>,
}

struct PairRate {
    window_started: Instant,
    attempts: u8,
}

impl AuthState {
    fn new() -> Self {
        Self {
            paired_tokens: Mutex::new(HashMap::new()),
            pair_attempts: Mutex::new(PairRate {
                window_started: Instant::now(),
                attempts: 0,
            }),
        }
    }

    async fn allow_pair_attempt(&self) -> bool {
        let mut rate = self.pair_attempts.lock().await;
        let now = Instant::now();
        if now.duration_since(rate.window_started) >= Duration::from_secs(60) {
            rate.window_started = now;
            rate.attempts = 0;
        }
        if rate.attempts >= 5 {
            return false;
        }
        rate.attempts += 1;
        true
    }

    async fn issue(&self) -> String {
        use rand::{distributions::Alphanumeric, Rng};
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(48)
            .map(char::from)
            .collect();
        self.paired_tokens
            .lock()
            .await
            .insert(token.clone(), Instant::now() + Duration::from_secs(3600));
        token
    }

    async fn is_valid(&self, token: &str) -> bool {
        let mut paired = self.paired_tokens.lock().await;
        paired.retain(|_, expires_at| *expires_at > Instant::now());
        paired.contains_key(token)
    }
    async fn revoke_if_valid(&self, token: &str) -> bool {
        let mut paired = self.paired_tokens.lock().await;
        paired.retain(|_, expires_at| *expires_at > Instant::now());
        paired.remove(token).is_some()
    }

    async fn revoke(&self, token: &str) -> bool {
        self.paired_tokens.lock().await.remove(token).is_some()
    }
}

async fn require_auth_state(headers: &axum::http::HeaderMap, auth: &AuthState) -> bool {
    if require_auth(headers) {
        return true;
    }
    let token = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));
    let Some(token) = token else {
        return false;
    };
    auth.is_valid(token).await
}
/// Shared runtime state.
pub struct ApiState {
    pub runtime: Arc<Mutex<pandora_orchestrator::PandoraRuntime>>,
    pub sessions_dir: std::path::PathBuf,

    auth: AuthState,
    delivery: delivery::DeliveryLedger,
}

// ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ Request/Response types ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬

/// Helper: extract auth headers and check, returning 401 if unauthorized.
macro_rules! auth_check {
    ($headers:expr, $state:expr) => {
        if !require_auth_state(&$headers, &$state.auth).await {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };
}

// ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ Handlers ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬

async fn pair(
    State(state): State<Arc<ApiState>>,
    Json(request): Json<protocol::PairRequest>,
) -> axum::response::Response {
    let expected = match std::env::var("PANDORA_PAIRING_CODE") {
        Ok(code) if !code.is_empty() => code,
        _ => return StatusCode::NOT_FOUND.into_response(),
    };
    if !state.auth.allow_pair_attempt().await {
        return StatusCode::TOO_MANY_REQUESTS.into_response();
    }
    if !constant_time_compare(&request.code, &expected) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let token = state.auth.issue().await;
    Json(protocol::PairResponse {
        api_version: protocol::API_VERSION.to_string(),
        token,
        expires_in_seconds: 3600,
    })
    .into_response()
}

async fn revoke(
    State(state): State<Arc<ApiState>>,
    headers: axum::http::HeaderMap,
    Json(request): Json<protocol::RevokeRequest>,
) -> axum::response::Response {
    let is_primary_credential = require_auth(&headers);
    let presented_token = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));
    if is_primary_credential {
        return if state.auth.revoke(&request.token).await {
            StatusCode::NO_CONTENT.into_response()
        } else {
            StatusCode::NOT_FOUND.into_response()
        };
    }
    if presented_token != Some(request.token.as_str()) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    if state.auth.revoke_if_valid(&request.token).await {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
async fn health() -> impl IntoResponse {
    Json(protocol::HealthResponse {
        api_version: protocol::API_VERSION.to_string(),
        status: "ok".to_string(),
        runtime: "pandora-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn node_info(
    State(state): State<Arc<ApiState>>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    auth_check!(headers, state);
    let node_id = std::env::var("PANDORA_NODE_ID").unwrap_or_else(|_| "local".to_string());
    let name = std::env::var("PANDORA_NODE_NAME").unwrap_or_else(|_| node_id.clone());
    Json(protocol::NodeInfo {
        api_version: protocol::API_VERSION.to_string(),
        node_id,
        name,
        version: env!("CARGO_PKG_VERSION").to_string(),
        platform: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        capabilities: vec![
            "execute".into(),
            "sessions".into(),
            "providers".into(),
            "events.websocket".into(),
        ],
        auth_required: std::env::var("PANDORA_API_TOKEN")
            .ok()
            .is_some_and(|token| !token.is_empty()),
    })
    .into_response()
}
fn configure_runtime(
    runtime: &mut pandora_orchestrator::PandoraRuntime,
    request: &protocol::ExecuteRequest,
) -> Result<(), String> {
    if let Some(name) = request.profile.as_deref() {
        let profile =
            pandora_types::profile::load_profile(name).map_err(|error| error.to_string())?;
        profile.apply_to(&mut runtime.plan);
    }
    runtime.plan.instruction = request.task.clone();
    if !request.strategy.is_empty() {
        runtime.plan.control_strategy = match request.strategy.as_str() {
            "closed" => pandora_types::execution_plan::ControlStrategy::Closed,
            "open" => pandora_types::execution_plan::ControlStrategy::Open,
            "human" => pandora_types::execution_plan::ControlStrategy::Human,
            "autonomous" => pandora_types::execution_plan::ControlStrategy::Autonomous,
            _ => pandora_types::execution_plan::ControlStrategy::SingleShot,
        };
    }
    if !request.evaluator.is_empty() {
        runtime.plan.evaluator = match request.evaluator.as_str() {
            "rust-tests" => pandora_types::execution_plan::EvaluatorKind::RustTests,
            "python-tests" => pandora_types::execution_plan::EvaluatorKind::PythonTests,
            value => pandora_types::execution_plan::EvaluatorKind::Custom(value.to_string()),
        };
    }
    Ok(())
}

async fn execute_request(
    state: &ApiState,
    request: &protocol::ExecuteRequest,
) -> protocol::ExecuteResponse {
    let domain = if request.domain.is_empty() {
        "default"
    } else {
        &request.domain
    };
    let mut runtime = state.runtime.lock().await;
    match configure_runtime(&mut runtime, request) {
        Err(error) => protocol::ExecuteResponse {
            api_version: protocol::API_VERSION.to_string(),
            session_id: String::new(),
            status: "error".into(),
            output: error,
            duration_ms: 0,
            provider: String::new(),
        },
        Ok(()) => {
            match tokio::time::timeout(execution_timeout(), runtime.run(&request.task, domain))
                .await
            {
                Ok(Ok(result)) => protocol::ExecuteResponse {
                    api_version: protocol::API_VERSION.to_string(),
                    session_id: result.execution_id,
                    status: if result.success {
                        "completed"
                    } else {
                        "failed"
                    }
                    .into(),
                    output: result.output.chars().take(2000).collect(),
                    duration_ms: result.duration_ms as u64,
                    provider: result.provider,
                },
                Ok(Err(error)) => protocol::ExecuteResponse {
                    api_version: protocol::API_VERSION.to_string(),
                    session_id: String::new(),
                    status: "error".into(),
                    output: error.to_string(),
                    duration_ms: 0,
                    provider: String::new(),
                },
                Err(_) => protocol::ExecuteResponse {
                    api_version: protocol::API_VERSION.to_string(),
                    session_id: String::new(),
                    status: "timeout".into(),
                    output: "execution exceeded the configured timeout".into(),
                    duration_ms: execution_timeout().as_millis() as u64,
                    provider: String::new(),
                },
            }
        }
    }
}

async fn execute_request_with_stream(
    state: Arc<ApiState>,
    request: protocol::ExecuteRequest,
    execution_id: String,
    stream_sender: tokio::sync::mpsc::Sender<pandora_types::provider::StreamChunk>,
) -> protocol::ExecuteResponse {
    let domain = if request.domain.is_empty() {
        "default".to_string()
    } else {
        request.domain.clone()
    };
    let task = request.task.clone();
    let runtime = Arc::clone(&state.runtime);
    let response_execution_id = execution_id.clone();
    let runtime_execution_id = execution_id.clone();
    let handle = tokio::runtime::Handle::current();
    let execution = tokio::task::spawn_blocking(move || {
        let mut runtime = runtime.blocking_lock();
        match configure_runtime(&mut runtime, &request) {
            Err(error) => protocol::ExecuteResponse {
                api_version: protocol::API_VERSION.to_string(),
                session_id: runtime_execution_id,
                status: "error".into(),
                output: error,
                duration_ms: 0,
                provider: String::new(),
            },
            Ok(()) => {
                let stream = Box::new(move |chunk| {
                    let _ = stream_sender.try_send(chunk);
                }) as pandora_types::provider::StreamCallback;
                match handle.block_on(runtime.run_with_execution_id_and_stream(
                    runtime_execution_id.clone(),
                    &task,
                    &domain,
                    Some(&stream),
                )) {
                    Ok(result) => protocol::ExecuteResponse {
                        api_version: protocol::API_VERSION.to_string(),
                        session_id: result.execution_id,
                        status: if result.success {
                            "completed"
                        } else {
                            "failed"
                        }
                        .into(),
                        output: result.output.chars().take(2000).collect(),
                        duration_ms: result.duration_ms as u64,
                        provider: result.provider,
                    },
                    Err(error) => protocol::ExecuteResponse {
                        api_version: protocol::API_VERSION.to_string(),
                        session_id: runtime_execution_id,
                        status: "error".into(),
                        output: error.to_string(),
                        duration_ms: 0,
                        provider: String::new(),
                    },
                }
            }
        }
    });

    match tokio::time::timeout(execution_timeout(), execution).await {
        Ok(Ok(response)) => response,
        Ok(Err(error)) => protocol::ExecuteResponse {
            api_version: protocol::API_VERSION.to_string(),
            session_id: response_execution_id,
            status: "error".into(),
            output: format!("execution worker failed: {error}"),
            duration_ms: 0,
            provider: String::new(),
        },
        Err(_) => protocol::ExecuteResponse {
            api_version: protocol::API_VERSION.to_string(),
            session_id: response_execution_id,
            status: "timeout".into(),
            output: "execution exceeded the configured timeout".into(),
            duration_ms: execution_timeout().as_millis() as u64,
            provider: String::new(),
        },
    }
}
fn websocket_event_from_response(response: protocol::ExecuteResponse) -> protocol::RuntimeEvent {
    let protocol::ExecuteResponse {
        status,
        session_id,
        output,
        ..
    } = response;
    match status.as_str() {
        "completed" => protocol::RuntimeEvent::Completed {
            session_id,
            success: true,
        },
        "failed" => protocol::RuntimeEvent::Completed {
            session_id,
            success: false,
        },
        _ => protocol::RuntimeEvent::Failed {
            session_id,
            error: output,
        },
    }
}

async fn execute(
    State(state): State<Arc<ApiState>>,
    headers: axum::http::HeaderMap,
    Json(req): Json<protocol::ExecuteRequest>,
) -> axum::response::Response {
    if !require_auth_state(&headers, &state.auth).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let response = execute_request(&state, &req).await;
    if let Ok(payload) = serde_json::to_string(&response) {
        if let Ok(delivery_id) = state
            .delivery
            .enqueue("http", &response.session_id, payload)
        {
            let _ = state.delivery.mark_delivered(&delivery_id);
        }
    }
    Json(response).into_response()
}

async fn sessions(
    State(state): State<Arc<ApiState>>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    auth_check!(headers, state);
    let dir = &state.sessions_dir;
    let mut s = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            s.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    Json(s).into_response()
}

async fn session_detail(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    auth_check!(headers, state);
    if !is_safe_session_id(&id) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error":"invalid session id"})),
        )
            .into_response();
    }
    let path = state.sessions_dir.join(format!("{id}.json"));
    match std::fs::read_to_string(&path) {
        Ok(json) => Json(serde_json::json!({"id":id,"data":json})).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error":"not found"})),
        )
            .into_response(),
    }
}

fn is_safe_session_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

async fn explain(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    auth_check!(headers, state);
    if !is_safe_session_id(&id) {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error":"invalid session id"})),
        )
            .into_response();
    }
    let path = state.sessions_dir.join(format!("{id}.json"));
    match std::fs::read_to_string(&path) {
        Ok(json) => {
            let session: Option<pandora_types::Session> = serde_json::from_str(&json).ok();
            match session {
                Some(s) => Json(serde_json::json!({"id":s.id,"prompt":s.prompt,"status":format!("{:?}",s.status),"timeline":s.timeline.len()})).into_response(),
                None => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error":"parse error"}))).into_response(),
            }
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error":"not found"})),
        )
            .into_response(),
    }
}

async fn websocket(
    ws: axum::extract::ws::WebSocketUpgrade,
    State(state): State<Arc<ApiState>>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    if !require_auth_state(&headers, &state.auth).await {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    ws.max_message_size(1_048_576)
        .max_frame_size(1_048_576)
        .on_upgrade(move |socket| websocket_session(socket, state))
        .into_response()
}

async fn send_websocket_event(
    socket: &mut axum::extract::ws::WebSocket,
    state: &ApiState,
    sequence: &mut u64,
    event: protocol::RuntimeEvent,
) -> bool {
    use axum::extract::ws::Message;

    *sequence += 1;
    let envelope = protocol::EventEnvelope {
        api_version: protocol::API_VERSION.to_string(),
        sequence: *sequence,
        event,
    };
    let payload = match serde_json::to_string(&envelope) {
        Ok(payload) => payload,
        Err(_) => return false,
    };
    let session_id = match &envelope.event {
        protocol::RuntimeEvent::Started { session_id, .. }
        | protocol::RuntimeEvent::Output { session_id, .. }
        | protocol::RuntimeEvent::ToolCall { session_id, .. }
        | protocol::RuntimeEvent::ApprovalRequired { session_id, .. }
        | protocol::RuntimeEvent::Completed { session_id, .. }
        | protocol::RuntimeEvent::Failed { session_id, .. } => session_id,
    };
    let delivery_id = state
        .delivery
        .enqueue("websocket", session_id, payload.clone())
        .ok();
    if socket.send(Message::Text(payload)).await.is_err() {
        return false;
    }
    if let Some(delivery_id) = delivery_id {
        let _ = state.delivery.mark_delivered(&delivery_id);
    }
    true
}

async fn send_stream_chunk(
    socket: &mut axum::extract::ws::WebSocket,
    state: &ApiState,
    sequence: &mut u64,
    session_id: &str,
    chunk: pandora_types::provider::StreamChunk,
) -> bool {
    for tool_call in chunk.tool_calls {
        if !send_websocket_event(
            socket,
            state,
            sequence,
            protocol::RuntimeEvent::ToolCall {
                session_id: session_id.to_string(),
                tool: tool_call.name,
            },
        )
        .await
        {
            return false;
        }
    }
    if chunk.text.is_empty() {
        return true;
    }
    send_websocket_event(
        socket,
        state,
        sequence,
        protocol::RuntimeEvent::Output {
            session_id: session_id.to_string(),
            chunk: chunk.text,
        },
    )
    .await
}

async fn websocket_session(mut socket: axum::extract::ws::WebSocket, state: Arc<ApiState>) {
    use axum::extract::ws::Message;

    let mut sequence = 0_u64;
    while let Some(Ok(Message::Text(text))) = socket.recv().await {
        let request: protocol::ExecuteRequest = match serde_json::from_str(&text) {
            Ok(request) => request,
            Err(error) => {
                sequence += 1;
                let event = protocol::EventEnvelope {
                    api_version: protocol::API_VERSION.to_string(),
                    sequence,
                    event: protocol::RuntimeEvent::Failed {
                        session_id: String::new(),
                        error: format!("invalid request: {error}"),
                    },
                };
                let _ = socket
                    .send(Message::Text(serde_json::to_string(&event).unwrap()))
                    .await;
                continue;
            }
        };

        let execution_id = next_execution_id();
        if !send_websocket_event(
            &mut socket,
            &state,
            &mut sequence,
            protocol::RuntimeEvent::Started {
                session_id: execution_id.clone(),
                task: request.task.clone(),
            },
        )
        .await
        {
            break;
        }

        let (stream_sender, mut stream_receiver) = tokio::sync::mpsc::channel(256);
        let execution = execute_request_with_stream(
            Arc::clone(&state),
            request,
            execution_id.clone(),
            stream_sender,
        );
        tokio::pin!(execution);
        let response = loop {
            tokio::select! {
                Some(chunk) = stream_receiver.recv() => {
                    if !send_stream_chunk(
                        &mut socket,
                        &state,
                        &mut sequence,
                        &execution_id,
                        chunk,
                    ).await {
                        return;
                    }
                }
                response = &mut execution => break response,
            }
        };
        while let Ok(chunk) = stream_receiver.try_recv() {
            if !send_stream_chunk(&mut socket, &state, &mut sequence, &execution_id, chunk).await {
                return;
            }
        }
        if !send_websocket_event(
            &mut socket,
            &state,
            &mut sequence,
            websocket_event_from_response(response),
        )
        .await
        {
            break;
        }
    }
}
async fn deliveries(
    State(state): State<Arc<ApiState>>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    auth_check!(headers, state);
    match state.delivery.list() {
        Ok(records) => Json(records).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error.to_string()})),
        )
            .into_response(),
    }
}

async fn providers(
    State(state): State<Arc<ApiState>>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    auth_check!(headers, state);
    let providers = pandora_types::provider_health::check_ollama();
    Json(
        serde_json::json!({"providers":[{"name":providers.name,"status":providers.status,"models":providers.model_count,"latency_ms":providers.latency_ms}]}),
    ).into_response()
}
const DASHBOARD_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Pandora</title>
<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap');
*{margin:0;padding:0;box-sizing:border-box}
:root{
  --glass:rgba(15,15,28,0.75);
  --glass2:rgba(20,20,38,0.6);
  --border:rgba(255,255,255,0.06);
  --text:rgba(255,255,255,0.92);
  --text2:rgba(255,255,255,0.45);
  --accent:#7c3aed;
  --accent2:rgba(124,58,237,0.2);
  --green:#22c55e;
  --red:#ef4444;
  --amber:#eab308;
}
body{
  font-family:'Inter',system-ui,sans-serif;
  background:linear-gradient(135deg,#0a0a16,#0d0d22,#0f0a1e);
  color:var(--text);
  height:100vh;
  display:flex;
  overflow:hidden;
}
.g{background:var(--glass);backdrop-filter:blur(20px) saturate(180%);-webkit-backdrop-filter:blur(20px) saturate(180%);border:1px solid var(--border);border-radius:12px}
.gp{background:var(--glass2);backdrop-filter:blur(14px) saturate(150%);border:1px solid var(--border)}
.gb{transition:all 0.2s}
.gb:hover{background:rgba(255,255,255,0.04);border-color:rgba(255,255,255,0.1)}
::-webkit-scrollbar{width:5px}
::-webkit-scrollbar-track{background:transparent}
::-webkit-scrollbar-thumb{background:rgba(255,255,255,0.08);border-radius:3px}
.sidebar{width:220px;display:flex;flex-direction:column;padding:12px 10px;margin:6px;border-radius:14px;gap:2px}
.sidebar h1{font-size:17px;font-weight:700;color:#a78bfa;margin-bottom:16px;padding:0 8px}
.nav-btn{display:flex;align-items:center;gap:8px;padding:8px 10px;border-radius:8px;border:none;background:transparent;color:var(--text2);cursor:pointer;font-size:12px;font-weight:500;text-align:left;width:100%}
.nav-btn.active,.nav-btn:hover{background:var(--accent2);color:#c4b5fd}
.nav-btn svg{width:16px;height:16px;opacity:0.7}
.sidebar-foot{padding:8px;font-size:10px;color:rgba(255,255,255,0.2);border-top:1px solid var(--border);margin-top:auto}
.main{flex:1;display:flex;flex-direction:column;margin:6px 6px 6px 0}
.titlebar{display:flex;align-items:center;padding:8px 14px;border-radius:14px 14px 0 0;gap:10px;font-size:11px;color:var(--text2)}
.titlebar .dot{width:9px;height:9px;border-radius:50%;display:inline-block}
.chat-area{flex:1;display:flex;flex-direction:column;background:rgba(10,10,20,0.45);overflow:hidden}
.messages{flex:1;overflow-y:auto;padding:16px 20px;display:flex;flex-direction:column;gap:10px}
.msg{max-width:88%;padding:10px 14px;border-radius:10px;font-size:13px;line-height:1.55;white-space:pre-wrap;word-break:break-word}
.msg.user{align-self:flex-end;background:var(--accent2);border:1px solid rgba(124,58,237,0.25);color:var(--text)}
.msg.system{align-self:flex-start;background:rgba(255,255,255,0.03);border:1px solid var(--border);color:rgba(255,255,255,0.85)}
.msg.err{align-self:flex-start;background:rgba(239,68,68,0.12);border:1px solid rgba(239,68,68,0.25);color:#fca5a5}
.msg .time{font-size:10px;color:var(--text2);margin-top:4px}
.input-bar{padding:10px 16px;border-top:1px solid var(--border);display:flex;gap:8px}
.input-bar input{flex:1;padding:9px 14px;background:rgba(255,255,255,0.03);border:1px solid var(--border);border-radius:8px;color:var(--text);font-size:13px;outline:none;font-family:inherit}
.input-bar input:focus{border-color:rgba(124,58,237,0.5)}
.input-bar button{padding:9px 18px;background:var(--accent);border:none;border-radius:8px;color:#fff;cursor:pointer;font-size:13px;font-weight:600;white-space:nowrap}
.input-bar button:disabled{opacity:0.4;cursor:default}
.panel{padding:20px 24px;overflow-y:auto;flex:1}
.panel h2{font-size:14px;font-weight:600;color:var(--text2);margin-bottom:12px;text-transform:uppercase;letter-spacing:0.5px}
.card{background:var(--glass2);border:1px solid var(--border);border-radius:8px;padding:12px 16px;margin-bottom:6px;font-size:13px}
.tag{display:inline-block;padding:4px 10px;border-radius:16px;font-size:11px;background:rgba(255,255,255,0.04);border:1px solid var(--border);margin:2px}
</style>
</head>
<body>

<div class="sidebar g">
  <h1>ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ Pandora</h1>
  <button class="nav-btn active" onclick="ST('chat')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/></svg>Chat</button>
  <button class="nav-btn" onclick="ST('harnesses')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg>Harnesses</button>
  <button class="nav-btn" onclick="ST('genes')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>Genes</button>
  <button class="nav-btn" onclick="ST('providers')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>Providers</button>
  <button class="nav-btn" onclick="ST('sessions')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>Sessions</button>
  <button class="nav-btn" onclick="ST('settings')"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>Settings</button>
  <div class="sidebar-foot">v0.5.0<br><span id="sid">desktop-...</span></div>
</div>

<div class="main">
  <div class="titlebar gp">
    <span class="dot" style="background:var(--green)" id="hd"></span>
    <span style="flex:1" id="ht">checking...</span>
    <span style="color:var(--green)">ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ governed</span>
  </div>

  <div class="chat-area">
    <div id="chat-panel" class="messages">
      <div class="msg system">Pandora is ready. Type a task or /command.</div>
    </div>

    <div id="harness-panel" class="panel" style="display:none"></div>
    <div id="gene-panel" class="panel" style="display:none"></div>
    <div id="provider-panel" class="panel" style="display:none"></div>
    <div id="session-panel" class="panel" style="display:none"></div>
    <div id="settings-panel" class="panel" style="display:none">
      <h2>Settings</h2>
      <div class="card">Run <code>pandora doctor</code> for full diagnostics.</div>
      <div class="card" style="margin-top:8px">
        <div style="color:var(--text2);margin-bottom:4px">Session ID</div>
        <code id="sid2" style="font-size:12px">...</code>
      </div>
    </div>

    <div class="input-bar">
      <input id="inp" placeholder="Type a task or /command..." onkeydown="if(event.key==='Enter')S()" autofocus>
      <button id="btn" onclick="S()">Send</button>
    </div>
  </div>
</div>

<script>
let T=Date.now(),tab='chat';
document.querySelectorAll('.nav-btn').forEach(b=>b.addEventListener('click',function(){document.querySelectorAll('.nav-btn').forEach(x=>x.classList.remove('active'));this.classList.add('active')}));

async function H(){
  try{let r=await fetch('/health');let d=await r.json();document.getElementById('hd').style.background='var(--green)';document.getElementById('ht').textContent='connected ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â· v'+(d.version||'0.2.0');document.getElementById('sid').textContent=(d.session_id||'desktop-...').slice(0,20)}catch(e){document.getElementById('hd').style.background='var(--red)';document.getElementById('ht').textContent='offline'}
}
H();setInterval(H,30000);

function ST(t){
  tab=t;
  ['chat-panel','harness-panel','gene-panel','provider-panel','session-panel','settings-panel'].forEach(id=>document.getElementById(id).style.display='none');
  document.getElementById(t+'-panel').style.display=t==='chat'?'flex':'block';
  document.getElementById('inp').style.display=t==='chat'?'block':'none';
  document.getElementById('btn').style.display=t==='chat'?'inline-block':'none';
  if(t==='harnesses')LH();
  if(t==='genes')LG();
  if(t==='providers')LP();
  if(t==='sessions')LS();
  if(t==='settings'){document.getElementById('sid2').textContent=document.getElementById('sid').textContent}
}

async function S(){
  let inp=document.getElementById('inp'),task=inp.value.trim();
  if(!task)return;
  inp.value='';inp.disabled=true;document.getElementById('btn').disabled=true;
  let c=document.getElementById('chat-panel');
  c.insertAdjacentHTML('beforeend','<div class="msg user">'+E(task)+'</div><div class="msg system" id="ld">ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ Executing...</div>');
  try{
    let r=await fetch('/execute',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({task:task,domain:'general'})});
    let d=await r.json();
    document.getElementById('ld')?.remove();
    if(d.output||d.result)c.insertAdjacentHTML('beforeend','<div class="msg system">'+E(d.output||d.result)+'<div class="time">'+(d.duration_ms||'')+'ms</div></div>');
    else if(d.error)c.insertAdjacentHTML('beforeend','<div class="msg err">'+E(d.error)+'</div>');
    else c.insertAdjacentHTML('beforeend','<div class="msg system">'+E(JSON.stringify(d,null,2))+'</div>');
  }catch(e){
    document.getElementById('ld')?.remove();
    c.insertAdjacentHTML('beforeend','<div class="msg err">Connection error: '+e.message+'</div>');
  }
  inp.disabled=false;document.getElementById('btn').disabled=false;inp.focus();
  c.scrollTop=c.scrollHeight;
}

async function LH(){
  try{let r=await fetch('/harnesses');let d=await r.json();let h='<h2>Installed Harnesses</h2>';if(Array.isArray(d))d.forEach(hh=>h+='<div class="card"><b>'+E(hh.id||hh)+'</b></div>');document.getElementById('harness-panel').innerHTML=h}catch(e){document.getElementById('harness-panel').innerHTML='<div class="msg err">'+e.message+'</div>'}
}
async function LG(){
  try{let r=await fetch('/genes');let d=await r.json();let h='<h2>Installed Genes</h2><div style="display:flex;flex-wrap:wrap;gap:6px">';if(Array.isArray(d))d.forEach(g=>h+='<span class="tag">'+E(g.id||g)+'</span>');h+='</div>';document.getElementById('gene-panel').innerHTML=h}catch(e){document.getElementById('gene-panel').innerHTML='<div class="msg err">'+e.message+'</div>'}
}
async function LP(){
  try{let r=await fetch('/providers');let d=await r.json();let h='<h2>Providers</h2>';let ps=Array.isArray(d)?d:(d.connections||[]);ps.forEach(p=>h+='<div class="card"><b>'+E(p.name||p)+'</b><div style="color:var(--text2);font-size:11px;margin-top:2px">'+E(p.endpoint||p.kind||'')+'</div></div>');document.getElementById('provider-panel').innerHTML=h||'<div class="card">No providers configured.</div>'}catch(e){document.getElementById('provider-panel').innerHTML='<div class="msg err">'+e.message+'</div>'}
}
async function LS(){
  try{let r=await fetch('/sessions');let d=await r.json();let h='<h2>Sessions</h2>';let ss=Array.isArray(d)?d:[];ss.slice(0,30).forEach(s=>h+='<div class="card">'+E(s.id?.slice(0,16)||'?')+'...<div style="color:var(--text2);font-size:11px">'+E(s.prompt||s.task||'')+'</div></div>');document.getElementById('session-panel').innerHTML=h||'<div class="card">No sessions yet.</div>'}catch(e){document.getElementById('session-panel').innerHTML='<div class="msg err">'+e.message+'</div>'}
}
function E(s){let d=document.createElement('div');d.textContent=s;return d.innerHTML}
</script>
</body>
</html>
"##;

async fn dashboard() -> impl IntoResponse {
    axum::response::Html(DASHBOARD_HTML)
}

async fn harnesses_list(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let runtime = state.runtime.lock().await;
    let mut harnesses: Vec<String> = runtime
        .council
        .harnesses
        .all_entries()
        .into_iter()
        .map(|(harness, _)| harness.id().to_string())
        .collect();
    drop(runtime);
    harnesses.sort();
    Json(harnesses)
}

async fn genes_list(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let runtime = state.runtime.lock().await;
    let mut genes: Vec<String> = runtime
        .council
        .all_genes()
        .into_iter()
        .map(|gene| gene.id().to_string())
        .collect();
    drop(runtime);
    genes.sort();
    Json(genes)
}
// ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ Server ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚ÂÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬

pub fn router(sessions_dir: std::path::PathBuf) -> Router {
    let mut runtime = pandora_orchestrator::PandoraRuntime::new();
    pandora_harnesses::register_all(&mut runtime.council);
    let state = Arc::new(ApiState {
        runtime: Arc::new(Mutex::new(runtime)),
        sessions_dir: sessions_dir.clone(),
        auth: AuthState::new(),
        delivery: delivery::DeliveryLedger::new(&sessions_dir),
    });
    Router::new()
        .route("/", get(dashboard))
        .route("/health", get(health))
        .route("/api/v1/health", get(health))
        .route("/api/v1/auth/pair", post(pair))
        .route("/api/v1/auth/revoke", post(revoke))
        .route("/api/v1/node", get(node_info))
        .route("/api/v1/execute", post(execute))
        .route("/api/v1/sessions", get(sessions))
        .route("/api/v1/sessions/:id", get(session_detail))
        .route("/api/v1/explain/:id", get(explain))
        .route("/api/v1/providers", get(providers))
        .route("/api/v1/harnesses", get(harnesses_list))
        .route("/api/v1/genes", get(genes_list))
        .route("/api/v1/ws", get(websocket))
        .route("/api/v1/deliveries", get(deliveries))
        .route("/execute", post(execute))
        .route("/sessions", get(sessions))
        .route("/sessions/:id", get(session_detail))
        .route("/explain/:id", get(explain))
        .route("/providers", get(providers))
        .route("/harnesses", get(harnesses_list))
        .route("/genes", get(genes_list))
        .with_state(state)
}

pub async fn serve_listener(
    listener: tokio::net::TcpListener,
    sessions_dir: std::path::PathBuf,
) -> Result<(), anyhow::Error> {
    let address = listener.local_addr()?;
    println!("[API] Listening on {address}");
    axum::serve(listener, router(sessions_dir)).await?;
    Ok(())
}

pub async fn serve(addr: &str, sessions_dir: std::path::PathBuf) -> Result<(), anyhow::Error> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    serve_listener(listener, sessions_dir).await
}
pub mod mcp;

#[cfg(test)]
mod tests;
