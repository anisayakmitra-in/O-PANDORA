//! Pandora Desktop — Tauri Rust Backend
//!
//! Exposes the full Pandora runtime through Tauri IPC commands.
//! One PandoraRuntime. Multiple surfaces (CLI, TUI, Web, Desktop).

use pandora_orchestrator::PandoraRuntime;
use pandora_types::{ParliamentVerdict, connection_manager::ConnectionRegistry};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

// ── State ──

struct DesktopState {
    runtime: Arc<Mutex<PandoraRuntime>>,
    session_id: String,
    project_path: Option<String>,
}

#[derive(Serialize)]
struct ExecutionResult {
    success: bool,
    output: String,
    duration_ms: u64,
    execution_id: String,
    provider: String,
}

#[derive(Serialize)]
struct HealthStatus {
    runtime: bool,
    version: String,
    session_id: String,
    harnesses: usize,
    genes: usize,
    providers: usize,
}

#[derive(Serialize)]
struct ProjectInfo {
    path: String,
    name: String,
    branch: String,
    dirty: bool,
    last_session: Option<String>,
}

#[derive(Serialize)]
struct HarnessInfo {
    id: String,
    kind: String,
    version: String,
    enabled: bool,
}

#[derive(Serialize)]
struct GeneInfo {
    id: String,
    kind: String,
    version: String,
    capabilities: Vec<String>,
}

#[derive(Serialize)]
struct ProviderInfo {
    name: String,
    kind: String,
    endpoint: String,
    model: String,
    healthy: bool,
}

#[derive(Serialize)]
struct SessionInfo {
    id: String,
    prompt: String,
    created: String,
    status: String,
}

#[derive(Serialize)]
struct ContextInfo {
    used_tokens: u64,
    limit_tokens: u64,
    system_prompt: u64,
    conversation: u64,
    memory: u64,
    tool_results: u64,
}

#[derive(Serialize)]
struct ApprovalInfo {
    id: String,
    reason: String,
    tool: String,
    risk: String,
    timestamp: String,
}

// ── Core Commands ──

#[tauri::command]
async fn run_task(
    state: State<'_, DesktopState>,
    task: String,
    domain: Option<String>,
) -> Result<ExecutionResult, String> {
    let domain = domain.unwrap_or_else(|| "general".into());
    let mut runtime = state.runtime.lock().await;
    match runtime.run(&task, &domain).await {
        Ok(report) => Ok(ExecutionResult {
            success: report.success,
            output: report.output,
            duration_ms: report.duration_ms,
            execution_id: report.execution_id,
            provider: report.provider,
        }),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn health(state: State<'_, DesktopState>) -> Result<HealthStatus, String> {
    let runtime = state.runtime.lock().await;
    let harnesses = runtime.council.installed_entries().len();
    let genes = runtime.council.genes.iter().count();
    let cr = ConnectionRegistry::load();
    Ok(HealthStatus {
        runtime: true,
        version: env!("CARGO_PKG_VERSION").into(),
        session_id: state.session_id.clone(),
        harnesses,
        genes,
        providers: cr.connections.len(),
    })
}

#[tauri::command]
async fn open_project(
    state: State<'_, DesktopState>,
    path: String,
) -> Result<ProjectInfo, String> {
    let p = std::path::Path::new(&path);
    let name = p.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());

    let branch = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(&path)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    let dirty = std::process::Command::new("git")
        .args(["diff", "--stat"])
        .current_dir(&path)
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);

    Ok(ProjectInfo {
        path,
        name,
        branch: if branch.is_empty() { "main".into() } else { branch },
        dirty,
        last_session: None,
    })
}

// ── Harness Commands ──

#[tauri::command]
async fn list_harnesses(state: State<'_, DesktopState>) -> Result<Vec<HarnessInfo>, String> {
    let runtime = state.runtime.lock().await;
    let entries: Vec<HarnessInfo> = runtime.council.installed_entries()
        .iter()
        .map(|(h, s)| HarnessInfo {
            id: h.manifest().id.clone(),
            kind: format!("{:?}", h.manifest().kind),
            version: h.manifest().version.clone(),
            enabled: matches!(s, pandora_shadow_council::HarnessState::Enabled),
        })
        .collect();
    Ok(entries)
}

#[tauri::command]
async fn enable_harness(state: State<'_, DesktopState>, id: String) -> Result<(), String> {
    let mut runtime = state.runtime.lock().await;
    runtime.council.enable(&id)
}

#[tauri::command]
async fn disable_harness(state: State<'_, DesktopState>, id: String) -> Result<(), String> {
    let mut runtime = state.runtime.lock().await;
    runtime.council.disable(&id)
}

// ── Gene Commands ──

#[tauri::command]
async fn list_genes(state: State<'_, DesktopState>) -> Result<Vec<GeneInfo>, String> {
    let runtime = state.runtime.lock().await;
    let genes: Vec<GeneInfo> = runtime.council.genes.iter()
        .map(|g| GeneInfo {
            id: g.manifest().id.clone(),
            kind: format!("{:?}", g.manifest().kind),
            version: g.manifest().version.clone(),
            capabilities: g.manifest().capabilities.clone(),
        })
        .collect();
    Ok(genes)
}

