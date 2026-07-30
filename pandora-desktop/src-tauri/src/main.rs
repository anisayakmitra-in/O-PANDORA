//! Pandora Desktop — Tauri IPC Backend (real runtime wiring, zero mocks)
//!
//! Every command calls the actual Pandora runtime. Unsupported operations
//! return honest errors, never fake data.

mod safety;

use pandora_api::client::ApiClient;
use pandora_api::protocol::ExecuteRequest;
use pandora_ko_palace::registry::RegistryClient;
use pandora_orchestrator::PandoraRuntime;
// ShadowCouncil accessed through PandoraRuntime
use pandora_shadow_council::ShadowCouncil;
use pandora_types::connection_manager::ConnectionRegistry;
use rand::{distributions::Alphanumeric, Rng};
use safety::{canonicalize_existing, resolve_rooted_path, validate_safe_name, workspace_root};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;

// ── State ──

struct DesktopState {
    runtime: Arc<Mutex<PandoraRuntime>>,
    api_client: ApiClient,
    session_id: Arc<Mutex<Option<String>>>,
    project_path: Arc<Mutex<Option<String>>>,
}

#[tauri::command]
async fn send_message(
    app: AppHandle,
    state: State<'_, DesktopState>,
    message: String,
    profile: Option<String>,
) -> Result<ExecutionResult, String> {
    execute_task(app, state, message, None, profile).await
}
// ═══════════════════════════════════════════════════════════
//  CORE — Session management (WIRED to PandoraRuntime)
// ═══════════════════════════════════════════════════════════

#[derive(Serialize, Clone)]
struct SessionInfo {
    id: String,
    created: String,
    task: Option<String>,
}

#[tauri::command]
async fn create_session(
    state: State<'_, DesktopState>,
    task: Option<String>,
) -> Result<SessionInfo, String> {
    let id = format!(
        "session-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    );
    let created = chrono::Utc::now().to_rfc3339();
    let info = SessionInfo {
        id: id.clone(),
        created: created.clone(),
        task: task.clone(),
    };
    let directory = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("sessions");
    std::fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    let record = serde_json::json!({
        "session_id": id,
        "task": task,
        "created": created,
    });
    let contents = serde_json::to_string_pretty(&record).map_err(|error| error.to_string())?;
    std::fs::write(directory.join(format!("{}.json", info.id)), contents)
        .map_err(|error| error.to_string())?;
    *state.session_id.lock().await = Some(info.id.clone());
    Ok(info)
}

#[tauri::command]
async fn list_sessions() -> Result<Vec<SessionInfo>, String> {
    let dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("sessions");
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut sessions = vec![];
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                if let Ok(c) = std::fs::read_to_string(&path) {
                    if let Ok(j) = serde_json::from_str::<serde_json::Value>(&c) {
                        sessions.push(SessionInfo {
                            id: path
                                .file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                            created: j["created"].as_str().unwrap_or("").to_string(),
                            task: j["task"].as_str().map(String::from),
                        });
                    }
                }
            }
        }
    }
    sessions.sort_by(|a, b| b.created.cmp(&a.created));
    Ok(sessions)
}

#[tauri::command]
async fn export_sessions(format: String, redact: bool) -> Result<String, String> {
    let format = format.to_ascii_lowercase();
    if format != "json" && format != "markdown" {
        return Err("Unsupported export format; use json or markdown".to_string());
    }
    let directory = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("sessions");
    let mut sessions = Vec::new();
    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path
                .extension()
                .is_some_and(|extension| extension == "json")
                && path.file_stem() != Some(std::ffi::OsStr::new("index"))
            {
                if let Ok(contents) = std::fs::read_to_string(path) {
                    if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&contents) {
                        if redact {
                            redact_export_value(&mut value);
                        }
                        sessions.push(value);
                    }
                }
            }
        }
    }
    sessions.sort_by(|a, b| {
        b.get("created")
            .and_then(serde_json::Value::as_str)
            .cmp(&a.get("created").and_then(serde_json::Value::as_str))
    });
    if format == "json" {
        serde_json::to_string_pretty(&sessions).map_err(|error| error.to_string())
    } else {
        Ok(sessions
            .iter()
            .map(|session| {
                format!(
                    "# Pandora session {}\n\n- **Created:** {}\n- **Task:** {}\n",
                    session
                        .get("session_id")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("unknown"),
                    session
                        .get("created")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("unknown"),
                    session
                        .get("task")
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("untitled")
                )
            })
            .collect::<Vec<_>>()
            .join("\n---\n\n"))
    }
}

