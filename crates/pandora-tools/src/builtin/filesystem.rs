//! Built-in `read_file` tool implementation.

use std::fs;
use std::path::Path;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::capability::ToolCapability;
use crate::error::{Result, ToolError};
use crate::manifest::{ToolManifest, ToolMode};
use crate::permission::ToolPermission;
use crate::traits::Tool;
use crate::types::{ToolInput, ToolOutput};

/// Read the contents of a file at the given path.
///
/// Returns the file contents as a `String` on success, or a
/// `ToolError::Io` with the underlying error message on failure.
pub fn read_file(path: impl AsRef<Path>) -> Result<String> {
    fs::read_to_string(path.as_ref()).map_err(|e| ToolError::Io(e.to_string()))
}

/// Built-in tool that reads a file from disk.
///
/// Expects a `{"path": "..."}` JSON input. Returns a
/// `{"content": "..."}` JSON output. Cancellable.
pub struct ReadFileTool;

impl Default for ReadFileTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadFileTool {
    /// Create a new `ReadFileTool`.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for ReadFileTool {
    fn manifest(&self) -> ToolManifest {
        ToolManifest::new("fs.read", "Read File", "0.1.0")
            .with_description("Read the contents of a UTF-8 text file from disk.")
            .with_input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"]
            }))
            .with_output_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "content": { "type": "string" }
                },
                "required": ["content"]
            }))
            .with_mode(ToolMode::Native)
            .with_capabilities(
                crate::capability::ToolCapabilitySet::new()
                    .with(ToolCapability::required("filesystem.read")),
            )
            .with_permissions(
                crate::permission::ToolPermissionSet::new()
                    .with(ToolPermission::new("fs.read.path")),
            )
    }

    async fn execute(&self, input: ToolInput, cancel: CancellationToken) -> Result<ToolOutput> {
        if cancel.is_cancelled() {
            return Err(ToolError::Cancelled);
        }

        let path = input
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ToolError::InvalidInput("missing string field 'path'".into()))?;

        // Cancellation check is best-effort before a blocking read.
        if cancel.is_cancelled() {
            return Err(ToolError::Cancelled);
        }

        let content = read_file(path)?;
        Ok(serde_json::json!({ "content": content }))
    }
}
