//! Pandora Runtime API — local HTTP server exposing ExecutionController.
//!
//! Start with: pandora serve
//! Endpoints:
//!   GET  /health
//!   POST /execute       — run a task or plan
//!   GET  /sessions       — list sessions
//!   GET  /sessions/{id}  — get session detail
//!   GET  /explain/{id}   — explain a session
//!   GET  /graph/{id}     — render provenance graph
//!   GET  /artifacts/{id} — list artifacts
//!   GET  /providers      — provider health
//!
//! This is the foundation for MCP, IDE integration, and fleet workers.

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use pandora_types::execution_plan::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Shared runtime state.
pub struct ApiState {
    pub runtime: Arc<Mutex<pandora_orchestrator::PandoraRuntime>>,
    pub sessions_dir: std::path::PathBuf,
}

// ── Request/Response types ──

#[derive(Debug, Deserialize)]
struct ExecuteRequest { task: String, #[serde(default)] domain: String, #[serde(default)] strategy: String, #[serde(default)] evaluator: String }

#[derive(Debug, Serialize)]
struct ExecuteResponse { session_id: String, status: String, output: String, duration_ms: u64, provider: String }

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct SessionInfo { id: String, prompt: String, status: String, timeline_count: usize }

// ── Handlers ──

async fn health() -> impl IntoResponse { Json(serde_json::json!({"status":"ok","runtime":"pandora-api","version":"0.1.0"})) }

async fn execute(State(state): State<Arc<ApiState>>, Json(req): Json<ExecuteRequest>) -> impl IntoResponse {
    let strategy = match req.strategy.as_str() { "closed" => ControlStrategy::Closed, "human" => ControlStrategy::Human, _ => ControlStrategy::SingleShot };
    let eval = match req.evaluator.as_str() { "rust-tests" => EvaluatorKind::RustTests, _ => EvaluatorKind::None };
    let _plan = ExecutionPlan {
        instruction: req.task.clone(), control_strategy: strategy, evaluator: eval,
        stop_conditions: vec![StopCondition::GoalMet], ..Default::default()
    };
    let domain = if req.domain.is_empty() { "default".to_string() } else { req.domain.clone() };
    let mut runtime = state.runtime.lock().await;
    match runtime.run(&req.task, &domain).await {
        Ok(r) => Json(ExecuteResponse { session_id: r.execution_id, status: if r.success {"completed".into()} else {"failed".into()}, output: r.output.chars().take(2000).collect(), duration_ms: r.duration_ms as u64, provider: r.provider }),
        Err(e) => Json(ExecuteResponse { session_id: String::new(), status: "error".into(), output: e.to_string(), duration_ms: 0, provider: String::new() }),
    }
}

async fn sessions(State(state): State<Arc<ApiState>>) -> impl IntoResponse {
    let dir = &state.sessions_dir;
    let mut s = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() { s.push(entry.file_name().to_string_lossy().to_string()); }
    }
    Json(s)
}

async fn session_detail(State(state): State<Arc<ApiState>>, Path(id): Path<String>) -> impl IntoResponse {
    let path = state.sessions_dir.join(format!("{id}.json"));
    match std::fs::read_to_string(&path) {
        Ok(json) => Json(serde_json::json!({"id":id,"data":json})).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error":"not found"}))).into_response(),
    }
}

async fn explain(State(state): State<Arc<ApiState>>, Path(id): Path<String>) -> impl IntoResponse {
    let path = state.sessions_dir.join(format!("{id}.json"));
    match std::fs::read_to_string(&path) {
        Ok(json) => {
            let session: Option<pandora_types::Session> = serde_json::from_str(&json).ok();
            match session {
                Some(s) => Json(serde_json::json!({"id":s.id,"prompt":s.prompt,"status":format!("{:?}",s.status),"timeline":s.timeline.len()})).into_response(),
                None => (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error":"parse error"}))).into_response(),
            }
        }
        Err(_) => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error":"not found"}))).into_response(),
    }
}

async fn providers() -> impl IntoResponse {
    let providers = pandora_types::provider_health::check_ollama();
    Json(serde_json::json!({"providers":[{"name":providers.name,"status":providers.status,"models":providers.model_count,"latency_ms":providers.latency_ms}]}))
}

// ── Server ──

pub async fn serve(addr: &str, sessions_dir: std::path::PathBuf) -> Result<(), anyhow::Error> {
    let runtime = pandora_orchestrator::PandoraRuntime::new();
    let state = Arc::new(ApiState { runtime: Arc::new(Mutex::new(runtime)), sessions_dir });
    let app = Router::new()
        .route("/health", get(health))
        .route("/execute", post(execute))
        .route("/sessions", get(sessions))
        .route("/sessions/{id}", get(session_detail))
        .route("/explain/{id}", get(explain))
        .route("/providers", get(providers))
        .with_state(state);
    println!("[API] Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
pub mod mcp;
