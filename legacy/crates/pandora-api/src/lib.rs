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

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use pandora_types::execution_plan::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Check Bearer token. If PANDORA_API_TOKEN is set, auth is mandatory.
/// If not set, the API runs in insecure mode (dev/CI only).
fn require_auth(headers: &axum::http::HeaderMap) -> bool {
    // Check for --insecure-plaintext flag via env
    if std::env::var("PANDORA_INSECURE").is_ok() {
        return true;
    }

    let expected = match std::env::var("PANDORA_API_TOKEN") {
        Ok(t) if !t.is_empty() => t,
        _ => {
            // In dev mode, warn but allow
            if std::env::var("PANDORA_DEV_MODE").is_ok() {
                eprintln!("[SECURITY] PANDORA_API_TOKEN not set — API is unprotected. Set a token or run with --insecure-plaintext.");
                return true;
            }
            // Production: require token
            return false;
        }
    };
    let auth = headers.get("authorization").and_then(|v| v.to_str().ok());
    auth.and_then(|a| a.strip_prefix("Bearer "))
        .is_some_and(|t| constant_time_compare(t, &expected))
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
use tokio::sync::Mutex;

/// Shared runtime state.
pub struct ApiState {
    pub runtime: Arc<Mutex<pandora_orchestrator::PandoraRuntime>>,
    pub sessions_dir: std::path::PathBuf,
}

// ── Request/Response types ──

#[derive(Debug, Deserialize)]
struct ExecuteRequest {
    task: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    strategy: String,
    #[serde(default)]
    evaluator: String,
}

#[derive(Debug, Serialize)]
struct ExecuteResponse {
    session_id: String,
    status: String,
    output: String,
    duration_ms: u64,
    provider: String,
}

#[derive(Debug, Serialize)]
#[expect(dead_code)]
struct SessionInfo {
    id: String,
    prompt: String,
    status: String,
    timeline_count: usize,
}

/// Helper: extract auth headers and check, returning 401 if unauthorized.
macro_rules! auth_check {
    ($headers:expr) => {
        if !require_auth(&$headers) {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };
}

// ── Handlers ──

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({"status":"ok","runtime":"pandora-api","version":"0.2.0"}))
}

