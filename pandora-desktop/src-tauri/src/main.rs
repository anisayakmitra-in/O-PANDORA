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


// ── Phase F: Advanced Commands ──

#[derive(Serialize)]
struct PipelineNode {
    id: String, label: String, kind: String, status: String,
    children: Vec<String>, metadata: Option<serde_json::Value>,
}

#[tauri::command]
async fn architecture_graph() -> Result<Vec<PipelineNode>, String> {
    Ok(vec![
        PipelineNode { id: "parliament".into(), label: "Parliament".into(), kind: "governance".into(), status: "active".into(),
            children: vec!["shadow-council".into()], metadata: Some(serde_json::json!({"verdicts_today": 12, "policies": 5})) },
        PipelineNode { id: "shadow-council".into(), label: "Shadow Council".into(), kind: "routing".into(), status: "active".into(),
            children: vec!["harness-coding".into(), "harness-general".into()], metadata: Some(serde_json::json!({"capabilities": ["filesystem","shell","network","codegen"]})) },
        PipelineNode { id: "harness-coding".into(), label: "coding Harness".into(), kind: "domain".into(), status: "enabled".into(),
            children: vec!["gene-shell".into(), "gene-fs".into(), "gene-review".into()], metadata: Some(serde_json::json!({"type": "DomainHarness"})) },
        PipelineNode { id: "harness-general".into(), label: "general Harness".into(), kind: "source".into(), status: "enabled".into(),
            children: vec!["gene-search".into(), "gene-summarize".into()], metadata: Some(serde_json::json!({"type": "SourceHarness"})) },
        PipelineNode { id: "gene-shell".into(), label: "shell Gene".into(), kind: "gene".into(), status: "active".into(),
            children: vec!["provider".into()], metadata: Some(serde_json::json!({"permissions": ["shell"], "trust": "verified"})) },
        PipelineNode { id: "gene-fs".into(), label: "filesystem Gene".into(), kind: "gene".into(), status: "active".into(),
            children: vec!["provider".into()], metadata: Some(serde_json::json!({"permissions": ["filesystem read/write"], "trust": "verified"})) },
        PipelineNode { id: "gene-review".into(), label: "code-review Gene".into(), kind: "gene".into(), status: "active".into(),
            children: vec!["provider".into()], metadata: Some(serde_json::json!({"permissions": ["filesystem read"], "trust": "verified"})) },
        PipelineNode { id: "gene-search".into(), label: "web-search Gene".into(), kind: "gene".into(), status: "active".into(),
            children: vec!["provider".into()], metadata: Some(serde_json::json!({"permissions": ["network"], "trust": "verified"})) },
        PipelineNode { id: "gene-summarize".into(), label: "summarize Gene".into(), kind: "gene".into(), status: "active".into(),
            children: vec!["provider".into()], metadata: Some(serde_json::json!({"permissions": [], "trust": "verified"})) },
        PipelineNode { id: "provider".into(), label: "Provider".into(), kind: "connection".into(), status: "connected".into(),
            children: vec!["memory".into(), "telemetry".into()], metadata: Some(serde_json::json!({"model": "auto", "latency_ms": 45})) },
        PipelineNode { id: "memory".into(), label: "Memory".into(), kind: "storage".into(), status: "active".into(),
            children: vec![], metadata: Some(serde_json::json!({"entries": 42, "size_kb": 128})) },
        PipelineNode { id: "telemetry".into(), label: "Telemetry".into(), kind: "observability".into(), status: "active".into(),
            children: vec![], metadata: Some(serde_json::json!({"executions_today": 7})) },
    ])
}

// ── Multi-Agent View ──

#[derive(Serialize)]
struct WorkerStatus {
    id: String, task: String, harness: String, status: String,
    progress: u8, elapsed_ms: u64, model: String,
}

#[tauri::command]
async fn multi_agent_status() -> Result<Vec<WorkerStatus>, String> {
    Ok(vec![
        WorkerStatus { id: "main-1".into(), task: "Fix parser bug".into(), harness: "coding".into(), status: "completed".into(), progress: 100, elapsed_ms: 2340, model: "auto".into() },
        WorkerStatus { id: "sub-1".into(), task: "Read parser source".into(), harness: "general".into(), status: "completed".into(), progress: 100, elapsed_ms: 120, model: "auto".into() },
        WorkerStatus { id: "sub-2".into(), task: "Run failing test".into(), harness: "coding".into(), status: "completed".into(), progress: 100, elapsed_ms: 340, model: "auto".into() },
        WorkerStatus { id: "sub-3".into(), task: "Generate fix".into(), harness: "coding".into(), status: "running".into(), progress: 72, elapsed_ms: 890, model: "auto".into() },
        WorkerStatus { id: "sub-4".into(), task: "Write test".into(), harness: "coding".into(), status: "waiting".into(), progress: 0, elapsed_ms: 0, model: "auto".into() },
    ])
}

// ── Notifications ──

#[derive(Serialize)]
struct NotificationPrefs {
    task_complete: bool, approval_required: bool, task_failed: bool,
    fleet_disconnect: bool, package_update: bool,
}

#[tauri::command]
async fn notification_prefs() -> Result<NotificationPrefs, String> {
    Ok(NotificationPrefs {
        task_complete: true, approval_required: true, task_failed: true,
        fleet_disconnect: false, package_update: true,
    })
}

#[tauri::command]
async fn send_test_notification(message: String) -> Result<String, String> {
    Ok(format!("Notification sent: {message}"))
}

// ── Updates ──

#[derive(Serialize)]
struct UpdateStatus {
    current_version: String, latest_version: String,
    update_available: bool, release_notes: String, download_url: String,
}

#[tauri::command]
async fn check_updates() -> Result<UpdateStatus, String> {
    Ok(UpdateStatus {
        current_version: env!("CARGO_PKG_VERSION").into(),
        latest_version: "0.5.1".into(),
        update_available: false,
        release_notes: "Phase F: Architecture graph, multi-agent, notifications".into(),
        download_url: "https://github.com/anisayakmitra-in/O-PANDORA/releases".into(),
    })
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
            architecture_graph, multi_agent_status, notification_prefs, send_test_notification, check_updates,
        ])
        .run(tauri::generate_context!())
        .expect("Pandora Desktop failed to start");
}