// ── Provider Commands ──

#[tauri::command]
async fn list_providers() -> Result<Vec<ProviderInfo>, String> {
    let cr = ConnectionRegistry::load();
    let healthy = cr.healthy();
    let providers: Vec<ProviderInfo> = cr.connections.iter()
        .map(|c| ProviderInfo {
            name: c.name.clone(),
            kind: format!("{:?}", c.kind),
            endpoint: c.endpoint.clone(),
            model: c.default_model.clone(),
            healthy: healthy.iter().any(|h| h.name == c.name),
        })
        .collect();
    Ok(providers)
}

#[tauri::command]
async fn add_provider(
    name: String,
    kind: String,
    endpoint: String,
    model: Option<String>,
    api_key: Option<String>,
) -> Result<(), String> {
    use pandora_types::connection_manager::ConnectionKind;
    let kind = match kind.as_str() {
        "ollama" => ConnectionKind::Ollama,
        "openai" => ConnectionKind::OpenAI,
        "openai-compatible" => ConnectionKind::OpenAICompatible,
        "anthropic" => ConnectionKind::Anthropic,
        "gemini" => ConnectionKind::Gemini,
        "openrouter" => ConnectionKind::OpenRouter,
        "groq" => ConnectionKind::Groq,
        "deepseek" => ConnectionKind::DeepSeek,
        "custom" => ConnectionKind::Custom,
        _ => return Err(format!("Unknown kind: {kind}")),
    };
    let conn = pandora_types::connection_manager::Connection::new(&name, kind, &endpoint)
        .with_model(&model.unwrap_or_default());
    let mut reg = ConnectionRegistry::load();
    reg.add(conn).map_err(|e| e.to_string())
}

// ── Session Commands ──

#[tauri::command]
async fn list_sessions(state: State<'_, DesktopState>) -> Result<Vec<SessionInfo>, String> {
    let sessions_dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("sessions");

    if !sessions_dir.exists() {
        return Ok(vec![]);
    }

    let mut sessions = vec![];
    if let Ok(entries) = std::fs::read_dir(&sessions_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        sessions.push(SessionInfo {
                            id: path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                            prompt: json["prompt"].as_str().unwrap_or("").to_string(),
                            created: json["created"].as_str().unwrap_or("").to_string(),
                            status: json["status"].as_str().unwrap_or("completed").to_string(),
                        });
                    }
                }
            }
        }
    }
    Ok(sessions)
}

// ── Context Commands ──

#[tauri::command]
async fn context_info(state: State<'_, DesktopState>) -> Result<ContextInfo, String> {
    // Return estimated context usage
    Ok(ContextInfo {
        used_tokens: 0,
        limit_tokens: 128_000,
        system_prompt: 8_400,
        conversation: 0,
        memory: 0,
        tool_results: 0,
    })
}

// ── Approval Commands ──

#[tauri::command]
async fn list_approvals() -> Result<Vec<ApprovalInfo>, String> {
    let approvals_dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("approvals");

    if !approvals_dir.exists() {
        return Ok(vec![]);
    }

    let mut approvals = vec![];
    if let Ok(entries) = std::fs::read_dir(&approvals_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map_or(false, |e| e == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        if json["status"].as_str() == Some("pending") {
                            approvals.push(ApprovalInfo {
                                id: path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
                                reason: json["reason"].as_str().unwrap_or("").to_string(),
                                tool: json["tool_call"].as_str().unwrap_or("").to_string(),
                                risk: json["risk"].as_str().unwrap_or("unknown").to_string(),
                                timestamp: json["timestamp"].as_str().unwrap_or("").to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(approvals)
}

#[tauri::command]
async fn approve_action(id: String) -> Result<(), String> {
    let approvals_dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("approvals");
    let path = approvals_dir.join(format!("{id}.json"));
    if path.exists() {
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut json: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        json["status"] = serde_json::Value::String("approved".into());
        std::fs::write(&path, serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn reject_action(id: String) -> Result<(), String> {
    let approvals_dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("approvals");
    let path = approvals_dir.join(format!("{id}.json"));
    if path.exists() {
        let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let mut json: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        json["status"] = serde_json::Value::String("rejected".into());
        std::fs::write(&path, serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ── Main ──

fn main() {
    let mut runtime = PandoraRuntime::new();
    pandora_harnesses::register_all(&mut runtime.council);

    let session_id = format!(
        "desktop-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    );

    let state = DesktopState {
        runtime: Arc::new(Mutex::new(runtime)),
        session_id,
        project_path: None,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            run_task,
            health,
            open_project,
            list_harnesses,
            enable_harness,
            disable_harness,
            list_genes,
            list_providers,
            add_provider,
            list_sessions,
            context_info,
            list_approvals,
            approve_action,
            reject_action,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Pandora Desktop");
}