async fn execute(
    State(state): State<Arc<ApiState>>,
    headers: axum::http::HeaderMap,
    Json(req): Json<ExecuteRequest>,
) -> axum::response::Response {
    if !require_auth(&headers) {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    let strategy = match req.strategy.as_str() {
        "closed" => ControlStrategy::Closed,
        "human" => ControlStrategy::Human,
        _ => ControlStrategy::SingleShot,
    };
    let eval = match req.evaluator.as_str() {
        "rust-tests" => EvaluatorKind::RustTests,
        _ => EvaluatorKind::None,
    };
    let _plan = ExecutionPlan {
        instruction: req.task.clone(),
        control_strategy: strategy,
        evaluator: eval,
        stop_conditions: vec![StopCondition::GoalMet],
        ..Default::default()
    };
    let domain = if req.domain.is_empty() {
        "default".to_string()
    } else {
        req.domain.clone()
    };
    let mut runtime = state.runtime.lock().await;
    match runtime.run(&req.task, &domain).await {
        Ok(r) => Json(ExecuteResponse {
            session_id: r.execution_id,
            status: if r.success {
                "completed".into()
            } else {
                "failed".into()
            },
            output: r.output.chars().take(2000).collect(),
            duration_ms: r.duration_ms as u64,
            provider: r.provider,
        })
        .into_response(),
        Err(e) => Json(ExecuteResponse {
            session_id: String::new(),
            status: "error".into(),
            output: e.to_string(),
            duration_ms: 0,
            provider: String::new(),
        })
        .into_response(),
    }
}

async fn sessions(
    State(state): State<Arc<ApiState>>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    auth_check!(headers);
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
    auth_check!(headers);
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

async fn explain(
    State(state): State<Arc<ApiState>>,
    Path(id): Path<String>,
    headers: axum::http::HeaderMap,
) -> axum::response::Response {
    auth_check!(headers);
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

async fn providers(headers: axum::http::HeaderMap) -> axum::response::Response {
    auth_check!(headers);
    let providers = pandora_types::provider_health::check_ollama();
    Json(
        serde_json::json!({"providers":[{"name":providers.name,"status":providers.status,"models":providers.model_count,"latency_ms":providers.latency_ms}]}),
    ).into_response()
}
const DASHBOARD_HTML: &str = r##"<!DOCTYPE html>
<html lang="en">
<head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Pandora Dashboard</title>
<style>*{margin:0;padding:0;box-sizing:border-box}body{font-family:system-ui,-apple-system,sans-serif;background:#0a0a0f;color:#e0e0e0;height:100vh;display:flex}.sidebar{width:260px;background:#111118;border-right:1px solid #1a1a2e;display:flex;flex-direction:column;padding:16px}.sidebar h1{font-size:18px;color:#a78bfa;margin-bottom:24px}.sidebar nav{flex:1}.sidebar nav a{display:block;padding:8px 12px;border-radius:6px;color:#888;text-decoration:none;margin-bottom:4px;font-size:13px}.sidebar nav a:hover,.sidebar nav a.active{background:#1a1a2e;color:#e0e0e0}.main{flex:1;display:flex;flex-direction:column}.header{padding:12px 20px;border-bottom:1px solid #1a1a2e;display:flex;align-items:center;gap:12px}.header .dot{width:8px;height:8px;border-radius:50%;background:#22c55e}.header .status{font-size:13px;color:#888}.chat{flex:1;overflow-y:auto;padding:20px;display:flex;flex-direction:column;gap:16px}.msg{max-width:80%;padding:12px 16px;border-radius:12px;font-size:14px;line-height:1.5}.msg.user{align-self:flex-end;background:#1a1a2e}.msg.system{align-self:flex-start;background:#111118;border:1px solid #1a1a2e}.msg.error{background:#3b1414;border-color:#991b1b}.input-bar{padding:16px 20px;border-top:1px solid #1a1a2e;display:flex;gap:8px}.input-bar input{flex:1;padding:10px 16px;background:#111118;border:1px solid #1a1a2e;border-radius:8px;color:#e0e0e0;font-size:14px;outline:none}.input-bar input:focus{border-color:#a78bfa}.input-bar button{padding:10px 20px;background:#7c3aed;color:#fff;border:none;border-radius:8px;cursor:pointer;font-size:14px;font-weight:500}.input-bar button:hover{background:#6d28d9}.input-bar button:disabled{opacity:0.5}.panels{display:flex;gap:16px;padding:20px;flex-wrap:wrap}.card{background:#111118;border:1px solid #1a1a2e;border-radius:8px;padding:16px;flex:1;min-width:200px}.card h3{font-size:13px;color:#888;margin-bottom:8px;text-transform:uppercase;letter-spacing:0.5px}.card .val{font-size:24px;font-weight:600}.card .val.ok{color:#22c55e}.sessions{list-style:none}.sessions li{padding:6px 0;font-size:13px;border-bottom:1px solid #1a1a2e;color:#888}.sessions li:hover{color:#e0e0e0}</style></head>
<body><div class="sidebar"><h1>⚡ Pandora</h1><nav><a href="#" class="active" onclick="ST('chat');return false">Chat</a><a href="#" onclick="ST('dashboard');return false">Dashboard</a><a href="#" onclick="ST('sessions-tab');return false">Sessions</a><a href="#" onclick="ST('settings');return false">Settings</a></nav><div style="margin-top:auto;font-size:11px;color:#555">v0.2.0</div></div>
<div class="main"><div class="header"><div class="dot" id="SD"></div><span class="status" id="ST">checking...</span></div>
<div id="CV" class="chat"><div class="msg system">Ready. Ask me anything.</div></div>
<div id="DV" class="panels" style="display:none"><div class="card"><h3>Provider</h3><div class="val ok" id="PH">--</div></div><div class="card"><h3>Sessions</h3><div class="val" id="SC">--</div></div><div class="card"><h3>Uptime</h3><div class="val ok" id="UT">--</div></div><div class="card"><h3>API</h3><div class="val ok" id="AS">--</div></div></div>
<div id="SV" style="display:none;padding:20px;overflow-y:auto"><h3 style="margin-bottom:12px">Recent Sessions</h3><ul class="sessions" id="SL"></ul></div>
<div id="WV" style="display:none;padding:20px"><h3>Settings</h3><p style="color:#888;font-size:13px">Run <code>pandora doctor</code> for full diagnostics.</p></div>
<div class="input-bar"><input id="CI" placeholder="Type a task..." onkeydown="if(event.key==='Enter')S()"><button onclick="S()">Send</button></div></div>
<script>let T=Date.now();async function H(){try{let r=await fetch('/health');let d=await r.json();document.getElementById('SD').style.background='#22c55e';document.getElementById('ST').textContent='connected';document.getElementById('PH').textContent='OK';document.getElementById('AS').textContent='OK'}catch(e){document.getElementById('SD').style.background='#ef4444';document.getElementById('ST').textContent='offline'}}async function S(){let t=document.getElementById('CI').value.trim();if(!t)return;document.getElementById('CI').value='';let c=document.getElementById('CV');c.insertAdjacentHTML('beforeend','<div class="msg user">'+E(t)+'</div><div class="msg system" id="LD">...</div>');document.getElementById('CI').disabled=true;try{let r=await fetch('/execute',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({task:t,domain:'general'})});let d=await r.json();document.getElementById('LD')?.remove();if(d.output||d.result)c.insertAdjacentHTML('beforeend','<div class="msg system">'+E(d.output||d.result)+'</div>');else if(d.error)c.insertAdjacentHTML('beforeend','<div class="msg error">'+E(d.error)+'</div>');else c.insertAdjacentHTML('beforeend','<div class="msg system">'+E(JSON.stringify(d,null,2))+'</div>')}catch(e){document.getElementById('LD')?.remove();c.insertAdjacentHTML('beforeend','<div class="msg error">Error: '+e.message+'</div>')}document.getElementById('CI').disabled=false;document.getElementById('CI').focus();c.scrollTop=c.scrollHeight}async function LS(){try{let r=await fetch('/sessions');let ss=await r.json();let l=document.getElementById('SL');l.innerHTML=ss.slice(0,20).map(s=>'<li>'+s.id?.slice(0,12)+'... '+E(s.prompt||s.task||'')+'</li>').join('');document.getElementById('SC').textContent=ss.length}catch(e){document.getElementById('SC').textContent='--'}}function ST(t){document.getElementById('CV').style.display=t==='chat'?'flex':'none';document.getElementById('DV').style.display=t==='dashboard'?'flex':'none';document.getElementById('SV').style.display=t==='sessions-tab'?'block':'none';document.getElementById('WV').style.display=t==='settings'?'block':'none';if(t==='sessions-tab')LS()}function E(s){let d=document.createElement('div');d.textContent=s;return d.innerHTML}setInterval(()=>{let u=Math.floor((Date.now()-T)/1000);document.getElementById('UT').textContent=Math.floor(u/60)+'m '+u%60+'s'},1000);H();setInterval(H,30000);LS();</script></body></html>"##;

async fn dashboard() -> impl IntoResponse {
    axum::response::Html(DASHBOARD_HTML)
}


// ── Server ──

pub async fn serve(addr: &str, sessions_dir: std::path::PathBuf) -> Result<(), anyhow::Error> {
    let runtime = pandora_orchestrator::PandoraRuntime::new();
    let state = Arc::new(ApiState {
        runtime: Arc::new(Mutex::new(runtime)),
        sessions_dir,
    });
    let app = Router::new()
        .route("/", get(dashboard))
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

#[cfg(test)]
mod tests;
