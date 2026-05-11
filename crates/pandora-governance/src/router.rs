use std::sync::Arc;

use tokio::sync::mpsc;

use tracing::{
    info,
    instrument,
    warn,
};

use pandora_sandbox::config::{
    SandboxCommand,
    SandboxConfig,
};

use crate::context::ExecutionContext;

use crate::event::{
    ExecutionEvent,
    ExecutionEventKind,
};

use crate::killswitch::RuntimeKillSwitch;

use crate::tier::ExecutionTier;

use crate::traits::{
    AuditLogger,
    BannerManager,
    ConsentProvider,
    PolicyEvaluator,
};

use crate::error::GovernanceError;

pub struct ExecutionRouter {

    pub policy_evaluator:
        Arc<dyn PolicyEvaluator>,

    pub consent_provider:
        Arc<dyn ConsentProvider>,

    pub audit_logger:
        Arc<dyn AuditLogger>,

    pub banner_manager:
        Arc<dyn BannerManager>,

    pub kill_switch:
        RuntimeKillSwitch,
}

impl ExecutionRouter {

    #[instrument(skip(
        self,
        ctx,
        cmd,
        event_tx,
    ))]
    pub async fn execute(

        &self,

        ctx:
            Arc<ExecutionContext>,

        cmd:
            SandboxCommand,

        event_tx:
            mpsc::Sender<ExecutionEvent>,

    ) -> Result<i64, GovernanceError> {

        if ctx.is_cancelled() {

            return Err(
                GovernanceError::SystemHalted
            );
        }

        event_tx
            .send(
                ExecutionEvent::new(
                    ctx.clone(),
                    ExecutionEventKind::Started,
                )
            )
            .await
            .ok();

        match &ctx.tier {

            ExecutionTier::IsolatedSandbox => {

                info!(
                    "Tier 1 execution approved"
                );
            }

            ExecutionTier::GovernedElevated(
                config
            ) => {

                self.policy_evaluator
                    .evaluate_elevated(
                        config,
                        &cmd,
                    )
                    .await?;
            }

            ExecutionTier::HostUnrestricted => {

                let approved =
                    self.consent_provider
                        .request_sync_consent(
                            &cmd
                        )
                        .await?;

                if !approved {

                    event_tx
                        .send(
                            ExecutionEvent::new(
                                ctx.clone(),
                                ExecutionEventKind::GovernanceDenied {
                                    reason:
                                        String::from(
                                            "User denied execution"
                                        ),
                                },
                            )
                        )
                        .await
                        .ok();

                    return Err(
                        GovernanceError::ConsentDenied
                    );
                }
            }

            ExecutionTier::AutonomousOperator { .. } => {

                self.banner_manager
                    .set_warning_banner(
                        true,
                        "AUTONOMOUS OPERATOR ACTIVE",
                    )
                    .await;
            }

            ExecutionTier::UnboundedExecution { .. } => {

                self.banner_manager
                    .set_warning_banner(
                        true,
                        "WARNING: UNBOUNDED EXECUTION ACTIVE",
                    )
                    .await;
            }
        }

        let result =
            if ctx.tier.is_host_execution() {

                self.dispatch_host(
                    ctx.clone(),
                    cmd,
                    event_tx.clone(),
                )
                .await

            } else {

                let config =
                    match &ctx.tier {

                        ExecutionTier::GovernedElevated(
                            cfg
                        ) => cfg.clone(),

                        _ => SandboxConfig::default(),
                    };

                self.dispatch_sandbox(
                    ctx.clone(),
                    config,
                    cmd,
                    event_tx.clone(),
                )
                .await
            };

        match &result {

            Ok(exit_code) => {

                event_tx
                    .send(
                        ExecutionEvent::new(
                            ctx.clone(),
                            ExecutionEventKind::Finished {
                                exit_code:
                                    *exit_code,
                            },
                        )
                    )
                    .await
                    .ok();
            }

            Err(error) => {

                warn!(
                    "Execution failed: {:?}",
                    error
                );
            }
        }

        result
    }

    async fn dispatch_sandbox(

        &self,

        ctx:
            Arc<ExecutionContext>,

        _config:
            SandboxConfig,

        _cmd:
            SandboxCommand,

        event_tx:
            mpsc::Sender<ExecutionEvent>,

    ) -> Result<i64, GovernanceError> {

        event_tx
            .send(
                ExecutionEvent::new(
                    ctx.clone(),
                    ExecutionEventKind::Stdout {
                        line: String::from(
                            "sandbox execution placeholder started"
                        ),
                    },
                )
            )
            .await
            .ok();

        event_tx
            .send(
                ExecutionEvent::new(
                    ctx.clone(),
                    ExecutionEventKind::Finished {
                        exit_code: 0,
                    },
                )
            )
            .await
            .ok();

        Ok(0)
    }

    async fn dispatch_host(

        &self,

        ctx:
            Arc<ExecutionContext>,

        _cmd:
            SandboxCommand,

        event_tx:
            mpsc::Sender<ExecutionEvent>,

    ) -> Result<i64, GovernanceError> {

        event_tx
            .send(
                ExecutionEvent::new(
                    ctx.clone(),
                    ExecutionEventKind::HostExecutionStarted,
                )
            )
            .await
            .ok();

        event_tx
            .send(
                ExecutionEvent::new(
                    ctx.clone(),
                    ExecutionEventKind::Stdout {
                        line: String::from(
                            "host execution placeholder started"
                        ),
                    },
                )
            )
            .await
            .ok();

        event_tx
            .send(
                ExecutionEvent::new(
                    ctx.clone(),
                    ExecutionEventKind::Finished {
                        exit_code: 0,
                    },
                )
            )
            .await
            .ok();

        Ok(0)
}
}
