use std::sync::Arc;

use async_trait::async_trait;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use pandora_governance::context::ExecutionContext;

use pandora_governance::event::ExecutionEvent;

use pandora_governance::killswitch::RuntimeKillSwitch;

use pandora_governance::router::ExecutionRouter;

use pandora_governance::tier::ExecutionTier;

use pandora_governance::traits::{
    AuditLogger,
    BannerManager,
    ConsentProvider,
    PolicyEvaluator,
};

use pandora_sandbox::config::SandboxCommand;

use crate::adapter::{
    AdapterError,
    ExecutionAdapter,
};

use crate::task::{
    Task,
    TaskPayload,
};

pub struct GovernedExecutionAdapter;

impl GovernedExecutionAdapter {

    pub fn new() -> Self {

        Self
    }
}

#[async_trait]
impl ExecutionAdapter
    for GovernedExecutionAdapter {

    async fn execute_task(

        &self,

        task: &Task,

        cancel_token:
            CancellationToken,

        stdout_tx:
            mpsc::Sender<String>,

        stderr_tx:
            mpsc::Sender<String>,

    ) -> Result<i64, AdapterError> {

        let command =
            extract_command(
                &task.payload
            )?;

        let (
            governance_event_tx,
            mut governance_event_rx,
        ) = mpsc::channel::<ExecutionEvent>(1024);

        let router =
            build_router();

        let context =
            std::sync::Arc::new(
                ExecutionContext {

               trace_id:
                   uuid::Uuid::new_v4(),

               parent_trace_id:
                   None,

               session_id:
                   String::from(
                       "scheduler"
                   ),

              task_id:
                   Some(task.id),

              gene_id:
                   None,

              spawned_by:
                   None,

              tier:
                  task.tier.clone(),

              cancel_token,

              timeout:
                  None,
          }  
      ); 

        let stdout_forward =
            stdout_tx.clone();

        let stderr_forward =
            stderr_tx.clone();

        tokio::spawn(async move {

            while let Some(event) =
                governance_event_rx
                    .recv()
                    .await
            {

                match event.kind {

                    pandora_governance
                        ::event
                        ::ExecutionEventKind
                        ::Stdout {

                            line

                        } => {

                            let _ =
                                stdout_forward
                                    .send(line)
                                    .await;
                        }

                    pandora_governance
                        ::event
                        ::ExecutionEventKind
                        ::Stderr {

                            line

                        } => {

                            let _ =
                                stderr_forward
                                    .send(line)
                                    .await;
                        }

                    _ => {}
                }
            }
        });

        router
            .execute(
                context,
                command,
                governance_event_tx,
            )
            .await
            .map_err(|error| {

                AdapterError
                    ::ExecutionFailed(
                        error.to_string()
                    )
            })
    }
}

fn extract_command(

    payload:
        &TaskPayload,

) -> Result<
    SandboxCommand,
    AdapterError,
> {

    match payload {

        TaskPayload
            ::ExecuteCommand {

                command

            } => {

                if command.is_empty() {

                    return Err(
                        AdapterError
                            ::PayloadError(
                                String::from(
                                    "empty command"
                                )
                            )
                    );
                }

                Ok(
                    SandboxCommand {

                        cmd:
                            command.clone(),

                        env:
                            vec![],

                        working_dir:
                            String::from("."),

                        timeout:
                            std::time::Duration
                                ::from_secs(300),
                    }
                )
            }

        _ => Err(
            AdapterError
                ::PayloadError(
                    String::from(
                        "payload not executable"
                    )
                )
        ),
    }
}

fn build_router()
    -> ExecutionRouter {

    struct StubPolicyEvaluator;

    #[async_trait]
    impl PolicyEvaluator
        for StubPolicyEvaluator {

        async fn evaluate_elevated(

            &self,

            _config:
                &pandora_sandbox
                    ::config
                    ::SandboxConfig,

            _cmd:
                &SandboxCommand,

        ) -> Result<
            (),
            pandora_governance
                ::error
                ::GovernanceError,
        > {

            Ok(())
        }
    }

    struct StubConsentProvider;

    #[async_trait]
    impl ConsentProvider
        for StubConsentProvider {

        async fn request_sync_consent(

            &self,

            _cmd:
                &SandboxCommand,

        ) -> Result<
            bool,
            pandora_governance
                ::error
                ::GovernanceError,
        > {

            Ok(true)
        }

        async fn verify_persistent_opt_in(

            &self,

            _tier:
                &ExecutionTier,

        ) -> Result<
            String,
            pandora_governance
                ::error
                ::GovernanceError,
        > {

            Ok(
                String::from(
                    "stub-opt-in"
                )
            )
        }
    }

    struct StubAuditLogger;

    #[async_trait]
    impl AuditLogger
        for StubAuditLogger {

        async fn log_event(

            &self,

            _event:
                pandora_governance
                    ::audit
                    ::AuditEvent,

        ) -> Result<
            (),
            pandora_governance
                ::error
                ::GovernanceError,
        > {

            Ok(())
        }
    }

    struct StubBannerManager;

    #[async_trait]
    impl BannerManager
        for StubBannerManager {

        async fn set_warning_banner(

            &self,

            _active: bool,

            _message: &str,

        ) {
        }
    }

    ExecutionRouter {

        policy_evaluator:
            Arc::new(
                StubPolicyEvaluator
            ),

        consent_provider:
            Arc::new(
                StubConsentProvider
            ),

        audit_logger:
            Arc::new(
                StubAuditLogger
            ),

        banner_manager:
            Arc::new(
                StubBannerManager
            ),

        kill_switch:
            RuntimeKillSwitch::new(),
    }
}

