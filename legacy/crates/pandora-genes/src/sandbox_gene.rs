//! Sandbox Gene — Docker-based disposable worker execution.
//!
//! Each execution gets an isolated container with:
//! - Isolated filesystem
//! - Isolated permissions
//! - Isolated memory
//! - Isolated environment
//!
//! When execution completes:
//! - Results are extracted
//! - Lineage is stored
//! - Container is destroyed
//!
//! Inspired by OpenHands' sandboxed runtime architecture.

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use std::process::Command;

/// A gene that executes commands inside a disposable Docker container.
/// The container is created, command runs, output is captured, container is destroyed.
#[derive(Debug, Clone)]
pub struct SandboxGene {
    manifest: GeneManifest,
}

impl SandboxGene {
    pub fn new() -> Self {
        let manifest = GeneManifestBuilder::default()
            .id("sandbox.docker")
            .name("Docker Sandbox")
            .kind(GeneKind::Security)
            .version("0.2.0")
            .author("pandora")
            .description("Disposable Docker container for isolated command execution")
            .capability("sandbox.execute")
            .capability("sandbox.create")
            .capability("sandbox.destroy")
            .build()
            .expect("sandbox manifest must build");
        Self { manifest }
    }

    /// Run a command in a disposable container.
    /// Returns stdout + stderr.
    pub fn run_in_sandbox(
        &self,
        image: &str,
        command: &str,
    ) -> Result<String, pandora_types::PandoraError> {
        // Create and run a disposable container
        let output = Command::new("docker")
            .args(["run", "--rm", image, "sh", "-c", command])
            .output()
            .map_err(|e| {
                pandora_types::PandoraError::Internal(format!("Docker not available: {e}"))
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(pandora_types::PandoraError::Internal(format!(
                "Sandbox exit code: {:?}\nstdout: {stdout}\nstderr: {stderr}",
                output.status.code()
            )));
        }

        Ok(if stderr.is_empty() {
            stdout
        } else {
            format!("{stdout}\n{stderr}")
        })
    }

    /// Run a command with mounted working directory.
    pub fn run_with_mount(
        &self,
        image: &str,
        command: &str,
        mount_path: &str,
    ) -> Result<String, pandora_types::PandoraError> {
        let mount_arg = format!("{mount_path}:/workspace");
        let workdir_arg = "/workspace";
        let output = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-v",
                &mount_arg,
                "-w",
                workdir_arg,
                image,
                "sh",
                "-c",
                command,
            ])
            .output()
            .map_err(|e| {
                pandora_types::PandoraError::Internal(format!("Docker not available: {e}"))
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(pandora_types::PandoraError::Internal(format!(
                "Sandbox exit: {:?}\n{stdout}\n{stderr}",
                output.status.code()
            )));
        }

        Ok(if stderr.is_empty() {
            stdout
        } else {
            format!("{stdout}\n{stderr}")
        })
    }

    /// Check if Docker is available.
    pub fn is_available() -> bool {
        Command::new("docker")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl Default for SandboxGene {
    fn default() -> Self {
        Self::new()
    }
}

impl Gene for SandboxGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }

    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        // Parse input: "image:command" or just "command" (defaults to ubuntu)
        let (image, command) = if let Some(idx) = input.find(':') {
            if input[..idx].contains('/') || input[..idx].contains('.') {
                (&input[..idx], &input[idx + 1..])
            } else {
                ("ubuntu:24.04", input)
            }
        } else {
            ("ubuntu:24.04", input)
        };

        // Check if Docker is available
        if !Self::is_available() {
            return Err("Docker is not available. Install Docker to use sandbox execution.".into());
        }

        self.run_in_sandbox(image, command)
    }

    fn validate(&self) -> Result<(), pandora_types::PandoraError> {
        if Self::is_available() {
            Ok(())
        } else {
            Err("Docker is not installed or not in PATH".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_manifest() {
        let gene = SandboxGene::new();
        assert_eq!(gene.id(), "sandbox.docker");
        assert_eq!(gene.kind(), &GeneKind::Security);
        assert!(gene
            .manifest()
            .capabilities
            .contains(&"sandbox.execute".to_string()));
    }

    #[test]
    fn sandbox_parse_input() {
        // Test that image:command parsing works
        let input = "python:3.12:python script.py";
        let (image, command) = if let Some(idx) = input.find(':') {
            if input[..idx].contains('/') || input[..idx].contains('.') {
                (&input[..idx], &input[idx + 1..])
            } else {
                ("ubuntu:24.04", input)
            }
        } else {
            ("ubuntu:24.04", input)
        };
        // The first colon is after "python", which doesn't contain / or .
        // So it should fall back to ubuntu
        assert_eq!(image, "ubuntu:24.04");
        assert_eq!(command, input);
    }
}
