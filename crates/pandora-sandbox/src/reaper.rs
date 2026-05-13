use bollard::query_parameters::RemoveContainerOptions;

use bollard::Docker;

use std::collections::HashSet;

use std::sync::Arc;

use tokio::sync::RwLock;

use tokio::time::{
    interval,
    Duration,
};

use tracing::{
    error,
    info,
    instrument,
    warn,
};

#[derive(Clone)]
pub struct ContainerReaper {

    docker:
        Docker,

    tracked_containers:
        Arc<
            RwLock<
                HashSet<String>
            >
        >,
}

impl ContainerReaper {

    pub fn new(
        docker: Docker
    ) -> Self {

        Self {

            docker,

            tracked_containers:

                Arc::new(
                    RwLock::new(
                        HashSet::new()
                    )
                ),
        }
    }

    pub async fn track(

        &self,

        container_id:
            String,

    ) {

        let mut tracking =

            self
                .tracked_containers
                .write()
                .await;

        tracking.insert(
            container_id
        );
    }

    pub async fn untrack(

        &self,

        container_id:
            &str,

    ) {

        let mut tracking =

            self
                .tracked_containers
                .write()
                .await;

        tracking.remove(
            container_id
        );
    }

    pub fn spawn_reaper_task(
        &self
    ) {

        let reaper =
            self.clone();

        tokio::spawn(
            async move {

                let mut ticker =

                    interval(
                        Duration::from_secs(
                            120
                        )
                    );

                loop {

                    ticker
                        .tick()
                        .await;

                    reaper
                        .reap_orphans()
                        .await;
                }
            }
        );
    }

    #[instrument(skip(self))]
    async fn reap_orphans(
        &self
    ) {

        let tracked = {

            let lock =

                self
                    .tracked_containers
                    .read()
                    .await;

            lock.clone()
        };

        if tracked.is_empty() {

            return;
        }

        info!(
            "reaper analyzing {} tracked containers",
            tracked.len()
        );

        for id in tracked {

            let options =

                RemoveContainerOptions {

                    force:
                        true,

                    v:
                        true,

                    link:
                        false,
                };

            match self
                .docker
                .remove_container(
                    &id,
                    Some(options),
                )
                .await
            {

                Ok(_) => {

                    warn!(
                        container_id = %id,
                        "reaper destroyed orphaned container"
                    );

                    self
                        .untrack(
                            &id
                        )
                        .await;
                }

                Err(error_value) => {

                    error!(
                        container_id = %id,
                        error = %error_value,
                        "reaper failed to remove container"
                    );
                }
            }
        }
    }
}