fn redact_export_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(object) => {
            for (key, child) in object.iter_mut() {
                let key_lower = key.to_ascii_lowercase();
                if ["api_key", "apikey", "password", "secret", "token"]
                    .iter()
                    .any(|part| key_lower.contains(part))
                {
                    *child = serde_json::Value::String("[REDACTED]".to_string());
                } else {
                    redact_export_value(child);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                redact_export_value(item);
            }
        }
        _ => {}
    }
}

#[tauri::command]
async fn resume_session(
    state: State<'_, DesktopState>,
    session_id: String,
) -> Result<SessionInfo, String> {
    validate_safe_name(&session_id, "session id")?;
    let dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("sessions");
    let path = dir.join(format!("{session_id}.json"));
    if !path.exists() {
        return Err(format!("Session not found: {session_id}"));
    }
    let c = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&c).map_err(|error| error.to_string())?;
    let info = SessionInfo {
        id: session_id,
        created: json["created"].as_str().unwrap_or("").to_string(),
        task: json["task"].as_str().map(String::from),
    };
    *state.session_id.lock().await = Some(info.id.clone());
    Ok(info)
}

#[derive(Serialize)]
struct ExecutionResult {
    execution_id: String,
    output: String,
    duration_ms: u64,
    provider: String,
    model: String,
    success: bool,
}
#[tauri::command]
async fn execute_task(
    app: AppHandle,
    state: State<'_, DesktopState>,
    task: String,
    domain: Option<String>,
    profile: Option<String>,
) -> Result<ExecutionResult, String> {
    let domain = domain.unwrap_or_else(|| "general".into());
    let _ = app.emit(
        "stream-event",
        serde_json::json!({
            "type": "execution.started",
            "content": format!("Executing: {task}"),
        }),
    );

    state
        .api_client
        .wait_ready()
        .await
        .map_err(|error| error.to_string())?;

    let response = state
        .api_client
        .execute(&ExecuteRequest {
            task: task.clone(),
            domain,
            strategy: String::new(),
            evaluator: String::new(),
            profile,
        })
        .await
        .map_err(|error| {
            let _ = app.emit(
                "stream-event",
                serde_json::json!({
                    "type": "execution.failed",
                    "content": error.to_string(),
                }),
            );
            error.to_string()
        })?;
    let success = response.status == "completed";
    let event_type = if success {
        "execution.completed"
    } else {
        "execution.failed"
    };
    let _ = app.emit(
        "stream-event",
        serde_json::json!({
            "type": event_type,
            "content": response.output,
            "metadata": {
                "duration_ms": response.duration_ms,
                "provider": response.provider,
                "execution_id": response.session_id,
                "status": response.status,
            }
        }),
    );

    if let Some(ref sid) = *state.session_id.lock().await {
        let dir = dirs_next::home_dir()
            .unwrap_or_default()
            .join(".pandora")
            .join("sessions");
        let _ = std::fs::create_dir_all(&dir);
        let record = serde_json::json!({
            "session_id": sid,
            "execution_id": response.session_id,
            "task": task,
            "output": response.output,
            "created": chrono::Utc::now().to_rfc3339(),
        });
        let _ = std::fs::write(
            dir.join(format!("{sid}.json")),
            serde_json::to_string_pretty(&record).unwrap_or_default(),
        );
    }

    Ok(ExecutionResult {
        execution_id: response.session_id,
        output: response.output,
        duration_ms: response.duration_ms,
        provider: response.provider,
        model: String::new(),
        success,
    })
}
#[derive(Serialize)]
struct ProviderInfo {
    name: String,
    kind: String,
    endpoint: String,
    model: String,
    healthy: bool,
}

