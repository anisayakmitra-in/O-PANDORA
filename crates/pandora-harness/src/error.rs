use thiserror::Error;

/// Errors that can occur in harness operations.
#[derive(Debug, Error)]
pub enum HarnessError {
    /// Harness not found in registry.
    #[error("harness not found: {0}")]
    NotFound(String),

    /// Duplicate harness registration attempted.
    #[error("harness already registered: {0}")]
    AlreadyRegistered(String),

    /// Invalid harness manifest.
    #[error("invalid manifest: {0}")]
    InvalidManifest(String),

    /// Harness initialization failed.
    #[error("initialization failed: {0}")]
    InitializationFailed(String),

    /// Harness shutdown failed.
    #[error("shutdown failed: {0}")]
    ShutdownFailed(String),

    /// Role mismatch for operation.
    #[error("role mismatch: expected {expected:?}, found {found:?}")]
    RoleMismatch { expected: crate::roles::HarnessRole, found: crate::roles::HarnessRole },

    /// Dependency resolution failed.
    #[error("dependency resolution failed: {0}")]
    DependencyResolutionFailed(String),
}

/// Result type for harness operations.
pub type Result<T> = std::result::Result<T, HarnessError>;
