use bollard::container::{Config, CreateContainerOptions, RemoveContainerOptions};
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn, error, instrument};

use crate::config::{SandboxConfig, SandboxCommand};
use crate::error::SandboxError;
use crate::engine::SandboxEngine;

pub struct ActiveSandbox {
    engine: SandboxEngine,
    container_id: String,
    cancel_token: CancellationToken,
}

impl ActiveSandbox {
    #[instrument(skip(engine, config))]
    pub async fn provision(
        engine: SandboxEngine,
        config: SandboxConfig,
    ) -> Result<Self, SandboxError> {
        // ... (Mount validation using security.rs goes here) ...

        let container_config = Config {
            image: Some(config.image),
            host_config: Some(crate::security::hardened_host_config(
                config.limits.memory_bytes,
                config.limits.nano_cpus,
            )),
            user: Some("1000:1000".to_string()), // Never run as root
            labels: Some([("pandora.ephemeral", "true")].into_iter().map(|(k,v)| (k.to_string(), v.to_string())).collect()),
            cmd: Some(vec!["sleep".to_string(), "infinity".to_string()]),
            ..Default::default()
        };

        let container = engine.docker.create_container::<&str, String>(None, container_config).await?;
        engine.docker.start_container::<String>(&container.id, None).await?;

        Ok(Self {
            engine,
            container_id: container.id,
            cancel_token: CancellationToken::new(),
        })
    }

    /// Executes a command and streams output via channels, supporting immediate cancellation.
    #[instrument(skip(self, cmd, stdout_tx, stderr_tx))]
    pub async fn execute_streamed(
        &self,
        cmd: SandboxCommand,
        stdout_tx: mpsc::Sender<String>,
        stderr_tx: mpsc::Sender<String>,
    ) -> Result<i64, SandboxError> {
        let exec_options = bollard::exec::CreateExecOptions {
            cmd: Some(cmd.cmd),
            working_dir: Some(cmd.working_dir),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        };

        let exec = self.engine.docker.create_exec(&self.container_id, exec_options).await?;
        let exec_start = self.engine.docker.start_exec(&exec.id, None);

        let stream_processor = async {
            if let bollard::exec::StartExecResults::Attached { mut output, .. } = exec_start.await? {
                while let Some(Ok(msg)) = output.next().await {
                    match msg {
                        bollard::container::LogOutput::StdOut { message } => {
                            let _ = stdout_tx.send(String::from_utf8_lossy(&message).to_string()).await;
                        }
                        bollard::container::LogOutput::StdErr { message } => {
                            let _ = stderr_tx.send(String::from_utf8_lossy(&message).to_string()).await;
                        }
                        _ => {}
                    }
                }
            }
            let inspect = self.engine.docker.inspect_exec(&exec.id).await?;
            Ok::<i64, bollard::errors::Error>(inspect.exit_code.unwrap_or(-1))
        };

        // Timeout and Cancellation multiplexing
        tokio::select! {
            _ = self.cancel_token.cancelled() => {
                warn!("Execution cancelled by runtime.");
                Err(SandboxError::Cancelled)
            }
            _ = tokio::time::sleep(cmd.timeout) => {
                warn!("Execution timed out.");
                Err(SandboxError::Timeout)
            }
            result = stream_processor => {
                result.map_err(|e| SandboxError::ExecutionFailed(e.to_string()))
            }
        }
    }

    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    #[instrument(skip(self))]
    pub async fn teardown(self) {
        let options = RemoveContainerOptions {
            force: true,
            v: true,
            link: false,
        };
        if let Err(e) = self.engine.docker.remove_container(&self.container_id, Some(options)).await {
            error!("Failed to teardown container {}: {}", self.container_id, e);
        } else {
            info!("Successfully destroyed sandbox {}", self.container_id);
        }
    }
}
