use pandora_governance::router::ExecutionRouter;

use std::sync::Arc;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use pandora_governance::context::ExecutionContext;

use pandora_governance::event::ExecutionEvent;

use pandora_governance::tier::ExecutionTier;

use pandora_sandbox::config::{SandboxCommand, SandboxConfig};

use async_trait::async_trait;

use pandora_governance::audit::AuditEvent;

use pandora_governance::error::GovernanceError;

use pandora_governance::traits::{AuditLogger, BannerManager, ConsentProvider, PolicyEvaluator};

struct TestPolicyEvaluator;

struct TestConsentProvider;

struct TestAuditLogger;

struct TestBannerManager;

#[async_trait]
impl PolicyEvaluator for TestPolicyEvaluator {
    async fn evaluate_elevated(
        &self,

        _config: &SandboxConfig,

        _cmd: &SandboxCommand,
    ) -> Result<(), GovernanceError> {
        Ok(())
    }
}

#[async_trait]
impl ConsentProvider for TestConsentProvider {
    async fn request_sync_consent(&self, _cmd: &SandboxCommand) -> Result<bool, GovernanceError> {
        Ok(true)
    }

    async fn verify_persistent_opt_in(
        &self,

        _tier: &ExecutionTier,
    ) -> Result<String, GovernanceError> {
        Ok(String::from("TEST_OPT_IN"))
    }
}

#[async_trait]
impl AuditLogger for TestAuditLogger {
    async fn log_event(&self, _event: AuditEvent) -> Result<(), GovernanceError> {
        Ok(())
    }
}

#[async_trait]
impl BannerManager for TestBannerManager {
    async fn set_warning_banner(&self, _enabled: bool, _message: &str) {}
}

#[tokio::main]
async fn main() {
    let (event_tx, mut event_rx) = mpsc::channel::<ExecutionEvent>(1024);

    let router = ExecutionRouter {
        policy_evaluator: Arc::new(TestPolicyEvaluator),

        consent_provider: Arc::new(TestConsentProvider),

        audit_logger: Arc::new(TestAuditLogger),

        banner_manager: Arc::new(TestBannerManager),

        kill_switch: pandora_governance::killswitch::RuntimeKillSwitch::new(),
    };

    let context = Arc::new(ExecutionContext {
        trace_id: uuid::Uuid::new_v4(),

        parent_trace_id: None,

        session_id: String::from("test"),

        task_id: None,

        gene_id: None,

        spawned_by: None,

        tier: ExecutionTier::Tier1Isolated,

        cancel_token: CancellationToken::new(),

        timeout: None,
    });

    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            println!("[EVENT] {:?}", event.kind);
        }
    });

    let command = SandboxCommand::new(vec!["echo", "PANDORA_EXECUTION_OK"], 10);

    let result = router.execute(context, command, event_tx).await;

    println!("\nFINAL RESULT: {:?}", result);
}
