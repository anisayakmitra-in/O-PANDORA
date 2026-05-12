use std::sync::Arc;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use tracing::{
    error,
    info,
};

use crate::event::SchedulerEvent;

use crate::persistence::TaskStore;

use crate::scheduler::SchedulerKernel;

use crate::task::{
    ExecutionTier,
    Task,
    TaskPayload,
};

pub struct SchedulerRuntime {

    kernel:
        Arc<SchedulerKernel>,
}

impl SchedulerRuntime {

    pub async fn new(
        store_path: impl Into<std::path::PathBuf>,
    ) -> Result<Self, Box<dyn std::error::Error>> {

        let (event_tx, mut event_rx) =
            mpsc::channel::<SchedulerEvent>(1024);

        let store =
            TaskStore::new(
                store_path.into()
            );

        let kernel =
            Arc::new(
                SchedulerKernel::new(
                    store,
                    event_tx,
                    CancellationToken::new(),
                )
            );

        kernel
            .boot()
            .await?;

        let heartbeat =
            kernel.clone();

        tokio::spawn(async move {

            heartbeat
                .run_heartbeat_loop()
                .await;
        });

        tokio::spawn(async move {

            while let Some(event) =
                event_rx.recv().await
            {

                info!(
                    "scheduler event: {:?}",
                    event
                );
            }
        });

        Ok(
            Self {
                kernel
            }
        )
    }

    pub async fn enqueue_immediate(
        &self,
        tier: ExecutionTier,
        payload: TaskPayload,
    ) -> Result<(), Box<dyn std::error::Error>> {

        let task =
            Task::new(
                tier,
                payload,
            );

        self.kernel
            .enqueue(task)
            .await
    }

    pub async fn enqueue_delayed(
        &self,
        tier: ExecutionTier,
        payload: TaskPayload,
        delay_seconds: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {

        let task =
            Task::new(
                tier,
                payload,
            )
            .with_delay(
                delay_seconds
            );

        self.kernel
            .enqueue(task)
            .await
    }
}