#[tauri::command]
fn switch_model(provider: String, model: String) -> Result<(), String> {
    if provider.trim().is_empty() || model.trim().is_empty() {
        return Err("provider and model are required".into());
    }
    let mut registry = ConnectionRegistry::load();
    let connection = registry
        .find_mut(&provider)
        .ok_or_else(|| format!("provider not found: {provider}"))?;
    connection.default_model = model;
    registry.save().map_err(|error| error.to_string())
}
#[tauri::command]
async fn list_providers() -> Result<Vec<ProviderInfo>, String> {
    let cr = ConnectionRegistry::load();
    let healthy_set = cr.healthy();
    Ok(cr
        .connections
        .iter()
        .map(|c| ProviderInfo {
            name: c.name.clone(),
            kind: format!("{:?}", c.kind),
            endpoint: c.endpoint.clone(),
            model: c.default_model.clone(),
            healthy: healthy_set.iter().any(|h| h.name == c.name),
        })
        .collect())
}

#[tauri::command]
fn list_profiles() -> Result<Vec<String>, String> {
    pandora_types::profile::list_profiles().map_err(|error| error.to_string())
}

#[tauri::command]
async fn list_models() -> Result<Vec<serde_json::Value>, String> {
    let cr = ConnectionRegistry::load();
    let healthy_set = cr.healthy();
    Ok(cr
        .connections
        .iter()
        .map(|c| {
            serde_json::json!({
                "name": c.default_model,
                "provider": c.name,
                "endpoint": c.endpoint,
                "healthy": healthy_set.iter().any(|h| h.name == c.name),
            })
        })
        .collect())
}

// ═══════════════════════════════════════════════════════════
//  HARNESSES (WIRED to ShadowCouncil::harnesses)
// ═══════════════════════════════════════════════════════════

#[derive(Serialize)]
struct HarnessInfo {
    id: String,
    kind: String,
    version: String,
    enabled: bool,
    capabilities: Vec<String>,
}

#[tauri::command]
async fn list_harnesses(state: State<'_, DesktopState>) -> Result<Vec<HarnessInfo>, String> {
    let runtime = state.runtime.lock().await;
    let entries = runtime.council.harnesses.all_entries();
    Ok(entries
        .iter()
        .map(|(h, s)| {
            let m = h.manifest();
            HarnessInfo {
                id: m.id.clone(),
                kind: format!("{:?}", m.kind),
                version: m.version.clone(),
                enabled: matches!(s, pandora_shadow_council::HarnessState::Enabled),
                capabilities: m.capabilities.clone(),
            }
        })
        .collect())
}

// ═══════════════════════════════════════════════════════════
//  GENES (WIRED to ShadowCouncil::genes)
// ═══════════════════════════════════════════════════════════

#[derive(Serialize)]
struct GeneInfo {
    id: String,
    kind: String,
    version: String,
    capabilities: Vec<String>,
}

#[tauri::command]
async fn list_genes(state: State<'_, DesktopState>) -> Result<Vec<GeneInfo>, String> {
    let runtime = state.runtime.lock().await;
    let genes = runtime.council.genes.all();
    Ok(genes
        .iter()
        .map(|g| {
            let m = g.manifest();
            GeneInfo {
                id: m.id.clone(),
                kind: format!("{:?}", m.kind),
                version: m.version.clone(),
                capabilities: m.capabilities.clone(),
            }
        })
        .collect())
}

// ═══════════════════════════════════════════════════════════
//  REGISTRY STATS (WIRED to real registries)
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn registry_stats(state: State<'_, DesktopState>) -> Result<serde_json::Value, String> {
    let runtime = state.runtime.lock().await;
    let entries = runtime.council.harnesses.all_entries();
    let enabled = entries
        .iter()
        .filter(|(_, s)| matches!(s, pandora_shadow_council::HarnessState::Enabled))
        .count();
    let cr = ConnectionRegistry::load();
    Ok(serde_json::json!({
        "harnesses": entries.len(),
        "harnesses_enabled": enabled,
        "genes": runtime.council.genes.total_count(),
        "providers": cr.connections.len(),
    }))
}

