use chrono::Utc;

use std::collections::BTreeMap;

use std::sync::Arc;

use tokio::sync::{
    mpsc,
    RwLock,
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

use crate::adapter::ExecutionAdapter;

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

    adapter:
        Arc<dyn ExecutionAdapter>,
}

impl SchedulerKernel {

    pub fn new(
        store: TaskStore,
        event_tx:
            mpsc::Sender<SchedulerEvent>,
        cancel_token:
            CancellationToken,
        adapter:
            Arc<dyn ExecutionAdapter>,
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

            adapter,
        }
    }

    pub async fn boot(
        &self,
    ) -> Result<
        (),
        Box<dyn std::error::Error>
    > {

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

        for (
            id,
            mut task
        ) in tasks {

            if task.status ==
                TaskStatus::Running
            {

                task.status =
                    TaskStatus::Pending;

                store
                    .update_task(
                        &task
                    )
                    .await?;

                warn!(
                    task_id = %id,
                    "recovered interrupted task"
                );
            }

            if task.status ==
                TaskStatus::Pending
            {

                queue
                    .entry(
                        task.next_run
                    )
                    .or_default()
                    .push(id);
            }
        }

        info!(
            "scheduler boot complete"
        );

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn run_heartbeat_loop(
        self: Arc<Self>,
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
                            .process_due_tasks()
                            .await
                    {

                        error!(
                            error = %error,
                            "heartbeat processing failure"
                        );
                    }
                }

                _ = self
                    .cancel_token
                    .cancelled() =>
                {

                    info!(
                        "scheduler heartbeat shutdown"
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
    ) -> Result<
        (),
        Box<dyn std::error::Error>
    > {

        {
            let mut store =
                self.store.write().await;

            store
                .update_task(
                    &task
                )
                .await?;
        }

        let _ =
            self.event_tx
                .send(
                    SchedulerEvent
                        ::TaskQueued(
                            task.id
                        )
                )
                .await;

        {
            let mut queue =
                self.queue.write().await;

            queue
                .entry(
                    task.next_run
                )
                .or_default()
                .push(task.id);
        }

        Ok(())
    }

    async fn process_due_tasks(
        &self,
    ) -> Result<
        (),
        Box<dyn std::error::Error>
    > {

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

            for (
                _,
                mut task_ids
            ) in future_tasks {

                to_execute.append(
                    &mut task_ids
                );
            }
        }

        for task_id in to_execute {

            let kernel =
                Arc::new(
                    self.clone_inner()
                );

            tokio::spawn(async move {

                kernel
                    .dispatch_task(
                        task_id
                    )
                    .await;
            });
        }

        Ok(())
    }

    fn clone_inner(
        &self,
    ) -> Self {

        Self {

            store:
                self.store.clone(),

            queue:
                self.queue.clone(),

            event_tx:
                self.event_tx.clone(),

            cancel_token:
                self.cancel_token.clone(),

            adapter:
                self.adapter.clone(),
        }
    }

    async fn dispatch_task(
        &self,
        task_id: Uuid,
    ) {

        let mut task = {

            let store =
                self.store.read().await;

            match store.get_task(
                &task_id
            ) {

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
            .update_task_state(
                &task
            )
            .await;

        let _ =
            self.event_tx
                .send(
                    SchedulerEvent
                        ::TaskStarted(
                            task.id
                        )
                )
                .await;

        let (
            stdout_tx,
            mut stdout_rx
        ) = mpsc::channel(1024);

        let (
            stderr_tx,
            mut stderr_rx
        ) = mpsc::channel(1024);

        let event_tx =
            self.event_tx.clone();

        let task_id_clone =
            task.id;

        tokio::spawn(async move {

            loop {

                tokio::select! {

                    Some(stdout) =
                        stdout_rx.recv() =>
                    {

                        let _ =
                            event_tx
                                .send(
                                    SchedulerEvent
                                        ::TaskOutput {

                                            id:
                                                task_id_clone,

                                            stream:
                                                "stdout"
                                                    .into(),

                                            data:
                                                stdout,
                                        }
                                )
                                .await;
                    }

                    Some(stderr) =
                        stderr_rx.recv() =>
                    {

                        let _ =
                            event_tx
                                .send(
                                    SchedulerEvent
                                        ::TaskOutput {

                                            id:
                                                task_id_clone,

                                            stream:
                                                "stderr"
                                                    .into(),

                                            data:
                                                stderr,
                                        }
                                )
                                .await;
                    }

                    else => break,
                }
            }
        });

        let adapter =
            self.adapter.clone();

        let cancel_token =
            self.cancel_token.clone();

        let execution_future =
            adapter.execute_task(
                &task,
                cancel_token,
                stdout_tx,
                stderr_tx,
            );

        let result =
            Watchdog
                ::enforce_temporal_budget(
                    &task,
                    execution_future,
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
                    .update_task_state(
                        &task
                    )
                    .await;

                let _ =
                    self.event_tx
                        .send(
                            SchedulerEvent
                                ::TaskCompleted(
                                    task.id
                                )
                        )
                        .await;
            }

            Err(error) => {

                warn!(
                    task_id = %task.id,
                    error = %error,
                    "task execution failed"
                );

                crate::retry
                    ::apply_retry_policy(
                        &mut task
                    );

                self
                    .update_task_state(
                        &task
                    )
                    .await;

                let _ =
                    self.event_tx
                        .send(
                            SchedulerEvent
                                ::TaskFailed(
                                    task.id
                                )
                        )
                        .await;
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
                error = %error,
                "failed persisting task"
            );
        }
    }
}
