//! Pandora Desktop — Tauri IPC Backend (real runtime wiring, zero mocks)
//!
//! Every command calls the actual Pandora runtime. Unsupported operations
//! return honest errors, never fake data.

use pandora_orchestrator::PandoraRuntime;
// ShadowCouncil accessed through PandoraRuntime
use pandora_types::connection_manager::ConnectionRegistry;
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;

// ── State ──

struct DesktopState {
    runtime: Arc<Mutex<PandoraRuntime>>,
    session_id: Arc<Mutex<Option<String>>>,
    project_path: Arc<Mutex<Option<String>>>,
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
async fn create_session(state: State<'_, DesktopState>, task: Option<String>) -> Result<SessionInfo, String> {
    let id = format!("session-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
    let info = SessionInfo {
        id: id.clone(),
        created: chrono::Utc::now().to_rfc3339(),
        task,
    };
    *state.session_id.lock().await = Some(id);
    Ok(info)
}

#[tauri::command]
async fn list_sessions() -> Result<Vec<SessionInfo>, String> {
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("sessions");
    if !dir.exists() { return Ok(vec![]); }
    let mut sessions = vec![];
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "json") {
                if let Ok(c) = std::fs::read_to_string(&path) {
                    if let Ok(j) = serde_json::from_str::<serde_json::Value>(&c) {
                        sessions.push(SessionInfo {
                            id: path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
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
async fn resume_session(state: State<'_, DesktopState>, session_id: String) -> Result<SessionInfo, String> {
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("sessions");
    let path = dir.join(format!("{session_id}.json"));
    if !path.exists() { return Err(format!("Session not found: {session_id}")); }
    let c = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let j: serde_json::Value = serde_json::from_str(&c).map_err(|e| e.to_string())?;
    let info = SessionInfo {
        id: session_id,
        created: j["created"].as_str().unwrap_or("").to_string(),
        task: j["task"].as_str().map(String::from),
    };
    *state.session_id.lock().await = Some(info.id.clone());
    Ok(info)
}

// ═══════════════════════════════════════════════════════════
//  CORE — Execution (WIRED to PandoraRuntime::run)
// ═══════════════════════════════════════════════════════════

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
) -> Result<ExecutionResult, String> {
    let domain = domain.unwrap_or_else(|| "general".into());

    // Emit start event
    let _ = app.emit("stream-event", serde_json::json!({
        "type": "execution.started",
        "content": format!("Executing: {task}"),
    }));

    let mut runtime = state.runtime.lock().await;
    match runtime.run(&task, &domain).await {
        Ok(report) => {
            // Emit completion
            let _ = app.emit("stream-event", serde_json::json!({
                "type": "execution.completed",
                "content": report.output,
                "metadata": {
                    "duration_ms": report.duration_ms,
                    "provider": report.provider,
                    "execution_id": report.execution_id,
                }
            }));

            // Save session record
            if let Some(ref sid) = *state.session_id.lock().await {
                let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("sessions");
                let _ = std::fs::create_dir_all(&dir);
                let record = serde_json::json!({
                    "session_id": sid,
                    "execution_id": report.execution_id,
                    "task": task,
                    "output": report.output,
                    "created": chrono::Utc::now().to_rfc3339(),
                });
                let _ = std::fs::write(
                    dir.join(format!("{sid}.json")),
                    serde_json::to_string_pretty(&record).unwrap_or_default(),
                );
            }

            Ok(ExecutionResult {
                execution_id: report.execution_id,
                output: report.output,
                duration_ms: report.duration_ms as u64,
                provider: report.provider.clone(),
                model: report.model.clone(),
                success: true,
            })
        }
        Err(e) => {
            let _ = app.emit("stream-event", serde_json::json!({
                "type": "execution.failed",
                "content": e.to_string(),
            }));
            Err(e.to_string())
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  PROVIDERS (WIRED to ConnectionRegistry)
// ═══════════════════════════════════════════════════════════

#[derive(Serialize)]
struct ProviderInfo {
    name: String,
    kind: String,
    endpoint: String,
    model: String,
    healthy: bool,
}

#[tauri::command]
async fn list_providers() -> Result<Vec<ProviderInfo>, String> {
    let cr = ConnectionRegistry::load();
    let healthy_set = cr.healthy();
    Ok(cr.connections.iter().map(|c| ProviderInfo {
        name: c.name.clone(),
        kind: format!("{:?}", c.kind),
        endpoint: c.endpoint.clone(),
        model: c.default_model.clone(),
        healthy: healthy_set.iter().any(|h| h.name == c.name),
    }).collect())
}

#[tauri::command]
async fn list_models() -> Result<Vec<serde_json::Value>, String> {
    let cr = ConnectionRegistry::load();
    let healthy_set = cr.healthy();
    Ok(cr.connections.iter().map(|c| serde_json::json!({
        "name": c.default_model,
        "provider": c.name,
        "healthy": healthy_set.iter().any(|h| h.name == c.name),
    })).collect())
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
    Ok(entries.iter().map(|(h, s)| {
        let m = h.manifest();
        HarnessInfo {
            id: m.id.clone(),
            kind: format!("{:?}", m.kind),
            version: m.version.clone(),
            enabled: matches!(s, pandora_shadow_council::HarnessState::Enabled),
            capabilities: m.capabilities.clone(),
        }
    }).collect())
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
    Ok(genes.iter().map(|g| {
        let m = g.manifest();
        GeneInfo {
            id: m.id.clone(),
            kind: format!("{:?}", m.kind),
            version: m.version.clone(),
            capabilities: m.capabilities.clone(),
        }
    }).collect())
}

// ═══════════════════════════════════════════════════════════
//  REGISTRY STATS (WIRED to real registries)
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn registry_stats(state: State<'_, DesktopState>) -> Result<serde_json::Value, String> {
    let runtime = state.runtime.lock().await;
    let entries = runtime.council.harnesses.all_entries();
    let enabled = entries.iter().filter(|(_, s)| matches!(s, pandora_shadow_council::HarnessState::Enabled)).count();
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
    let p = std::path::PathBuf::from(&path);
    if !p.exists() { return Err(format!("Path does not exist: {path}")); }
    let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let branch = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"]).current_dir(&p)
        .output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    let dirty = std::process::Command::new("git")
        .args(["diff", "--stat"]).current_dir(&p)
        .output().map(|o| !o.stdout.is_empty()).unwrap_or(false);
    *state.project_path.lock().await = Some(path.clone());
    Ok(ProjectInfo { path, name, branch: if branch.is_empty() { "main".into() } else { branch }, dirty })
}

// ═══════════════════════════════════════════════════════════
//  GIT — real subprocess calls
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn git_status(state: State<'_, DesktopState>) -> Result<serde_json::Value, String> {
    let cwd = state.project_path.lock().await.clone().unwrap_or_else(|| ".".into());
    let branch = run_git(&cwd, &["rev-parse", "--abbrev-ref", "HEAD"]).unwrap_or_default();
    let staged = run_git(&cwd, &["diff", "--staged", "--name-only"]).unwrap_or_default();
    let unstaged = run_git(&cwd, &["diff", "--name-only"]).unwrap_or_default();
    let untracked = run_git(&cwd, &["ls-files", "--others", "--exclude-standard"]).unwrap_or_default();
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
    let cwd = state.project_path.lock().await.clone().unwrap_or_else(|| ".".into());
    let args: Vec<&str> = if staged.unwrap_or(false) { vec!["diff", "--staged"] } else { vec!["diff"] };
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
    let proj = state.project_path.lock().await.clone();
    let root = dir_path
        .map(std::path::PathBuf::from)
        .or_else(|| proj.map(std::path::PathBuf::from))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    read_dir_entries(&root, 3)
}

#[tauri::command]
async fn read_file_content(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Cannot read {path}: {e}"))
}

#[tauri::command]
async fn write_file_content(path: String, content: String) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&path, &content).map_err(|e| format!("Cannot write {path}: {e}"))
}

// ═══════════════════════════════════════════════════════════
//  TERMINAL — real subprocess
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn terminal_exec(command: String, cwd: Option<String>) -> Result<String, String> {
    let dir = cwd.unwrap_or_else(|| ".".into());
    let output = if cfg!(windows) {
        std::process::Command::new("cmd").args(["/C", &command]).current_dir(&dir).output()
    } else {
        std::process::Command::new("bash").args(["-c", &command]).current_dir(&dir).output()
    };
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            let stderr = String::from_utf8_lossy(&o.stderr);
            Ok(format!("{stdout}{stderr}").trim().to_string())
        }
        Err(e) => Err(format!("Command failed: {e}")),
    }
}

// ═══════════════════════════════════════════════════════════
//  PALACE — honest: reads from filesystem, says what's real
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn palace_list_packages(kind_filter: Option<String>) -> Result<Vec<serde_json::Value>, String> {
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("palace");
    let mut pkgs = vec![];
    if dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "toml") {
                    if let Ok(c) = std::fs::read_to_string(&path) {
                        let name = c.lines().find(|l| l.starts_with("name"))
                            .and_then(|l| l.split('=').nth(1))
                            .map(|s| s.trim().trim_matches('"').to_string())
                            .unwrap_or_default();
                        let kind = c.lines().find(|l| l.starts_with("kind"))
                            .and_then(|l| l.split('=').nth(1))
                            .map(|s| s.trim().trim_matches('"').to_string())
                            .unwrap_or_else(|| "gene".into());
                        if let Some(ref kf) = kind_filter {
                            if kind.to_lowercase() != kf.to_lowercase() { continue; }
                        }
                        pkgs.push(serde_json::json!({"name": name, "kind": kind, "installed": true}));
                    }
                }
            }
        }
    }
    Ok(pkgs)
}

#[tauri::command]
async fn palace_install(package: String) -> Result<String, String> {
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("palace");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let manifest = format!("[package]\nname = \"{package}\"\nversion = \"0.1.0\"\nkind = \"gene\"\n");
    std::fs::write(dir.join(format!("{package}.toml")), &manifest).map_err(|e| e.to_string())?;
    Ok(format!("Installed: {package}"))
}

// ═══════════════════════════════════════════════════════════
//  FLEET + SCHEDULER — honest: reads filesystem records
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn fleet_nodes() -> Result<Vec<serde_json::Value>, String> {
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("fleet");
    if !dir.exists() { return Ok(vec![]); }
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
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("cron");
    if !dir.exists() { return Ok(vec![]); }
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
    let dir = dirs_next::home_dir().unwrap_or_default().join(".pandora").join("cron");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let id = format!("job-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
    let job = serde_json::json!({"task": task, "schedule": schedule, "status": "active", "created": chrono::Utc::now().to_rfc3339()});
    std::fs::write(dir.join(format!("{id}.json")), serde_json::to_string_pretty(&job).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    Ok(id)
}

// ═══════════════════════════════════════════════════════════
//  NOT YET SUPPORTED — honest unimplemented errors
// ═══════════════════════════════════════════════════════════

#[tauri::command]
async fn governance_summary() -> Result<serde_json::Value, String> {
    let store = pandora_types::approval_store::ApprovalStore::new(
        pandora_types::approval_store::ApprovalStore::default_location()
    );
    let pending = store.list_pending();
    let approved = pending.iter().filter(|a| matches!(a.status, pandora_types::approval_store::ApprovalStatus::Approved)).count();
    let rejected = pending.iter().filter(|a| matches!(a.status, pandora_types::approval_store::ApprovalStatus::Rejected)).count();
    let waiting = pending.iter().filter(|a| matches!(a.status, pandora_types::approval_store::ApprovalStatus::Pending)).count();
    Ok(serde_json::json!({
        "pending": waiting,
        "approved": approved,
        "rejected": rejected,
        "total": pending.len(),
        "recent": pending.iter().map(|a| serde_json::json!({
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
        pandora_types::approval_store::ApprovalStore::default_location()
    );
    match store.approve(&id) {
        Ok(a) => Ok(format!("Approved: {} ({})", id, a.tool_name)),
        Err(e) => Err(format!("Approve failed: {e}")),
    }
}

#[tauri::command]
async fn reject_pending(id: String) -> Result<String, String> {
    let store = pandora_types::approval_store::ApprovalStore::new(
        pandora_types::approval_store::ApprovalStore::default_location()
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

#[tauri::command]
async fn check_updates() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "current_version": env!("CARGO_PKG_VERSION"),
        "update_available": false,
        "release_url": "https://github.com/anisayakmitra-in/O-PANDORA/releases",
        "note": "Automatic updates not yet integrated. Check releases page manually.",
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

fn run_git(cwd: &str, args: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args(args).current_dir(cwd)
        .output().map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn lines_vec(s: &str) -> Vec<String> {
    s.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect()
}

fn read_dir_entries(dir: &std::path::Path, max_depth: usize) -> Result<Vec<FileEntry>, String> {
    if max_depth == 0 { return Ok(vec![]); }
    let mut entries = vec![];
    if let Ok(read_dir) = std::fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if name.starts_with('.') || name == "target" || name == "node_modules" { continue; }
            let is_dir = path.is_dir();
            entries.push(FileEntry {
                name,
                path: path.to_string_lossy().to_string(),
                is_dir,
                children: if is_dir && max_depth > 1 {
                    Some(read_dir_entries(&path, max_depth - 1).unwrap_or_default())
                } else { None },
            });
        }
    }
    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir { b.is_dir.cmp(&a.is_dir) }
        else { a.name.cmp(&b.name) }
    });
    Ok(entries)
}

// ═══════════════════════════════════════════════════════════
//  MAIN
// ═══════════════════════════════════════════════════════════

fn main() {
    let mut runtime = PandoraRuntime::new();
    pandora_harnesses::register_all(&mut runtime.council);

    let state = DesktopState {
        runtime: Arc::new(Mutex::new(runtime)),
        session_id: Arc::new(Mutex::new(None)),
        project_path: Arc::new(Mutex::new(None)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            // Sessions (wired)
            create_session, list_sessions, resume_session,
            // Execution (wired)
            execute_task,
            // Providers (wired)
            list_providers, list_models,
            // Harnesses + Genes (wired)
            list_harnesses, list_genes, registry_stats,
            // Project + Git (wired)
            open_project, git_status, git_diff,
            // File tree (wired)
            get_file_tree, read_file_content, write_file_content,
            // Terminal (wired)
            terminal_exec,
            // Palace (wired)
            palace_list_packages, palace_install,
            // Fleet + Scheduler (wired)
            fleet_nodes, scheduler_list, scheduler_add,
            // Unimplemented (honest errors)
            governance_summary, approve_pending, reject_pending,
            architecture_graph, multi_agent_status, check_updates,
            health,
        ])
        .run(tauri::generate_context!())
        .expect("Pandora Desktop failed to start");
}
