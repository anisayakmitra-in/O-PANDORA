//! Built-in `shell` tool.
//!
//! ⚠️ NOT IMPLEMENTED. The previous `ShellTool` was a pure mock that
//! only printed a string. It is preserved here as a real
//! [`Tool`] implementation shell so the registry can advertise the
//! capability, but [`execute`] returns
//! [`ToolError::ExecutionFailed`] until a sandboxed execution
//! backend is added in a future change. Real shell execution will
//! require a sandboxing layer that is out of scope for this
//! contract crate.

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::capability::ToolCapability;
use crate::error::{Result, ToolError};
use crate::manifest::{ToolManifest, ToolMode};
use crate::permission::ToolPermission;
use crate::traits::Tool;
use crate::types::{ToolInput, ToolOutput};

/// Built-in `shell` tool.
pub struct ShellTool;

impl Default for ShellTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellTool {
    /// Create a new `ShellTool`.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for ShellTool {
    fn manifest(&self) -> ToolManifest {
        ToolManifest::new("os.shell", "Shell", "0.1.0")
            .with_description("Execute a shell command in a sandboxed environment.")
            .with_input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string" }
                },
                "required": ["command"]
            }))
            .with_output_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "stdout": { "type": "string" },
                    "stderr": { "type": "string" },
                    "exit_code": { "type": "integer" }
                },
                "required": ["stdout", "stderr", "exit_code"]
            }))
            .with_mode(ToolMode::Sandboxed)
            .with_capabilities(
                crate::capability::ToolCapabilitySet::new()
                    .with(ToolCapability::required("process.exec")),
            )
            .with_permissions(
                crate::permission::ToolPermissionSet::new()
                    .with(ToolPermission::new("os.shell.exec")),
            )
    }

    async fn execute(&self, _input: ToolInput, cancel: CancellationToken) -> Result<ToolOutput> {
        if cancel.is_cancelled() {
            return Err(ToolError::Cancelled);
        }
        Err(ToolError::ExecutionFailed(
            "shell execution is not yet implemented; see builtin/shell.rs".into(),
        ))
    }
}
