use crate::task::Task;
use thiserror::Error;
use tokio::time::{timeout, Duration};
use tokio_util::sync::CancellationToken;

#[derive(Error, Debug)]
pub enum WatchdogError {
    #[error("Execution timed out (budget breached)")]
    Timeout,
    #[error("Execution cancelled by system")]
    Cancelled,
    #[error("Internal execution error: {0}")]
    Execution(String),
}

pub struct Watchdog;

impl Watchdog {
    /// Wraps the execution payload in a strict temporal budget and cancellation context.
    pub async fn execute_with_budget(
        task: &Task,
        system_cancel: CancellationToken,
    ) -> Result<(), WatchdogError> {
        let duration = Duration::from_secs(task.budget.max_runtime_seconds);
        
        let execution_future = async {
            // Mock execution routing. In reality, this interfaces with `pandora-runtime`
            // and passes `task.tier` and `task.payload` to the Executor.
            tokio::select! {
                _ = system_cancel.cancelled() => {
                    Err(WatchdogError::Cancelled)
                }
                // Simulate work
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    Ok(()) 
                }
            }
        };

        match timeout(duration, execution_future).await {
            Ok(result) => result,
            Err(_) => Err(WatchdogError::Timeout), // tokio::time::timeout throws Elapsed
        }
    }
}
