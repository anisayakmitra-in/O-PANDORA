use async_trait::async_trait;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use thiserror::Error;

use crate::task::Task;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    #[error("payload error: {0}")]
    PayloadError(String),
}

#[async_trait]
pub trait ExecutionAdapter: Send + Sync {
    async fn execute_task(
        &self,

        task: &Task,

        cancel_token: CancellationToken,

        stdout_tx: mpsc::Sender<String>,

        stderr_tx: mpsc::Sender<String>,
    ) -> Result<i64, AdapterError>;
}
