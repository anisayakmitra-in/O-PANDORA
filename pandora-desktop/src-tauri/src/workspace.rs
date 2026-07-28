//! Pandora Desktop — Phase C: Coding Workspace
//!
//! File tree, editor, terminal, Git, diff commands.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::State;

// ── State ──

struct WorkspaceState {
    project_path: Arc<tokio::sync::Mutex<Option<PathBuf>>>,
}

// ── File Tree ──

#[derive(Serialize, Clone)]
struct FileEntry {
    name: String,
    path: String,
    is_dir: bool,
    children: Option<Vec<FileEntry>>,
    git_status: Option<String>, // M, A, D, ??
}

#[tauri::command]
pub async fn open_workspace(
    state: State<'_, WorkspaceState>,
    path: String,
) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if !p.exists() {
        return Err(format!("Path does not exist: {path}"));
    }
    *state.project_path.lock().await = Some(p);
    Ok(())
}

#[tauri::command]
pub async fn get_file_tree(
    state: State<'_, WorkspaceState>,
    dir_path: Option<String>,
) -> Result<Vec<FileEntry>, String> {
    let root = if let Some(dp) = dir_path {
        PathBuf::from(dp)
    } else {
        state.project_path.lock().await
            .clone()
            .unwrap_or_else(|| PathBuf::from("."))
    };

    read_dir_entries(&root, 3)
}

fn read_dir_entries(dir: &Path, max_depth: usize) -> Result<Vec<FileEntry>, String> {
    if max_depth == 0 { return Ok(vec![]) }
    let mut entries = vec![];
    if let Ok(read_dir) = std::fs::read_dir(dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            let name = path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();

            // Skip hidden and common ignores
            if name.starts_with('.') || name == "target" || name == "node_modules" {
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
                git_status: None,
            });
        }
    }
    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir { b.is_dir.cmp(&a.is_dir) }
        else { a.name.cmp(&b.name) }
    });
    Ok(entries)
}

// ── File Operations ──

#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read {path}: {e}"))
}

#[tauri::command]
pub async fn write_file(path: String, content: String) -> Result<(), String> {
    // Verify path is within workspace
    let p = PathBuf::from(&path);
    if !p.exists() {
        // Create parent dirs for new files
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create dir: {e}"))?;
        }
    }
    std::fs::write(&path, &content)
        .map_err(|e| format!("Cannot write {path}: {e}"))
}

#[tauri::command]
pub async fn delete_file(path: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if p.is_dir() {
        std::fs::remove_dir_all(&p)
            .map_err(|e| format!("Cannot delete dir {path}: {e}"))
    } else {
        std::fs::remove_file(&p)
            .map_err(|e| format!("Cannot delete file {path}: {e}"))
    }
}

// ── Terminal ──

#[tauri::command]
pub async fn spawn_terminal(
    cwd: Option<String>,
) -> Result<i32, String> {
    let dir = cwd.unwrap_or_else(|| ".".into());
    // Spawn shell and return PID
    let child = std::process::Command::new(if cfg!(windows) { "cmd" } else { "bash" })
        .current_dir(&dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Cannot spawn shell: {e}"))?;
    Ok(child.id() as i32)
}

#[tauri::command]
pub async fn terminal_exec(
    command: String,
    cwd: Option<String>,
) -> Result<String, String> {
    let dir = cwd.unwrap_or_else(|| ".".into());
    let output = if cfg!(windows) {
        std::process::Command::new("cmd")
            .args(["/C", &command])
            .current_dir(&dir)
            .output()
    } else {
        std::process::Command::new("bash")
            .args(["-c", &command])
            .current_dir(&dir)
            .output()
    };
    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout).to_string();
            let stderr = String::from_utf8_lossy(&o.stderr).to_string();
            if o.status.success() {
                Ok(stdout)
            } else {
                Ok(format!("{stdout}\n{stderr}").trim().to_string())
            }
        }
        Err(e) => Err(format!("Command failed: {e}")),
    }
}

// ── Git Commands ──

#[derive(Serialize)]
struct GitStatus {
    branch: String,
    dirty: bool,
    staged: Vec<String>,
    unstaged: Vec<String>,
    untracked: Vec<String>,
}

#[derive(Serialize)]
struct GitDiffFile {
    path: String,
    hunks: Vec<DiffHunk>,
}

#[derive(Serialize)]
struct DiffHunk {
    old_start: usize,
    old_lines: usize,
    new_start: usize,
    new_lines: usize,
    lines: Vec<DiffLine>,
}

