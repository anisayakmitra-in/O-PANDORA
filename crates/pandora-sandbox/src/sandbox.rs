use crate::config::{
    SandboxCommand,
    SandboxConfig,
};

use crate::error::SandboxError;

use crate::reaper::ContainerReaper;

use bollard::container::LogOutput;

use bollard::exec::{
    CreateExecOptions,
    StartExecResults,
};

use bollard::models::{
    ContainerCreateBody,
    HostConfig,
};

use bollard::query_parameters::{
    CreateContainerOptions,
    RemoveContainerOptions,
    StartContainerOptions,
};

use bollard::Docker;

use futures_util::StreamExt;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use tracing::{
    error,
    info,
    instrument,
};

pub struct ActiveSandbox {

    docker:
        Docker,

    reaper:
        ContainerReaper,

    pub container_id:
        String,
}

impl ActiveSandbox {

    #[instrument(skip(docker, reaper, config))]
    pub async fn provision(

        docker:
            Docker,

        reaper:
            ContainerReaper,

        config:
            SandboxConfig,

    ) -> Result<Self, SandboxError> {

        let host_config =
            HostConfig {

                memory:
                    Some(
                        config
                            .limits
                            .memory_bytes
                    ),

                nano_cpus:
                    Some(
                        config
                            .limits
                            .nano_cpus
                    ),

                readonly_rootfs:
                    Some(
                        config
                            .readonly_rootfs
                    ),

                cap_drop:
                    Some(
                        config
                            .drop_capabilities
                    ),

                network_mode:

                    if config.network_disabled {

                        Some(
                            String::from(
                                "none"
                            )
                        )

                    } else {

                        None
                    },

                ..Default::default()
            };

        let container_config =
            ContainerCreateBody {

                image:
                    Some(
                        config.image
                    ),

                user:
                    Some(
                        config.user_namespace
                    ),

                tty:
                    Some(false),

                cmd:
                    Some(
                        vec![
                            String::from(
                                "sleep"
                            ),
                            String::from(
                                "infinity"
                            ),
                        ]
                    ),

                host_config:
                    Some(
                        host_config
                    ),

                ..Default::default()
            };

        let created =
            docker
                .create_container(
                    None::<CreateContainerOptions>,
                    container_config,
                )
                .await
                .map_err(
                    |error| {

                        SandboxError
                            ::InitFailed(
                                error
                                    .to_string()
                            )
                    }
                )?;

        docker
            .start_container(
                &created.id,
                None::<StartContainerOptions>,
            )
            .await
            .map_err(
                |error| {

                    SandboxError
                        ::InitFailed(
                            error
                                .to_string()
                        )
                }
            )?;

        reaper
            .track(
                created.id.clone()
            )
            .await;

        info!(
            "sandbox provisioned: {}",
            created.id
        );

        Ok(
            Self {

                docker,

                reaper,

                container_id:
                    created.id,
            }
        )
    }

    #[instrument(skip(self, cmd, cancel_token, stdout_tx, stderr_tx))]
    pub async fn execute_streamed(

        &self,

        cmd:
            SandboxCommand,

        cancel_token:
            CancellationToken,

        stdout_tx:
            mpsc::Sender<String>,

        stderr_tx:
            mpsc::Sender<String>,

    ) -> Result<i64, SandboxError> {

        let exec =
            self
                .docker
                .create_exec(

                    &self.container_id,

                    CreateExecOptions {

                        cmd:
                            Some(
                                cmd.cmd
                            ),

                        attach_stdout:
                            Some(true),

                        attach_stderr:
                            Some(true),

                        working_dir:
                            Some(
                                cmd.working_dir
                            ),

                        ..Default::default()
                    },
                )
                .await
                .map_err(
                    |error| {

                        SandboxError
                            ::ExecutionFailed(
                                error
                                    .to_string()
                            )
                    }
                )?;

        let exec_id =
            exec.id.clone();

        let docker =
            self.docker.clone();

        let execution_future =
            async move {

                let result =
                    docker
                        .start_exec(
                            &exec_id,
                            None,
                        )
                        .await
                        .map_err(
                            |error| {

                                SandboxError
                                    ::ExecutionFailed(
                                        error
                                            .to_string()
                                    )
                            }
                        )?;

                match result {

                    StartExecResults::Attached {

                        mut output,

                        ..

                    } => {

                        while let Some(message) =
                            output.next().await
                        {

                            match message {

                                Ok(
                                    LogOutput::StdOut {

                                        message,

                                    }
                                ) => {

                                    let line =
                                        String::from_utf8_lossy(
                                            &message
                                        )
                                        .to_string();

                                    let _ =
                                        stdout_tx
                                            .send(line)
                                            .await;
                                }

                                Ok(
                                    LogOutput::StdErr {

                                        message,

                                    }
                                ) => {

                                    let line =
                                        String::from_utf8_lossy(
                                            &message
                                        )
                                        .to_string();

                                    let _ =
                                        stderr_tx
                                            .send(line)
                                            .await;
                                }

                                _ => {}
                            }
                        }
                    }

                    _ => {}
                }

                let inspect =
                    docker
                        .inspect_exec(
                            &exec_id
                        )
                        .await
                        .map_err(
                            |error| {

                                SandboxError
                                    ::ExecutionFailed(
                                        error
                                            .to_string()
                                    )
                            }
                        )?;

                Ok(
                    inspect
                        .exit_code
                        .unwrap_or(-1)
                )
            };

        tokio::select! {

            _ = cancel_token.cancelled() => {

                Err(
                    SandboxError
                        ::Cancelled
                )
            }

            _ = tokio::time::sleep(
                cmd.timeout
            ) => {

                Err(
                    SandboxError
                        ::Timeout(
                            cmd
                                .timeout
                                .as_secs()
                        )
                )
            }

            result = execution_future => {

                result
            }
        }
    }

    #[instrument(skip(self))]
    pub async fn teardown(
        self
    ) {

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
                &self.container_id,
                Some(options),
            )
            .await
        {

            Ok(_) => {

                self
                    .reaper
                    .untrack(
                        &self.container_id
                    )
                    .await;

                info!(
                    "sandbox destroyed: {}",
                    self.container_id
                );
            }

            Err(error_value) => {

                error!(
                    "sandbox teardown failed: {}",
                    error_value
                );
            }
        }
    }
}
