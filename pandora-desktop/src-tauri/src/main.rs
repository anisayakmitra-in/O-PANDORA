use pandora_orchestrator::PandoraRuntime;
use pandora_types::{PandoraError, ParliamentVerdict};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

struct AppState {
    runtime: Arc<Mutex<PandoraRuntime>>,
    session_id: String,
}

#[derive(Serialize)]
struct ExecutionResult {
    success: bool,
    output: String,
    duration_ms: u64,
    execution_id: String,
    provider: String,
    error: Option<String>,
}

#[tauri::command]
async fn run_task(
    state: State<'_, AppState>,
    task: String,
    domain: String,
) -> Result<ExecutionResult, String> {
    let mut runtime = state.runtime.lock().await;
    match runtime.run(&task, &domain).await {
        Ok(report) => Ok(ExecutionResult {
            success: report.success,
            output: report.output,
            duration_ms: report.duration_ms,
            execution_id: report.execution_id,
            provider: report.provider,
            error: None,
        }),
        Err(e) => Ok(ExecutionResult {
            success: false,
            output: String::new(),
            duration_ms: 0,
            execution_id: String::new(),
            provider: String::new(),
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
async fn get_session(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.session_id.clone())
}

#[tauri::command]
async fn get_harnesses(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let runtime = state.runtime.lock().await;
    let names: Vec<String> = runtime.council
        .installed_entries()
        .iter()
        .map(|e| e.id.clone())
        .collect();
    Ok(names)
}

#[tauri::command]
async fn get_genes(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let runtime = state.runtime.lock().await;
    let names: Vec<String> = runtime.council
        .genes
        .iter()
        .map(|g| g.manifest().id.clone())
        .collect();
    Ok(names)
}

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

    let state = AppState {
        runtime: Arc::new(Mutex::new(runtime)),
        session_id,
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            run_task,
            get_session,
            get_harnesses,
            get_genes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Pandora desktop");
}
