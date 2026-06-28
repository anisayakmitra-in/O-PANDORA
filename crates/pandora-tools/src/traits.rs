use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::error::Result;
use crate::manifest::ToolManifest;
use crate::types::{ToolInput, ToolMetadata, ToolOutput};

/// Core tool trait.
///
/// A tool is an asynchronous, cancellable unit of work that:
///
/// 1. Declares its identity, schema, and capabilities via
///    [`ToolManifest`].
/// 2. Validates incoming [`ToolInput`] against its declared schema.
/// 3. Executes and returns a structured [`ToolOutput`].
///
/// All tools are `Send + Sync` so they can be stored in a registry
/// and shared across tasks. Implementations may be Native Rust
/// (default), Sandboxed, or Remote — see [`ToolMode`](crate::manifest::ToolMode).
#[async_trait]
pub trait Tool: Send + Sync {
    /// Manifest describing this tool.
    fn manifest(&self) -> ToolManifest;

    /// Execute the tool with the given structured input.
    ///
    /// Implementations are expected to:
    /// 1. Validate `input` against `manifest().input_schema`.
    /// 2. Honor `cancel` — return [`ToolError::Cancelled`] if the
    ///    token is cancelled mid-execution.
    /// 3. Return a structured [`ToolOutput`] on success.
    async fn execute(&self, input: ToolInput, cancel: CancellationToken) -> Result<ToolOutput>;

    /// Validate input without executing.
    ///
    /// The default implementation accepts any input; tools with
    /// strict schemas should override this.
    async fn validate(&self, _input: &ToolInput) -> Result<()> {
        Ok(())
    }

    /// Return free-form metadata describing this tool instance.
    ///
    /// The default returns an empty map; tools can override to
    /// expose implementation-specific details (build info, stats,
    /// configuration, etc.).
    fn metadata(&self) -> ToolMetadata {
        ToolMetadata::new()
    }
}
