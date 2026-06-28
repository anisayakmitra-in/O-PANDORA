use thiserror::Error;

/// Errors that can occur during provider operations.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Provider is not available or not configured
    #[error("provider unavailable: {0}")]
    ProviderUnavailable(String),

    /// Generation failed due to provider error
    #[error("generation failed: {0}")]
    GenerationFailed(String),

    /// Generation was cancelled
    #[error("generation cancelled")]
    Cancelled,

    /// Model not found
    #[error("model not found: {0}")]
    ModelNotFound(String),

    /// Invalid request parameters
    #[error("invalid request: {0}")]
    InvalidRequest(String),

    /// Provider returned an error
    #[error("provider error: {0}")]
    ProviderError(String),

    /// Network/transport error
    #[error("transport error: {0}")]
    TransportError(String),

    /// Serialization/deserialization error
    #[error("serialization error: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, ProviderError>;
