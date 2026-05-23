use async_trait::async_trait;

use pandora_sandbox::config::{SandboxCommand, SandboxConfig};

use crate::audit::AuditEvent;

use crate::error::GovernanceError;

use crate::tier::ExecutionTier;

#[async_trait]
pub trait PolicyEvaluator: Send + Sync {
    async fn evaluate_elevated(
        &self,

        config: &SandboxConfig,

        cmd: &SandboxCommand,
    ) -> Result<(), GovernanceError>;
}

#[async_trait]
pub trait ConsentProvider: Send + Sync {
    async fn request_sync_consent(&self, cmd: &SandboxCommand) -> Result<bool, GovernanceError>;

    async fn verify_persistent_opt_in(
        &self,

        tier: &ExecutionTier,
    ) -> Result<String, GovernanceError>;
}

#[async_trait]
pub trait AuditLogger: Send + Sync {
    async fn log_event(&self, event: AuditEvent) -> Result<(), GovernanceError>;
}

#[async_trait]
pub trait BannerManager: Send + Sync {
    async fn set_warning_banner(&self, active: bool, message: &str);
}