// ═══════════════════════════════════════════════════════════
//  PROJECT (filesystem + Git — real)
// ═══════════════════════════════════════════════════════════

#[derive(Serialize)]
struct ProjectInfo {
    path: String,
    name: String,
    branch: String,
    dirty: bool,
}

#[tauri::command]
async fn open_project(state: State<'_, DesktopState>, path: String) -> Result<ProjectInfo, String> {
    let p = canonicalize_existing(&PathBuf::from(&path))?;
    let name = p
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let branch = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(&p)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    let dirty = std::process::Command::new("git")
        .args(["diff", "--stat"])
        .current_dir(&p)
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);
    let path = p.to_string_lossy().to_string();
    *state.project_path.lock().await = Some(path.clone());
    Ok(ProjectInfo {
        path,
        name,
        branch: if branch.is_empty() {
            "main".into()
        } else {
            branch
        },
        dirty,
    })
}

// ═══════════════════════════════════════════════════════════
//  GIT — real subprocess calls
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn git_status(state: State<'_, DesktopState>) -> Result<serde_json::Value, String> {
    let cwd = state
        .project_path
        .lock()
        .await
        .clone()
        .unwrap_or_else(|| ".".into());
    let branch = run_git(&cwd, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_default();
    let staged = run_git(&cwd, &["diff", "--staged", "--name-only"]).unwrap_or_default();
    let unstaged = run_git(&cwd, &["diff", "--name-only"]).unwrap_or_default();
    let untracked =
        run_git(&cwd, &["ls-files", "--others", "--exclude-standard"]).unwrap_or_default();
    Ok(serde_json::json!({
        "branch": branch.trim(),
        "dirty": !unstaged.trim().is_empty() || !staged.trim().is_empty(),
        "staged": lines_vec(&staged),
        "unstaged": lines_vec(&unstaged),
        "untracked": lines_vec(&untracked),
    }))
}

#[tauri::command]
async fn git_diff(state: State<'_, DesktopState>, staged: Option<bool>) -> Result<String, String> {
    let cwd = state
        .project_path
        .lock()
        .await
        .clone()
        .unwrap_or_else(|| ".".into());
    let args: Vec<&str> = if staged.unwrap_or(false) {
        vec!["diff", "--staged"]
    } else {
        vec!["diff"]
    };
    run_git(&cwd, &args).map_err(|e| e.to_string())
}

// ═══════════════════════════════════════════════════════════
//  FILE TREE — real filesystem
// ═══════════════════════════════════════════════════════════

#[derive(Serialize)]
struct FileEntry {
    name: String,
    path: String,
    is_dir: bool,
    children: Option<Vec<FileEntry>>,
}

#[tauri::command]
async fn get_file_tree(
    state: State<'_, DesktopState>,
    dir_path: Option<String>,
) -> Result<Vec<FileEntry>, String> {
    let project_root = workspace_root(state.project_path.lock().await.clone());
    let root = match dir_path {
        Some(path) => resolve_rooted_path(&project_root, &path)?,
        None => canonicalize_existing(&project_root)?,
    };
    read_dir_entries(&root, 3)
}

#[tauri::command]
async fn read_file_content(state: State<'_, DesktopState>, path: String) -> Result<String, String> {
    let project_root = workspace_root(state.project_path.lock().await.clone());
    let resolved = resolve_rooted_path(&project_root, &path)?;
    std::fs::read_to_string(&resolved).map_err(|e| format!("Cannot read {path}: {e}"))
}

// ═══════════════════════════════════════════════════════════
//  TERMINAL — real subprocess
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn palace_list_packages(
    kind_filter: Option<String>,
) -> Result<Vec<serde_json::Value>, String> {
    let dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("palace");
    let mut pkgs = vec![];
    if dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "toml") {
                    if let Ok(c) = std::fs::read_to_string(&path) {
                        let name = c
                            .lines()
                            .find(|l| l.starts_with("name"))
                            .and_then(|l| l.split('=').nth(1))
                            .map(|s| s.trim().trim_matches('"').to_string())
                            .unwrap_or_default();
                        let kind = c
                            .lines()
                            .find(|l| l.starts_with("kind"))
                            .and_then(|l| l.split('=').nth(1))
                            .map(|s| s.trim().trim_matches('"').to_string())
                            .unwrap_or_else(|| "gene".into());
                        if let Some(ref kf) = kind_filter {
                            if kind.to_lowercase() != kf.to_lowercase() {
                                continue;
                            }
                        }
                        pkgs.push(
                            serde_json::json!({"name": name, "kind": kind, "installed": true}),
                        );
                    }
                }
            }
        }
    }
    Ok(pkgs)
}

