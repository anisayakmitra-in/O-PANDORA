//! Pandora Desktop — Phases B-D: Core Agent + Workspace + Governance
//!
//! Architecture: One PandoraRuntime, shared via Arc<Mutex<>>, exposed through Tauri IPC.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use pandora_orchestrator::PandoraRuntime;
use pandora_types::connection_manager::ConnectionRegistry;

mod ecosystem;

// ── Shared State ──

struct DesktopState {
    runtime: Arc<Mutex<PandoraRuntime>>,
    active_session: Arc<Mutex<Option<SessionMeta>>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SessionMeta {
    id: String, name: String, created: String, model: String, provider: String,
}

#[derive(Clone, Serialize)]
struct StreamEvent {
    #[serde(rename = "type")] event_type: String,
    content: String, metadata: Option<serde_json::Value>,
}

// ── Phase B: Sessions ──

#[tauri::command]
async fn create_session(state: State<'_, DesktopState>, name: String) -> Result<SessionMeta, String> {
    let id = format!("session-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
    let meta = SessionMeta { id: id.clone(), name,
        created: chrono::Utc::now().to_rfc3339(),
        model: std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "auto".into()),
        provider: "default".into() };
    *state.active_session.lock().await = Some(meta.clone());
    Ok(meta)
}

#[tauri::command]
async fn list_sessions(_state: State<'_, DesktopState>) -> Result<Vec<SessionMeta>, String> {
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("sessions");
    if !dir.exists() { return Ok(vec![]); }
    let mut sessions = vec![];
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                if let Ok(c) = std::fs::read_to_string(&path) {
                    if let Ok(j) = serde_json::from_str::<serde_json::Value>(&c) {
                        sessions.push(SessionMeta {
                            id: path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                            name: j["prompt"].as_str().unwrap_or("Untitled").to_string(),
                            created: j["created"].as_str().unwrap_or("").to_string(),
                            model: j["model"].as_str().unwrap_or("auto").to_string(),
                            provider: j["provider"].as_str().unwrap_or("default").to_string(),
                        });
                    }
                }
            }
        }
    }
    sessions.sort_by(|a,b| b.created.cmp(&a.created));
    Ok(sessions)
}

#[tauri::command]
async fn resume_session(state: State<'_, DesktopState>, session_id: String) -> Result<SessionMeta, String> {
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("sessions");
    let path = dir.join(format!("{session_id}.json"));
    let c = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let j: serde_json::Value = serde_json::from_str(&c).map_err(|e| e.to_string())?;
    let meta = SessionMeta {
        id: session_id, name: j["prompt"].as_str().unwrap_or("Resumed").to_string(),
        created: j["created"].as_str().unwrap_or("").to_string(),
        model: j["model"].as_str().unwrap_or("auto").to_string(),
        provider: j["provider"].as_str().unwrap_or("default").to_string(),
    };
    *state.active_session.lock().await = Some(meta.clone());
    Ok(meta)
}

// ── Phase B: Chat ──

#[tauri::command]
async fn send_message(app: AppHandle, state: State<'_, DesktopState>, message: String) -> Result<String, String> {
    let _ = app.emit("stream-event", StreamEvent {
        event_type: "execution.started".into(), content: format!("Executing: {message}"), metadata: None,
    });
    let mut runtime = state.runtime.lock().await;
    match runtime.run(&message, "general").await {
        Ok(report) => {
            // TODO: wire to actual decision log API
            // ExecutionReport has: execution_id, output, duration_ms, provider, model
            let _ = &report;
            // TODO: emit gene execution events when decision log API is wired
            let _ = app.emit("stream-event", StreamEvent {
                event_type: "execution.completed".into(), content: report.output.clone(),
                metadata: Some(serde_json::json!({"duration_ms":report.duration_ms,"provider":report.provider,"execution_id":report.execution_id})),
            });
            Ok(report.output)
        }
        Err(e) => {
            let _ = app.emit("stream-event", StreamEvent { event_type: "execution.failed".into(), content: e.to_string(), metadata: None });
            Err(e.to_string())
        }
    }
}

// ── Phase B: Models ──

#[derive(Serialize)]
struct ModelInfo { name: String, provider: String, endpoint: String, healthy: bool, context_size: u64 }

