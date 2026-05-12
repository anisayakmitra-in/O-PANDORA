use chrono::Utc;

use std::collections::BTreeMap;
use std::sync::Arc;

use tokio::sync::{
    mpsc,
    RwLock,
    Semaphore,
};

use tokio::time::{
    interval,
    Duration,
};

use tokio_util::sync::CancellationToken;

use tracing::{
    error,
    info,
    instrument,
    warn,
};

use uuid::Uuid;

use crate::event::SchedulerEvent;

use crate::persistence::TaskStore;

use crate::task::{
    Task,
    TaskStatus,
};

use crate::watchdog::Watchdog;

pub struct SchedulerKernel {

    store:
        Arc<RwLock<TaskStore>>,

    queue:
        Arc<
            RwLock<
                BTreeMap<
                    chrono::DateTime<Utc>,
                    Vec<Uuid>
                >
            >
        >,

    event_tx:
        mpsc::Sender<SchedulerEvent>,

    cancel_token:
        CancellationToken,

    worker_limit:
        Arc<Semaphore>,
}

impl SchedulerKernel {

    pub fn new(
        store: TaskStore,
        event_tx: mpsc::Sender<SchedulerEvent>,
        cancel_token: CancellationToken,
    ) -> Self {

        Self {

            store:
                Arc::new(
                    RwLock::new(store)
                ),

            queue:
                Arc::new(
                    RwLock::new(
                        BTreeMap::new()
                    )
                ),

            event_tx,

            cancel_token,

            worker_limit:
                Arc::new(
                    Semaphore::new(32)
                ),
        }
    }

    #[instrument(skip(self))]
    pub async fn boot(
        &self,
    ) -> Result<(), Box<dyn std::error::Error>> {

        let mut store =
            self.store.write().await;

        store
            .load_and_compact()
            .await?;

        let tasks =
            store
                .get_all_tasks()
                .clone();

        let mut queue =
            self.queue.write().await;

        for (id, mut task) in tasks {

            if task.status ==
                TaskStatus::Running
            {

                task.status =
                    TaskStatus::Pending;

                store
                    .update_task(&task)
                    .await?;

                warn!(
                    task_id = %id,
                    "Recovered interrupted task"
                );
            }

            if task.status ==
                TaskStatus::Pending
            {

                queue
                    .entry(task.next_run)
                    .or_default()
                    .push(id);
            }
        }

        info!(
            "Scheduler Kernel booted"
        );

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn run_heartbeat_loop(
        self: Arc<Self>
    ) {

        let mut ticker =
            interval(
                Duration::from_millis(500)
            );

        loop {

            tokio::select! {

                _ = ticker.tick() => {

                    if let Err(error) =
                        self
                            .clone()
                            .process_due_tasks()
                            .await
                    {

                        error!(
                            "heartbeat failure: {}",
                            error
                        );
                    }
                }

                _ = self
                    .cancel_token
                    .cancelled() =>
                {

                    info!(
                        "scheduler shutdown"
                    );

                    break;
                }
            }
        }
    }

    #[instrument(skip(self, task))]
    pub async fn enqueue(
        &self,
        task: Task,
    ) -> Result<(), Box<dyn std::error::Error>> {

        {
            let mut store =
                self.store.write().await;

            store
                .update_task(&task)
                .await?;
        }

        let _ =
            self
                .event_tx
                .send(
                    SchedulerEvent::TaskQueued(
                        task.id
                    )
                )
                .await;

        {
            let mut queue =
                self.queue.write().await;

            queue
                .entry(task.next_run)
                .or_default()
                .push(task.id);
        }

        Ok(())
    }

    async fn process_due_tasks(
        self: Arc<Self>
    ) -> Result<(), Box<dyn std::error::Error>> {

        let now =
            Utc::now();

        let mut to_execute =
            Vec::new();

        {
            let mut queue =
                self.queue.write().await;

            let mut future_tasks =
                queue.split_off(&now);

            std::mem::swap(
                &mut *queue,
                &mut future_tasks,
            );

            let due_tasks =
                future_tasks;

            for (_, mut task_ids)
                in due_tasks
            {

                to_execute
                    .append(
                        &mut task_ids
                    );
            }
        }

        for task_id in to_execute {

            let kernel =
                self.clone();

            let permit =
                match kernel
                    .worker_limit
                    .clone()
                    .acquire_owned()
                    .await
            {

                Ok(permit) => permit,

                Err(error) => {

                    error!(
                        "failed to acquire permit: {}",
                        error
                    );

                    return Ok(());
                }
            };

            tokio::spawn(async move {

                let _permit =
                    permit;

                kernel
                    .dispatch_task(task_id)
                    .await;
            });
        }

        Ok(())
    }

    async fn dispatch_task(
        &self,
        task_id: Uuid,
    ) {

        let mut task = {

            let store =
                self.store.read().await;

            match store
                .get_task(&task_id)
            {

                Some(task) =>
                    task.clone(),

                None =>
                    return,
            }
        };

        task.status =
            TaskStatus::Running;

        task.attempts += 1;

        self
            .update_task_state(&task)
            .await;

        let _ =
            self
                .event_tx
                .send(
                    SchedulerEvent::TaskStarted(
                        task.id
                    )
                )
                .await;

        let result =
            Watchdog::execute_with_budget(
                &task,
                self.cancel_token.clone(),
            )
            .await;

        match result {

            Ok(_) => {

                task.invocations += 1;

                task.attempts = 0;

                crate::retry
                    ::calculate_next_recurrence(
                        &mut task
                    );

                self
                    .update_task_state(&task)
                    .await;

                let _ =
                    self
                        .event_tx
                        .send(
                            SchedulerEvent::TaskCompleted(
                                task.id
                            )
                        )
                        .await;
            }

            Err(
                crate::watchdog
                    ::WatchdogError::Timeout
            ) => {

                error!(
                    task_id = %task.id,
                    "task timeout"
                );

                task.status =
                    TaskStatus::Failed;

                self
                    .update_task_state(&task)
                    .await;

                let _ =
                    self
                        .event_tx
                        .send(
                            SchedulerEvent
                                ::WatchdogTriggered(
                                    task.id
                                )
                        )
                        .await;
            }

            Err(error) => {

                warn!(
                    task_id = %task.id,
                    error = %error,
                    "task failed"
                );

                crate::retry
                    ::apply_retry_policy(
                        &mut task
                    );

                self
                    .update_task_state(&task)
                    .await;

                if task.status ==
                    TaskStatus::Failed
                {

                    let _ =
                        self
                            .event_tx
                            .send(
                                SchedulerEvent::TaskFailed(
                                    task.id
                                )
                            )
                            .await;
                }
                else {

                    let _ =
                        self
                            .event_tx
                            .send(
                                SchedulerEvent::TaskRetried(
                                    task.id
                                )
                            )
                            .await;

                    let mut queue =
                        self.queue.write().await;

                    queue
                        .entry(task.next_run)
                        .or_default()
                        .push(task.id);
                }
            }
        }
    }

    async fn update_task_state(
        &self,
        task: &Task,
    ) {

        let mut store =
            self.store.write().await;

        if let Err(error) =
            store
                .update_task(task)
                .await
        {

            error!(
                "failed to persist task: {}",
                error
            );
        }
    }
}
