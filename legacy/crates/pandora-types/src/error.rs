//! Canonical Pandora error type.
//!
//! All public APIs return `Result<T, PandoraError>` instead of
//! `Result<T, String>`. This enables programmatic error handling,
//! structured context, and display-friendly messages.

use std::fmt;

/// The canonical error type for all Pandora operations.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PandoraError {
    NotFound(String),
    AlreadyExists(String),
    Config(String),
    Provider(String),
    Harness(String),
    Gene(String),
    Io(String),
    Validation(String),
    Internal(String),
    Cancelled,
    Connection(String),
    Execution(String),
    Policy(String),
    Permission(String),
    Fleet(String),
    Mcp(String),
}

impl fmt::Display for PandoraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "not found: {msg}"),
            Self::AlreadyExists(msg) => write!(f, "already exists: {msg}"),
            Self::Config(msg) => write!(f, "configuration error: {msg}"),
            Self::Provider(msg) => write!(f, "provider error: {msg}"),
            Self::Harness(msg) => write!(f, "harness error: {msg}"),
            Self::Gene(msg) => write!(f, "gene error: {msg}"),
            Self::Io(msg) => write!(f, "I/O error: {msg}"),
            Self::Validation(msg) => write!(f, "validation error: {msg}"),
            Self::Internal(msg) => write!(f, "internal error: {msg}"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Connection(msg) => write!(f, "connection: {msg}"),
            Self::Execution(msg) => write!(f, "execution: {msg}"),
            Self::Policy(msg) => write!(f, "policy: {msg}"),
            Self::Permission(msg) => write!(f, "permission: {msg}"),
            Self::Fleet(msg) => write!(f, "fleet: {msg}"),
            Self::Mcp(msg) => write!(f, "mcp: {msg}"),
        }
    }
}

impl std::error::Error for PandoraError {}

impl PandoraError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
    pub fn already_exists(msg: impl Into<String>) -> Self {
        Self::AlreadyExists(msg.into())
    }
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }
    pub fn provider(msg: impl Into<String>) -> Self {
        Self::Provider(msg.into())
    }
    pub fn harness(msg: impl Into<String>) -> Self {
        Self::Harness(msg.into())
    }
    pub fn gene(msg: impl Into<String>) -> Self {
        Self::Gene(msg.into())
    }
    pub fn io(msg: impl Into<String>) -> Self {
        Self::Io(msg.into())
    }
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

impl From<String> for PandoraError {
    fn from(msg: String) -> Self {
        Self::Internal(msg)
    }
}

impl From<&str> for PandoraError {
    fn from(msg: &str) -> Self {
        Self::Internal(msg.to_string())
    }
}
