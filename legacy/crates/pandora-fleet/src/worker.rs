//! Fleet Worker Server — HTTP endpoint for remote execution.

use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapability { pub provider: String, pub models: Vec<String>, pub sandbox_level: u8, pub max_concurrency: usize }

impl Default for WorkerCapability {
    fn default() -> Self { Self { provider: "ollama".into(), models: vec![std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "ollama/default".into()).into()], sandbox_level: 0, max_concurrency: 4 } }
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkerHealth { pub status: String, pub uptime_secs: u64, pub tasks_completed: u64 }

#[derive(Debug, Deserialize)]
struct ExecuteRequest { task: String, #[serde(default)] domain: String }

#[derive(Debug, Serialize)]
struct ExecuteResponse { execution_id: String, success: bool, output: String }

pub struct WorkerState {
    pub capability: WorkerCapability,
    pub uptime: std::time::Instant,
    pub completed: std::sync::atomic::AtomicU64,
}

pub async fn serve_worker(addr: &str, cap: WorkerCapability) -> Result<(), anyhow::Error> {
    let state = Arc::new(WorkerState {
        capability: cap, uptime: std::time::Instant::now(),
        completed: std::sync::atomic::AtomicU64::new(0),
    });
    let app = Router::new()
        .route("/health", get(worker_health))
        .route("/capability", get(worker_capability))
        .route("/execute", post(worker_execute))
        .with_state(state);
    println!("[Worker] Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn worker_health(State(s): State<Arc<WorkerState>>) -> Json<WorkerHealth> {
    Json(WorkerHealth { status: "healthy".into(), uptime_secs: s.uptime.elapsed().as_secs(), tasks_completed: s.completed.load(std::sync::atomic::Ordering::Relaxed) })
}

async fn worker_capability(State(s): State<Arc<WorkerState>>) -> Json<WorkerCapability> { Json(s.capability.clone()) }

async fn worker_execute(State(s): State<Arc<WorkerState>>, Json(req): Json<ExecuteRequest>) -> Json<ExecuteResponse> {
    use pandora_orchestrator::PandoraRuntime;
    let mut rt = PandoraRuntime::new();
    let domain = if req.domain.is_empty() { "default" } else { &req.domain };
    match rt.run(&req.task, domain).await {
        Ok(r) => { s.completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed); Json(ExecuteResponse { execution_id: r.execution_id, success: r.success, output: r.output }) }
        Err(e) => Json(ExecuteResponse { execution_id: String::new(), success: false, output: e.to_string() })
    }
}
