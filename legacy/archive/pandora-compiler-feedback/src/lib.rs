//! Pandora Compiler Feedback — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationTask {
    pub task_id: String,

    pub command: String,

    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    pub success: bool,

    pub stdout: String,

    pub stderr: String,
}

pub struct CompilerFeedbackEngine;

impl CompilerFeedbackEngine {
    pub async fn validate(task: &CompilationTask) -> CompilationResult {
        println!("[COMPILER] validating {}", task.task_id);

        let output = Command::new(&task.command).args(&task.args).output().await;

        match output {
            Ok(result) => CompilationResult {
                success: result.status.success(),

                stdout: String::from_utf8_lossy(&result.stdout).to_string(),

                stderr: String::from_utf8_lossy(&result.stderr).to_string(),
            },

            Err(error) => CompilationResult {
                success: false,

                stdout: String::new(),

                stderr: error.to_string(),
            },
        }
    }
}
