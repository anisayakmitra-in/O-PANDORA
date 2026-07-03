//! Pandora Execution Kernel — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTask {
    pub task_id: String,

    pub command: String,

    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub task_id: String,

    pub success: bool,

    pub stdout: String,

    pub stderr: String,
}

pub struct ExecutionKernel;

impl ExecutionKernel {
    pub async fn execute(task: &ExecutionTask) -> ExecutionResult {
        println!("[KERNEL] executing {}", task.task_id);

        let output = Command::new(&task.command).args(&task.args).output().await;

        match output {
            Ok(result) => ExecutionResult {
                task_id: task.task_id.clone(),

                success: result.status.success(),

                stdout: String::from_utf8_lossy(&result.stdout).to_string(),

                stderr: String::from_utf8_lossy(&result.stderr).to_string(),
            },

            Err(error) => ExecutionResult {
                task_id: task.task_id.clone(),

                success: false,

                stdout: String::new(),

                stderr: error.to_string(),
            },
        }
    }
}
