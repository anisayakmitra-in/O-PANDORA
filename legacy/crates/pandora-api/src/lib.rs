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
  <h1>⚡ Pandora</h1>
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
    <span style="color:var(--green)">⚡ governed</span>
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
  try{let r=await fetch('/health');let d=await r.json();document.getElementById('hd').style.background='var(--green)';document.getElementById('ht').textContent='connected · v'+(d.version||'0.2.0');document.getElementById('sid').textContent=(d.session_id||'desktop-...').slice(0,20)}catch(e){document.getElementById('hd').style.background='var(--red)';document.getElementById('ht').textContent='offline'}
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
  c.insertAdjacentHTML('beforeend','<div class="msg user">'+E(task)+'</div><div class="msg system" id="ld">• Executing...</div>');
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

async fn harnesses_list(_state: axum::extract::State<Arc<ApiState>>) -> impl IntoResponse {
    axum::Json(Vec::<String>::new())
}
async fn genes_list(_state: axum::extract::State<Arc<ApiState>>) -> impl IntoResponse {
    axum::Json(Vec::<String>::new())
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
        .route("/harnesses", get(harnesses_list))
        .route("/genes", get(genes_list))
        .with_state(state);
    println!("[API] Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
pub mod mcp;

#[cfg(test)]
mod tests;
