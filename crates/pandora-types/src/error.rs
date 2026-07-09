//! Canonical Pandora error type.
//!
//! All public APIs return `Result<T, PandoraError>` instead of `Result<T, String>`.
//! This enables programmatic error handling, structured context, and display-friendly messages.

use std::fmt;

/// The canonical error type for all Pandora operations.
#[derive(Debug, Clone)]
pub enum PandoraError {
    /// Resource not found (gene, harness, capability, provider, etc.)
    NotFound(String),
    /// Resource already exists (duplicate registration)
    AlreadyExists(String),
    /// Configuration error (missing env var, bad config file, etc.)
    Config(String),
    /// Provider error (LLM API failure, network error, etc.)
    Provider(String),
    /// Harness execution error
    Harness(String),
    /// Gene execution error
    Gene(String),
    /// Filesystem I/O error
    Io(String),
    /// Input validation error
    Validation(String),
    /// Internal/unexpected error
    Internal(String),
}

impl fmt::Display for PandoraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PandoraError::NotFound(msg) => write!(f, "not found: {}", msg),
            PandoraError::AlreadyExists(msg) => write!(f, "already exists: {}", msg),
            PandoraError::Config(msg) => write!(f, "configuration error: {}", msg),
            PandoraError::Provider(msg) => write!(f, "provider error: {}", msg),
            PandoraError::Harness(msg) => write!(f, "harness error: {}", msg),
            PandoraError::Gene(msg) => write!(f, "gene error: {}", msg),
            PandoraError::Io(msg) => write!(f, "I/O error: {}", msg),
            PandoraError::Validation(msg) => write!(f, "validation error: {}", msg),
            PandoraError::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for PandoraError {}

// Helper constructors
impl PandoraError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        PandoraError::NotFound(msg.into())
    }
    pub fn already_exists(msg: impl Into<String>) -> Self {
        PandoraError::AlreadyExists(msg.into())
    }
    pub fn config(msg: impl Into<String>) -> Self {
        PandoraError::Config(msg.into())
    }
    pub fn provider(msg: impl Into<String>) -> Self {
        PandoraError::Provider(msg.into())
    }
    pub fn harness(msg: impl Into<String>) -> Self {
        PandoraError::Harness(msg.into())
    }
    pub fn gene(msg: impl Into<String>) -> Self {
        PandoraError::Gene(msg.into())
    }
    pub fn io(msg: impl Into<String>) -> Self {
        PandoraError::Io(msg.into())
    }
    pub fn validation(msg: impl Into<String>) -> Self {
        PandoraError::Validation(msg.into())
    }
    pub fn internal(msg: impl Into<String>) -> Self {
        PandoraError::Internal(msg.into())
    }
}

impl From<String> for PandoraError {
    fn from(msg: String) -> Self {
        PandoraError::Internal(msg)
    }
}

impl From<&str> for PandoraError {
    fn from(msg: &str) -> Self {
        PandoraError::Internal(msg.to_string())
    }
}