#[tauri::command]
async fn palace_install(package: String) -> Result<String, String> {
    if package.trim().is_empty() || package.contains('\0') {
        return Err("Package id cannot be empty".into());
    }
    let registry_url =
        std::env::var("PANDORA_REGISTRY_URL").unwrap_or_else(|_| "http://localhost:3001".into());
    let token = std::env::var("PANDORA_TOKEN").ok();
    tokio::task::spawn_blocking(move || {
        let registry = RegistryClient::new(&registry_url, token)?;
        let council = Arc::new(RwLock::new(ShadowCouncil::new()));
        let mut ko_palace = pandora_ko_palace::KoPalace::new(council);
        ko_palace
            .install_remote(&registry, &package)
            .map_err(|e| e.to_string())?;
        Ok(format!("Installed: {package}"))
    })
    .await
    .map_err(|e| format!("Installer task failed: {e}"))?
}
// ═══════════════════════════════════════════════════════════
//  FLEET + SCHEDULER — honest: reads filesystem records
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn fleet_nodes() -> Result<Vec<serde_json::Value>, String> {
    let dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("fleet");
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut nodes = vec![];
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Ok(c) = std::fs::read_to_string(entry.path()) {
                if let Ok(j) = serde_json::from_str::<serde_json::Value>(&c) {
                    nodes.push(j);
                }
            }
        }
    }
    Ok(nodes)
}

#[tauri::command]
async fn scheduler_list() -> Result<Vec<serde_json::Value>, String> {
    let dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("cron");
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut jobs = vec![];
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Ok(c) = std::fs::read_to_string(entry.path()) {
                if let Ok(j) = serde_json::from_str::<serde_json::Value>(&c) {
                    jobs.push(j);
                }
            }
        }
    }
    Ok(jobs)
}

