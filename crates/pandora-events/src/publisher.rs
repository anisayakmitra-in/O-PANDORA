use std::sync::Arc;

use async_trait::async_trait;

use crate::error::Result;
use crate::types::{DynEvent, EventEnvelope, SubscriptionId};

/// Source of events. Any component that wishes to emit events
/// implements this trait.
#[async_trait]
pub trait Publisher: Send + Sync {
    /// Publish a pre-built event payload. The bus is responsible
    /// for building the envelope (metadata, id, timestamp).
    async fn publish(&self, event: DynEvent) -> Result<SubscriptionId>;

    /// Publish an already-built envelope, preserving its metadata.
    /// This is useful when a caller wants to forward an existing
    /// envelope without re-wrapping it.
    async fn publish_envelope(&self, envelope: EventEnvelope) -> Result<()>;
}

/// A simple closure-based publisher adapter, primarily for tests
/// and for ad-hoc publishers that need to wrap an external
/// component.
pub struct FnPublisher {
    name: String,
    sink: Arc<dyn Fn(EventEnvelope) -> Result<()> + Send + Sync>,
}

impl FnPublisher {
    /// Create a new `FnPublisher` that forwards every envelope to
    /// `sink`.
    pub fn new<F>(name: impl Into<String>, sink: F) -> Self
    where
        F: Fn(EventEnvelope) -> Result<()> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            sink: Arc::new(sink),
        }
    }
}

#[async_trait]
impl Publisher for FnPublisher {
    async fn publish(&self, event: DynEvent) -> Result<SubscriptionId> {
        let envelope = EventEnvelope::new(event, self.name.clone());
        (self.sink)(envelope)?;
        Ok(0)
    }

    async fn publish_envelope(&self, envelope: EventEnvelope) -> Result<()> {
        (self.sink)(envelope)
    }
}