#[tauri::command]
async fn list_models() -> Result<Vec<ModelInfo>, String> {
    let cr = ConnectionRegistry::load(); let healthy = cr.healthy(); let mut models = vec![];
    for conn in &cr.connections {
        let h = healthy.iter().any(|x| x.name == conn.name);
        if !conn.default_model.is_empty() { models.push(ModelInfo { name: conn.default_model.clone(), provider: conn.name.clone(), endpoint: conn.endpoint.clone(), healthy: h, context_size: 128000 }); }
        for m in &conn.models { models.push(ModelInfo { name: m.clone(), provider: conn.name.clone(), endpoint: conn.endpoint.clone(), healthy: h, context_size: 128000 }); }
    }
    if models.is_empty() { models.push(ModelInfo { name: std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "auto".into()), provider: "auto".into(), endpoint: String::new(), healthy: false, context_size: 0 }); }
    Ok(models)
}

#[tauri::command]
async fn switch_model(state: State<'_, DesktopState>, provider: String, model: String) -> Result<(), String> {
    std::env::set_var("PANDORA_DEFAULT_MODEL", &model);
    if let Some(ref mut s) = *state.active_session.lock().await { s.model = model; s.provider = provider; }
    Ok(())
}

#[derive(Serialize)]
struct HealthStatus { runtime: bool, version: String, active_session: Option<SessionMeta> }

#[tauri::command]
async fn health(state: State<'_, DesktopState>) -> Result<HealthStatus, String> {
    Ok(HealthStatus { runtime: true, version: env!("CARGO_PKG_VERSION").into(), active_session: state.active_session.lock().await.clone() })
}

// ── Phase E: Ecosystem Commands ──

#[tauri::command]
async fn palace_list(kind_filter: Option<String>) -> Result<Vec<ecosystem::PalacePackage>, String> {
    let mut pkgs = ecosystem::seed_packages();
    if let Some(ref kf) = kind_filter {
        pkgs.retain(|p| p.kind.to_lowercase() == kf.to_lowercase());
    }
    Ok(pkgs)
}

#[tauri::command]
async fn palace_install(package_id: String) -> Result<String, String> {
    Ok(format!("Installed: {package_id}"))
}

#[tauri::command]
async fn palace_search(query: String) -> Result<Vec<ecosystem::PalacePackage>, String> {
    let q = query.to_lowercase();
    Ok(ecosystem::seed_packages().into_iter()
        .filter(|p| p.name.to_lowercase().contains(&q) || p.description.to_lowercase().contains(&q))
        .collect())
}

#[tauri::command]
async fn fleet_nodes() -> Result<Vec<ecosystem::FleetNode>, String> {
    Ok(vec![ecosystem::FleetNode {
        id: "local".into(), name: "pandora-desktop".into(),
        platform: std::env::consts::OS.into(), status: "online".into(),
        current_task: None, capabilities: vec!["filesystem".into(), "shell".into()],
        last_seen: chrono::Utc::now().to_rfc3339(),
    }])
}

#[tauri::command]
async fn fleet_run(node_id: String, task: String) -> Result<String, String> {
    Ok(format!("Dispatched {task} to {node_id}"))
}

#[tauri::command]
async fn scheduler_list() -> Result<Vec<ecosystem::ScheduledJob>, String> {
    Ok(vec![
        ecosystem::ScheduledJob { id: "job-1".into(), task: "Daily audit".into(), schedule: "0 9 * * *".into(), status: "active".into(), last_run: None, next_run: Some("tomorrow 09:00".into()), project: "default".into() },
        ecosystem::ScheduledJob { id: "job-2".into(), task: "Weekly review".into(), schedule: "0 9 * * 1".into(), status: "paused".into(), last_run: Some("last Monday".into()), next_run: None, project: "default".into() },
    ])
}

#[tauri::command]
async fn scheduler_add(_task: String, _schedule: String) -> Result<String, String> {
    Ok(format!("job-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()))
}

#[tauri::command]
async fn scheduler_pause(job_id: String) -> Result<String, String> {
    Ok(format!("Paused: {job_id}"))
}

#[tauri::command]
async fn scheduler_resume(job_id: String) -> Result<String, String> {
    Ok(format!("Resumed: {job_id}"))
}


// ── Main ──

fn main() {
    let mut runtime = PandoraRuntime::new();
    pandora_harnesses::register_all(&mut runtime.council);

    let state = DesktopState {
        runtime: Arc::new(Mutex::new(runtime)),
        active_session: Arc::new(Mutex::new(None)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            create_session, list_sessions, resume_session,
            send_message, list_models, switch_model, health,
            palace_list, palace_install, palace_search, fleet_nodes, fleet_run, scheduler_list, scheduler_add, scheduler_pause, scheduler_resume,
        ])
        .run(tauri::generate_context!())
        .expect("Pandora Desktop failed to start");
}
