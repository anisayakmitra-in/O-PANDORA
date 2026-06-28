//! Built-in `web_scrape` tool.
//!
//! ⚠️ NOT IMPLEMENTED. The previous `WebScrapeTool` was a pure mock
//! that only printed a string. It is preserved here as a real
//! [`Tool`] implementation shell so the registry can advertise the
//! capability, but [`execute`] returns
//! [`ToolError::ExecutionFailed`] until a network-fetch backend is
//! added in a future change.

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::capability::ToolCapability;
use crate::error::{Result, ToolError};
use crate::manifest::{ToolManifest, ToolMode};
use crate::traits::Tool;
use crate::types::{ToolInput, ToolOutput};

/// Built-in `web_scrape` tool.
pub struct WebScrapeTool;

impl Default for WebScrapeTool {
    fn default() -> Self {
        Self::new()
    }
}

impl WebScrapeTool {
    /// Create a new `WebScrapeTool`.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for WebScrapeTool {
    fn manifest(&self) -> ToolManifest {
        ToolManifest::new("net.web.scrape", "Web Scrape", "0.1.0")
            .with_description("Fetch and extract text from a remote URL.")
            .with_input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string" }
                },
                "required": ["url"]
            }))
            .with_output_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "content": { "type": "string" }
                },
                "required": ["content"]
            }))
            .with_mode(ToolMode::Sandboxed)
            .with_capabilities(
                crate::capability::ToolCapabilitySet::new()
                    .with(ToolCapability::required("network.http")),
            )
    }

    async fn execute(&self, _input: ToolInput, cancel: CancellationToken) -> Result<ToolOutput> {
        if cancel.is_cancelled() {
            return Err(ToolError::Cancelled);
        }
        Err(ToolError::ExecutionFailed(
            "web_scrape is not yet implemented; see builtin/web_scrape.rs".into(),
        ))
    }
}