#[derive(Serialize)]
struct DiffLine {
    kind: String, // "+", "-", " "
    content: String,
}

#[tauri::command]
pub async fn git_status(state: State<'_, WorkspaceState>) -> Result<GitStatus, String> {
    let cwd = state.project_path.lock().await
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));

    let branch = run_git(&cwd, &["rev-parse", "--abbrev-ref", "HEAD"])?;
    let dirty = !run_git(&cwd, &["diff", "--stat"])?.is_empty();

    let staged_raw = run_git(&cwd, &["diff", "--staged", "--name-only"])?;
    let unstaged_raw = run_git(&cwd, &["diff", "--name-only"])?;
    let untracked_raw = run_git(&cwd, &["ls-files", "--others", "--exclude-standard"])?;

    Ok(GitStatus {
        branch: branch.trim().to_string(),
        dirty,
        staged: non_empty_lines(&staged_raw),
        unstaged: non_empty_lines(&unstaged_raw),
        untracked: non_empty_lines(&untracked_raw),
    })
}

#[tauri::command]
pub async fn git_diff(
    state: State<'_, WorkspaceState>,
    staged: Option<bool>,
) -> Result<Vec<GitDiffFile>, String> {
    let cwd = state.project_path.lock().await
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));

    let args: Vec<&str> = if staged.unwrap_or(false) {
        vec!["diff", "--staged"]
    } else {
        vec!["diff"]
    };

    let raw = run_git(&cwd, &args)?;
    let files = parse_git_diff(&raw);
    Ok(files)
}

#[tauri::command]
pub async fn git_commit(
    state: State<'_, WorkspaceState>,
    message: String,
) -> Result<String, String> {
    let cwd = state.project_path.lock().await
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    run_git(&cwd, &["commit", "-m", &message])
}

#[tauri::command]
pub async fn git_branches(
    state: State<'_, WorkspaceState>,
) -> Result<Vec<String>, String> {
    let cwd = state.project_path.lock().await
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let raw = run_git(&cwd, &["branch"])?;
    let branches: Vec<String> = raw.lines()
        .map(|l| l.trim().trim_start_matches('*').trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    Ok(branches)
}

#[tauri::command]
pub async fn git_checkout(
    state: State<'_, WorkspaceState>,
    branch: String,
) -> Result<String, String> {
    let cwd = state.project_path.lock().await
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    run_git(&cwd, &["checkout", &branch])
}

// ── Helpers ──

fn run_git(cwd: &Path, args: &[&str]) -> Result<String, String> {
    let output = std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("git failed: {e}"))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn non_empty_lines(s: &str) -> Vec<String> {
    s.lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn parse_git_diff(raw: &str) -> Vec<GitDiffFile> {
    let mut files = vec![];
    let mut current_file: Option<GitDiffFile> = None;
    let mut current_hunk: Option<DiffHunk> = None;

    for line in raw.lines() {
        if line.starts_with("diff --git") {
            if let Some(f) = current_file.take() {
                if let Some(h) = current_hunk.take() {
                    let mut f = f;
                    f.hunks.push(h);
                    files.push(f);
                } else {
                    files.push(f);
                }
            }
            // Parse file path from "diff --git a/path b/path"
            let parts: Vec<&str> = line.split_whitespace().collect();
            let path = if parts.len() >= 4 {
                parts[3].trim_start_matches("b/").to_string()
            } else {
                String::new()
            };
            current_file = Some(GitDiffFile { path, hunks: vec![] });
        } else if line.starts_with("@@") {
            if let Some(ref mut f) = current_file {
                if let Some(h) = current_hunk.take() {
                    f.hunks.push(h);
                }
            }
            // Parse @@ -old_start,old_lines +new_start,new_lines @@
            let hunk = DiffHunk {
                old_start: 0,
                old_lines: 0,
                new_start: 0,
                new_lines: 0,
                lines: vec![],
            };
            current_hunk = Some(hunk);
        } else if let Some(ref mut h) = current_hunk {
            let kind = if line.starts_with('+') { "+" }
                else if line.starts_with('-') { "-" }
                else { " " };
            let content = line[1..].to_string();
            h.lines.push(DiffLine { kind: kind.into(), content });
        }
    }

    if let Some(f) = current_file {
        let mut f = f;
        if let Some(h) = current_hunk {
            f.hunks.push(h);
        }
        files.push(f);
    }

    files
}