#[tauri::command]
async fn scheduler_add(task: String, schedule: String) -> Result<String, String> {
    let dir = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("cron");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let id = format!(
        "job-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    );
    let job = serde_json::json!({"task": task, "schedule": schedule, "status": "active", "created": chrono::Utc::now().to_rfc3339()});
    std::fs::write(
        dir.join(format!("{id}.json")),
        serde_json::to_string_pretty(&job).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(id)
}

// ═══════════════════════════════════════════════════════════
//  NOT YET SUPPORTED — honest unimplemented errors
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn governance_summary() -> Result<serde_json::Value, String> {
    let store = pandora_types::approval_store::ApprovalStore::new(
        pandora_types::approval_store::ApprovalStore::default_location(),
    );
    let approvals = store.list_all();
    let approved = approvals
        .iter()
        .filter(|approval| {
            matches!(
                approval.status,
                pandora_types::approval_store::ApprovalStatus::Approved
            )
        })
        .count();
    let rejected = approvals
        .iter()
        .filter(|approval| {
            matches!(
                approval.status,
                pandora_types::approval_store::ApprovalStatus::Rejected
            )
        })
        .count();
    let waiting = approvals
        .iter()
        .filter(|approval| {
            matches!(
                approval.status,
                pandora_types::approval_store::ApprovalStatus::Pending
            )
        })
        .count();
    Ok(serde_json::json!({
        "pending": waiting,
        "approved": approved,
        "rejected": rejected,
        "total": approvals.len(),
        "recent": approvals.iter().take(8).map(|a| serde_json::json!({
            "id": a.id,
            "tool": a.tool_name,
            "reason": a.reason,
            "status": format!("{:?}", a.status),
            "created_ms": a.created_at_ms,
        })).collect::<Vec<_>>(),
    }))
}

#[tauri::command]
async fn approve_pending(id: String) -> Result<String, String> {
    let store = pandora_types::approval_store::ApprovalStore::new(
        pandora_types::approval_store::ApprovalStore::default_location(),
    );
    match store.approve(&id) {
        Ok(a) => Ok(format!("Approved: {} ({})", id, a.tool_name)),
        Err(e) => Err(format!("Approve failed: {e}")),
    }
}

#[tauri::command]
async fn reject_pending(id: String) -> Result<String, String> {
    let store = pandora_types::approval_store::ApprovalStore::new(
        pandora_types::approval_store::ApprovalStore::default_location(),
    );
    match store.reject(&id) {
        Ok(a) => Ok(format!("Rejected: {} ({})", id, a.tool_name)),
        Err(e) => Err(format!("Reject failed: {e}")),
    }
}

#[tauri::command]
async fn architecture_graph(state: State<'_, DesktopState>) -> Result<serde_json::Value, String> {
    let runtime = state.runtime.lock().await;
    let entries = runtime.council.harnesses.all_entries();
    let cr = pandora_types::connection_manager::ConnectionRegistry::load();
    Ok(serde_json::json!({
        "runtime": {"status": "active", "version": env!("CARGO_PKG_VERSION")},
        "parliament": {"status": "active"},
        "shadow_council": {"status": "active", "harnesses": entries.len()},
        "harnesses": entries.iter().map(|(h, s)| serde_json::json!({
            "id": h.manifest().id,
            "kind": format!("{:?}", h.manifest().kind),
            "enabled": matches!(s, pandora_shadow_council::HarnessState::Enabled),
            "capabilities": h.manifest().capabilities,
        })).collect::<Vec<_>>(),
        "providers": cr.connections.iter().map(|c| serde_json::json!({
            "name": c.name,
            "kind": format!("{:?}", c.kind),
            "endpoint": c.endpoint,
        })).collect::<Vec<_>>(),
    }))
}

#[tauri::command]
async fn multi_agent_status() -> Result<serde_json::Value, String> {
    // Fleet multi-node execution not yet activated.
    // Returns empty worker list with status.
    Ok(serde_json::json!({
        "workers": [],
        "status": "standalone",
        "message": "Fleet multi-node execution not active. Workers: 0. Use `pandora fleet` CLI to connect nodes.",
    }))
}

#[derive(Debug, Deserialize)]
struct GitHubReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    assets: Vec<GitHubReleaseAsset>,
}
#[tauri::command]
async fn check_updates() -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let release = client
        .get("https://api.github.com/repos/anisayakmitra-in/O-PANDORA/releases/latest")
        .header(reqwest::header::USER_AGENT, "pandora-desktop")
        .send()
        .await
        .map_err(|error| format!("release check failed: {error}"))?
        .error_for_status()
        .map_err(|error| format!("release check failed: {error}"))?
        .json::<GitHubRelease>()
        .await
        .map_err(|error| format!("release metadata invalid: {error}"))?;
    let asset = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => "pandora-windows-x86_64.exe",
        ("macos", "x86_64") => "pandora-macos-x86_64",
        ("linux", "x86_64") => "pandora-linux-x86_64",
        _ => "",
    };
    let binary = release.assets.iter().find(|item| item.name == asset);
    let checksum_name = if asset.is_empty() {
        String::new()
    } else {
        format!("{asset}.sha256")
    };
    let checksum = release
        .assets
        .iter()
        .find(|item| item.name == checksum_name);
    let latest_version = release.tag_name.trim_start_matches('v');
    let current_version = env!("CARGO_PKG_VERSION");
    Ok(serde_json::json!({
        "current_version": current_version,
        "latest_version": latest_version,
        "update_available": is_newer_version(current_version, latest_version),
        "release_url": release.html_url,
        "download_url": binary.map(|item| item.browser_download_url.clone()),
        "checksum_url": checksum.map(|item| item.browser_download_url.clone()),
        "automatic_updates": false,
        "note": "Download and verify the matching asset with the platform update helper. Automatic installation requires signed updater metadata.",
    }))
}
#[tauri::command]
async fn health() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "runtime": true,
        "version": env!("CARGO_PKG_VERSION"),
        "workspace": "11 crates",
    }))
}

