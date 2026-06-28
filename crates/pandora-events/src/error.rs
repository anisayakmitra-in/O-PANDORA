use thiserror::Error;

/// Errors that can occur in the event system.
#[derive(Debug, Error)]
pub enum EventError {
    /// Subscriber is not registered.
    #[error("subscriber not found: {0}")]
    SubscriberNotFound(String),

    /// Event bus is full and the event was dropped.
    #[error("event bus full, dropped event {0}")]
    BusFull(String),

    /// Subscriber channel is closed.
    #[error("subscriber channel closed: {0}")]
    ChannelClosed(String),

    /// Filter rejected the event (used by ask/tell semantics).
    #[error("event filtered: {0}")]
    Filtered(String),

    /// Serialization or deserialization failure.
    #[error("serialization error: {0}")]
    Serialization(String),

    /// Event envelope is malformed.
    #[error("invalid event: {0}")]
    InvalidEvent(String),

    /// Catch-all for unexpected runtime errors.
    #[error("event system error: {0}")]
    Other(String),
}

/// Result alias for event operations.
pub type Result<T> = std::result::Result<T, EventError>;
