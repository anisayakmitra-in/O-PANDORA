use async_trait::async_trait;

use crate::config::{
    SandboxCommand,
    SandboxConfig,
};

use crate::error::SandboxError;

#[async_trait]
pub trait ExecutionAuthorizer:
    Send
    + Sync
{
    async fn authorize_command(
        &self,
        config: &SandboxConfig,
        cmd: &SandboxCommand,
    )
        -> Result<(), SandboxError>;
}
