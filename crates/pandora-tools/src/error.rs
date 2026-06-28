use thiserror::Error;

/// Errors that can occur during tool execution, registration, or lookup.
#[derive(Debug, Error)]
pub enum ToolError {
    /// Tool is not registered in the registry.
    #[error("tool not found: {0}")]
    NotFound(String),

    /// Tool input failed validation against the tool's declared schema.
    #[error("invalid tool input: {0}")]
    InvalidInput(String),

    /// Tool execution failed.
    #[error("tool execution failed: {0}")]
    ExecutionFailed(String),

    /// Tool is not permitted to perform the requested action.
    #[error("tool permission denied: {0}")]
    PermissionDenied(String),

    /// Tool execution was cancelled.
    #[error("tool execution cancelled")]
    Cancelled,

    /// Underlying I/O or transport error.
    #[error("tool I/O error: {0}")]
    Io(String),
}

/// Result alias for tool operations.
pub type Result<T> = std::result::Result<T, ToolError>;
