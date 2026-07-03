//! Pandora Docker Sandbox — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTask {
    pub image: String,

    pub command: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxResult {
    pub success: bool,

    pub stdout: String,

    pub stderr: String,
}

pub struct DockerSandboxEngine;

impl DockerSandboxEngine {
    pub async fn execute(task: &SandboxTask) -> SandboxResult {
        println!("[SANDBOX] launching {}", task.image);

        let mut args = vec!["run".to_string(), "--rm".to_string(), task.image.clone()];

        args.extend(task.command.clone());

        let output = Command::new("docker").args(&args).output().await;

        match output {
            Ok(result) => SandboxResult {
                success: result.status.success(),

                stdout: String::from_utf8_lossy(&result.stdout).to_string(),

                stderr: String::from_utf8_lossy(&result.stderr).to_string(),
            },

            Err(error) => SandboxResult {
                success: false,

                stdout: String::new(),

                stderr: error.to_string(),
            },
        }
    }
}
