use std::sync::Arc;

use tokio::process::Command;

use tokio::sync::mpsc;

use tracing::{
    error,
    info,
    instrument,
    warn,
};

use crate::audit::AuditEvent;

use crate::context::ExecutionContext;

use crate::error::GovernanceError;

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

use pandora_sandbox::config::{
    SandboxCommand,
    SandboxConfig,
};

use pandora_sandbox::reaper::ContainerReaper;

use pandora_sandbox::sandbox::ActiveSandbox;

use bollard::Docker;

#[derive(Clone)]
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

    #[instrument(
        skip(
            self,
            ctx,
            cmd,
            event_tx
        )
    )]
    pub async fn execute(

        &self,

        ctx:
            Arc<ExecutionContext>,

        cmd:
            SandboxCommand,

        event_tx:
            mpsc::Sender<ExecutionEvent>,

    ) -> Result<
        i64,
        GovernanceError,
    > {

        if self
            .kill_switch
            .is_triggered()
        {

            return Err(
                GovernanceError
                    ::SystemHalted
            );
        }

        if ctx.is_cancelled() {

            return Err(
                GovernanceError
                    ::SystemHalted
            );
        }

        self
            .emit_event(
                &event_tx,
                ctx.clone(),
                ExecutionEventKind::ExecutionStarted {
                    command:
                        cmd.cmd.clone(),
                },
            )
            .await;

        match &ctx.tier {

            ExecutionTier
                ::Tier1Isolated => {

                info!(
                    "tier1 isolated execution"
                );
            }

            ExecutionTier
                ::Tier2Governed {
                    ..
                } => {

                let config =
                    SandboxConfig
                        ::default();

                self
                    .policy_evaluator
                    .evaluate_elevated(
                        &config,
                        &cmd,
                    )
                    .await?;
            }

            ExecutionTier
                ::Tier3Host => {

                let approved =
                    self
                        .consent_provider
                        .request_sync_consent(
                            &cmd
                        )
                        .await?;

                if !approved {

                    return Err(
                        GovernanceError
                            ::ConsentDenied
                    );
                }
            }

            ExecutionTier
                ::Tier4Autonomous {
                    ..
                } => {

                self
                    .banner_manager
                    .set_warning_banner(
                        true,
                        "AUTONOMOUS OPERATOR ACTIVE",
                    )
                    .await;
            }

            ExecutionTier
                ::Tier5Unbounded {
                    ..
                } => {

                warn!(
                    "tier5 unbounded execution active"
                );

                self
                    .banner_manager
                    .set_warning_banner(
                        true,
                        "WARNING: UNBOUNDED EXECUTION ACTIVE",
                    )
                    .await;
            }
        }

        self
            .log_intent(
                ctx.clone(),
                &cmd,
                "dispatch",
            )
            .await;

        let result =

            if ctx
                .tier
                .is_host_execution()
            {

                self
                    .dispatch_host(
                        ctx.clone(),
                        cmd,
                        event_tx.clone(),
                    )
                    .await

            } else {

                self
                    .dispatch_sandbox(
                        ctx.clone(),
                        SandboxConfig::default(),
                        cmd,
                        event_tx.clone(),
                    )
                    .await
            };

        match &result {

            Ok(exit_code) => {

                self
                    .emit_event(
                        &event_tx,
                        ctx.clone(),
                        ExecutionEventKind::ExecutionCompleted {
                            exit_code:
                                *exit_code,
                        },
                    )
                    .await;
            }

            Err(error_value) => {

                self
                    .emit_event(
                        &event_tx,
                        ctx.clone(),
                        ExecutionEventKind::ExecutionFailed {
                            reason:
                                error_value
                                    .to_string(),
                        },
                    )
                    .await;
            }
        }

        result
    }

    async fn dispatch_host(

        &self,

        ctx:
            Arc<ExecutionContext>,

        cmd:
            SandboxCommand,

        event_tx:
            mpsc::Sender<ExecutionEvent>,

    ) -> Result<
        i64,
        GovernanceError,
    > {

        let mut command =
            Command::new(
                &cmd.cmd[0]
            );

        if cmd.cmd.len() > 1 {

            command.args(
                &cmd.cmd[1..]
            );
        }

        command.kill_on_drop(true);

        let output =
            command
                .output()
                .await
                .map_err(
                    |error| {

                        GovernanceError
                            ::ExecutionFailed(
                                error
                                    .to_string()
                            )
                    }
                )?;

        let stdout =
            String::from_utf8_lossy(
                &output.stdout
            );

        for line in stdout.lines() {

            self
                .emit_event(
                    &event_tx,
                    ctx.clone(),
                    ExecutionEventKind::Stdout {
                        line:
                            line
                                .to_string(),
                    },
                )
                .await;
        }

        let stderr =
            String::from_utf8_lossy(
                &output.stderr
            );

        for line in stderr.lines() {

            self
                .emit_event(
                    &event_tx,
                    ctx.clone(),
                    ExecutionEventKind::Stderr {
                        line:
                            line
                                .to_string(),
                    },
                )
                .await;
        }

        Ok(
            output
                .status
                .code()
                .unwrap_or(-1)
                as i64
        )
    }

    async fn dispatch_sandbox(

        &self,

        ctx:
            Arc<ExecutionContext>,

        config:
            SandboxConfig,

        cmd:
            SandboxCommand,

        event_tx:
            mpsc::Sender<ExecutionEvent>,

   ) -> Result<
       i64,
       GovernanceError,
   > {

       let docker =

           Docker
               ::connect_with_local_defaults()
               .map_err(
                   |error| {

                       GovernanceError
                           ::ExecutionFailed(
                               format!(
                                   "docker init failed: {}",
                                   error
                               )
                           )
                   }
             )?;

     let reaper =
         ContainerReaper::new(
             docker.clone()
         );

     let sandbox =

         ActiveSandbox
             ::provision(
                 docker,
                 reaper,
                 config,
             )
             .await
             .map_err(
                 |error| {

                     GovernanceError
                         ::ExecutionFailed(
                             format!(
                                 "sandbox provision failed: {}",
                                 error
                             )
                         )
                 }
             )?;

     self
         .emit_event(
             &event_tx,
             ctx.clone(),
             ExecutionEventKind
                 ::SandboxExecutionStarted {

                 command:
                     cmd.cmd.clone(),
             },
         )
         .await;

     let (
         stdout_tx,
         mut stdout_rx,
     ) =
         mpsc::channel::<String>(
             1024
         );

     let (
         stderr_tx,
         mut stderr_rx,
     ) =
         mpsc::channel::<String>(
             1024
         );

    let stdout_events =
        event_tx.clone();

    let stderr_events =
        event_tx.clone();

    let stdout_ctx =
        ctx.clone();

    let stderr_ctx =
        ctx.clone();

    tokio::spawn(
        async move {

            while let Some(line) =

                stdout_rx
                    .recv()
                    .await

            {

                let event =
                    ExecutionEvent::new(

                        stdout_ctx.clone(),

                        ExecutionEventKind
                            ::Stdout {

                            line,
                        },
                    );

                let _ =
                    stdout_events
                        .send(event)
                        .await;
            }
        }
    );

    tokio::spawn(
        async move {

            while let Some(line) =

                stderr_rx
                    .recv()
                    .await

            {

                let event =
                    ExecutionEvent::new(

                        stderr_ctx.clone(),

                        ExecutionEventKind
                            ::Stderr {

                            line,
                        },
                    );

                let _ =
                    stderr_events
                        .send(event)
                        .await;
            }
        }
    );

    let execution_result =

        sandbox
            .execute_streamed(

                cmd,

                ctx
                    .cancel_token
                    .clone(),

                stdout_tx,

                stderr_tx,
            )
            .await;

    sandbox
        .teardown()
        .await;

    match execution_result {

        Ok(exit_code) => {

            Ok(exit_code)
        }

        Err(error_value) => {

            Err(
                GovernanceError
                    ::ExecutionFailed(
                        error_value
                            .to_string()
                    )
            )
        }
    }
}

    async fn emit_event(

        &self,

        event_tx:
            &mpsc::Sender<ExecutionEvent>,

        ctx:
            Arc<ExecutionContext>,

        kind:
            ExecutionEventKind,

    ) {

        let event =
            ExecutionEvent::new(
                ctx,
                kind,
            );

        let _ =
            event_tx
                .send(event)
                .await;
    }

    async fn log_intent(

        &self,

        ctx:
            Arc<ExecutionContext>,

        cmd:
            &SandboxCommand,

        outcome:
            &str,

    ) {

        let event =
            AuditEvent {

                timestamp:
                    std::time::SystemTime
                        ::now(),

                trace_id:
                    ctx.trace_id
                        .to_string(),

                tier:
                    format!(
                        "{:?}",
                        ctx.tier
                    ),

                command:
                    cmd.cmd.clone(),

                environment:

                    if ctx
                        .tier
                        .is_host_execution()
                    {

                        String::from(
                            "host"
                        )

                    } else {

                        String::from(
                            "sandbox"
                        )
                    },

                operator_id:
                    None,

                outcome:
                    outcome
                        .to_string(),
            };

        if let Err(error_value) =

            self
                .audit_logger
                .log_event(event)
                .await

        {

            error!(
                "audit logging failure: {}",
                error_value
            );
        }
    }
}
