use thiserror::Error;

/// Errors that can occur in the storage / persistence layer.
#[derive(Debug, Error)]
pub enum StorageError {
    /// Backend is not available (e.g. connection refused).
    #[error("backend unavailable: {0}")]
    BackendUnavailable(String),

    /// Record was not found in the underlying store.
    #[error("not found: {0}")]
    NotFound(String),

    /// The record already exists (uniqueness violation).
    #[error("already exists: {0}")]
    AlreadyExists(String),

    /// Serialization or deserialization failure.
    #[error("serialization error: {0}")]
    Serialization(String),

    /// Underlying I/O failure.
    #[error("I/O error: {0}")]
    Io(String),

    /// The current transaction has been aborted.
    #[error("transaction aborted: {0}")]
    TransactionAborted(String),

    /// Snapshot operation failed.
    #[error("snapshot error: {0}")]
    Snapshot(String),

    /// Requested feature is not supported by the active backend.
    #[error("not supported by this backend: {0}")]
    Unsupported(String),

    /// Catch-all for unexpected backend errors.
    #[error("storage error: {0}")]
    Other(String),
}

/// Result alias for storage operations.
pub type Result<T> = std::result::Result<T, StorageError>;
