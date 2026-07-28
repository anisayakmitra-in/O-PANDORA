//! Pandora Desktop — Phase C: Coding Workspace
//!
//! Adds: file tree, editor, terminal, Git, diff panels

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

mod workspace;

// Re-use workspace state
use workspace::WorkspaceState;

// ── State ──

use pandora_orchestrator::PandoraRuntime;
use pandora_types::connection_manager::ConnectionRegistry;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

struct DesktopState {
    runtime: Arc<Mutex<PandoraRuntime>>,
    active_session: Arc<Mutex<Option<SessionMeta>>>,
    workspace: Arc<WorkspaceState>,
}

#[derive(Clone, Serialize, Deserialize)]
struct SessionMeta {
    id: String,
    name: String,
    created: String,
    model: String,
    provider: String,
}

#[derive(Clone, Serialize)]
struct StreamEvent {
    #[serde(rename = "type")]
    event_type: String,
    content: String,
    metadata: Option<serde_json::Value>,
}

// ── Session Commands ──

#[tauri::command]
async fn create_session(
    state: State<'_, DesktopState>,
    name: String,
) -> Result<SessionMeta, String> {
    let id = format!("session-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
    let meta = SessionMeta {
        id: id.clone(), name,
        created: chrono::Utc::now().to_rfc3339(),
        model: std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "auto".into()),
        provider: "default".into(),
    };
    *state.active_session.lock().await = Some(meta.clone());
    Ok(meta)
}

#[tauri::command]
async fn list_sessions(state: State<'_, DesktopState>) -> Result<Vec<SessionMeta>, String> {
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("sessions");
    if !dir.exists() { return Ok(vec![]); }
    let mut sessions = vec![];
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
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
        id: session_id,
        name: j["prompt"].as_str().unwrap_or("Resumed").to_string(),
        created: j["created"].as_str().unwrap_or("").to_string(),
        model: j["model"].as_str().unwrap_or("auto").to_string(),
        provider: j["provider"].as_str().unwrap_or("default").to_string(),
    };
    *state.active_session.lock().await = Some(meta.clone());
    Ok(meta)
}

// ── Chat Commands ──

#[tauri::command]
async fn send_message(app: AppHandle, state: State<'_, DesktopState>, message: String) -> Result<String, String> {
    let _ = app.emit("stream-event", StreamEvent {
        event_type: "execution.started".into(),
        content: format!("Executing: {message}"),
        metadata: None,
    });
    let mut runtime = state.runtime.lock().await;
    match runtime.run(&message, "general").await {
        Ok(report) => {
            for d in &report.decision_log.decisions {
                let _ = app.emit("stream-event", StreamEvent {
                    event_type: "gene.executed".into(),
                    content: format!("{}: {}", d.selected_gene.as_deref().unwrap_or("tool"), d.stage),
                    metadata: Some(serde_json::json!({
                        "gene": d.selected_gene, "harness": d.selected_harness,
                        "duration_ms": d.outcome.duration_ms, "success": d.outcome.success,
                    })),
                });
            }
            let _ = app.emit("stream-event", StreamEvent {
                event_type: "execution.completed".into(),
                content: report.output.clone(),
                metadata: Some(serde_json::json!({"duration_ms": report.duration_ms, "provider": report.provider, "execution_id": report.execution_id})),
            });
            Ok(report.output)
        }
        Err(e) => {
            let _ = app.emit("stream-event", StreamEvent {
                event_type: "execution.failed".into(), content: e.to_string(), metadata: None,
            });
            Err(e.to_string())
        }
    }
}

// ── Model Commands ──

#[derive(Serialize)]
struct ModelInfo { name: String, provider: String, endpoint: String, healthy: bool, context_size: u64 }

#[tauri::command]
async fn list_models() -> Result<Vec<ModelInfo>, String> {
    let cr = ConnectionRegistry::load();
    let healthy = cr.healthy();
    let mut models = vec![];
    for conn in &cr.connections {
        let h = healthy.iter().any(|x| x.name == conn.name);
        if !conn.default_model.is_empty() {
            models.push(ModelInfo { name: conn.default_model.clone(), provider: conn.name.clone(), endpoint: conn.endpoint.clone(), healthy: h, context_size: 128000 });
        }
        for m in &conn.models {
            models.push(ModelInfo { name: m.clone(), provider: conn.name.clone(), endpoint: conn.endpoint.clone(), healthy: h, context_size: 128000 });
        }
    }
    if models.is_empty() {
        models.push(ModelInfo { name: std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "auto".into()), provider: "auto".into(), endpoint: String::new(), healthy: false, context_size: 0 });
    }
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

// ── Main ──

fn main() {
    let mut runtime = PandoraRuntime::new();
    pandora_harnesses::register_all(&mut runtime.council);

    let state = DesktopState {
        runtime: Arc::new(Mutex::new(runtime)),
        active_session: Arc::new(Mutex::new(None)),
        workspace: Arc::new(WorkspaceState {
            project_path: Arc::new(Mutex::new(None)),
        }),
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
            // Phase C — Workspace
            workspace::open_workspace,
            workspace::get_file_tree,
            workspace::read_file,
            workspace::write_file,
            workspace::delete_file,
            workspace::spawn_terminal,
            workspace::terminal_exec,
            workspace::git_status,
            workspace::git_diff,
            workspace::git_commit,
            workspace::git_branches,
            workspace::git_checkout,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Pandora Desktop");
}
