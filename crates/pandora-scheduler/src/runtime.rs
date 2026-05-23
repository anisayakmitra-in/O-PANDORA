use std::sync::Arc;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use tracing::info;

use crate::adapter::ExecutionAdapter;

use crate::executor::GovernedExecutionAdapter;

use crate::persistence::TaskStore;

use crate::scheduler::SchedulerKernel;

pub struct SchedulerRuntime {
    kernel: Arc<SchedulerKernel>,

    cancel_token: CancellationToken,
}

impl SchedulerRuntime {
    pub async fn bootstrap(
        persistence_path: impl Into<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let persistence_path = persistence_path.into();

        let (event_tx, mut event_rx) = mpsc::channel(4096);

        let cancel_token = CancellationToken::new();

        let store = TaskStore::new(persistence_path);

        let adapter: Arc<dyn ExecutionAdapter> = Arc::new(GovernedExecutionAdapter::new());

        let kernel = Arc::new(SchedulerKernel::new(
            store,
            event_tx,
            cancel_token.clone(),
            adapter,
        ));

        kernel.boot().await?;

        tokio::spawn(async move {
            while let Some(event) = event_rx.recv().await {
                info!(
                    event = ?event,
                    "scheduler event"
                );
            }
        });

        Ok(Self {
            kernel,

            cancel_token,
        })
    }

    pub async fn start(&self) {
        let kernel = self.kernel.clone();

        tokio::spawn(async move {
            kernel.run_heartbeat_loop().await;
        });

        info!("scheduler runtime started");
    }

    pub async fn shutdown(&self) {
        self.cancel_token.cancel();

        info!("scheduler runtime shutdown requested");
    }

    pub fn kernel(&self) -> Arc<SchedulerKernel> {
        self.kernel.clone()
    }
}