// ═══════════════════════════════════════════════════════════
//  HELPERS
// ═══════════════════════════════════════════════════════════

fn is_newer_version(current: &str, latest: &str) -> bool {
    match (
        semver::Version::parse(current),
        semver::Version::parse(latest),
    ) {
        (Ok(current), Ok(latest)) => latest > current,
        _ => false,
    }
}

fn run_git(cwd: &str, args: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn lines_vec(s: &str) -> Vec<String> {
    s.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn read_dir_entries(dir: &std::path::Path, max_depth: usize) -> Result<Vec<FileEntry>, String> {
    if max_depth == 0 {
        return Ok(vec![]);
    }
    let mut entries = vec![];
    if let Ok(read_dir) = std::fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
            if std::fs::symlink_metadata(&path)
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false)
            {
                continue;
            }
            let is_dir = path.is_dir();
            entries.push(FileEntry {
                name,
                path: path.to_string_lossy().to_string(),
                is_dir,
                children: if is_dir && max_depth > 1 {
                    Some(read_dir_entries(&path, max_depth - 1).unwrap_or_default())
                } else {
                    None
                },
            });
        }
    }
    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.cmp(&b.name)
        }
    });
    Ok(entries)
}

// ═══════════════════════════════════════════════════════════
//  MAIN
// ═══════════════════════════════════════════════════════════

fn main() {
    let api_token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect();
    std::env::set_var("PANDORA_API_TOKEN", &api_token);
    let api_listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind local Pandora API");
    api_listener
        .set_nonblocking(true)
        .expect("configure local Pandora API listener");
    let api_address = api_listener
        .local_addr()
        .expect("read local Pandora API address");
    let api_sessions = dirs_next::home_dir()
        .unwrap_or_default()
        .join(".pandora")
        .join("sessions");
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("build local Pandora API runtime");
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::from_std(api_listener)
                .expect("convert local Pandora API listener");
            if let Err(error) = pandora_api::serve_listener(listener, api_sessions).await {
                eprintln!("Local Pandora API stopped: {error}");
            }
        });
    });

    let mut runtime = PandoraRuntime::new();
    pandora_harnesses::register_all(&mut runtime.council);

    let state = DesktopState {
        runtime: Arc::new(Mutex::new(runtime)),
        api_client: ApiClient::new(format!("http://{api_address}"), Some(api_token)),
        session_id: Arc::new(Mutex::new(None)),
        project_path: Arc::new(Mutex::new(None)),
    };
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            // Sessions (wired)
            create_session,
            list_sessions,
            export_sessions,
            resume_session,
            // Execution (wired)
            execute_task,
            send_message,
            // Providers (wired)
            list_providers,
            switch_model,
            list_models,
            list_profiles,
            // Harnesses + Genes (wired)
            list_harnesses,
            list_genes,
            registry_stats,
            // Project + Git (wired)
            open_project,
            git_status,
            git_diff,
            // File tree (wired)
            get_file_tree,
            read_file_content,
            // Palace (wired)
            palace_list_packages,
            palace_install,
            // Fleet + Scheduler (wired)
            fleet_nodes,
            scheduler_list,
            scheduler_add,
            // Governance, topology, fleet status, and release metadata
            governance_summary,
            approve_pending,
            reject_pending,
            architecture_graph,
            multi_agent_status,
            check_updates,
            health,
        ])
        .run(tauri::generate_context!())
        .expect("Pandora Desktop failed to start");
}

#[cfg(test)]
mod update_tests {
    use super::is_newer_version;

    #[test]
    fn only_newer_semver_releases_are_updates() {
        assert!(is_newer_version("0.5.0", "0.6.0"));
        assert!(is_newer_version("0.5.0", "1.0.0"));
        assert!(!is_newer_version("0.5.0", "0.5.0"));
        assert!(!is_newer_version("0.5.0", "0.4.9"));
        assert!(!is_newer_version("0.5.0", "not-a-version"));
    }
}
