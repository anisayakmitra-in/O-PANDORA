use std::sync::Arc;

use async_trait::async_trait;

use crate::error::Result;
use crate::filter::EventFilter;
use crate::types::{EventEnvelope, SubscriberId};

/// Asynchronous subscriber to events flowing through the bus.
///
/// Implementors are typically small adapters that forward events
/// to a downstream system (a logger, a memory store, a metric
/// collector, a governance auditor, ...). The contract is
/// deliberately minimal so any consumer can plug in.
#[async_trait]
pub trait Subscriber: Send + Sync {
    /// Stable subscriber id. Return the same id for the same logical
    /// subscriber across multiple bus attachments.
    fn id(&self) -> SubscriberId;

    /// Human-readable name (used for diagnostics).
    fn name(&self) -> &str;

    /// Receive an envelope that already passed the subscriber's
    /// filter. Returning `Err` does not propagate to the bus;
    /// subscribers are expected to handle their own internal errors
    /// gracefully and only return `Err` for unrecoverable situations.
    async fn on_event(&self, envelope: &EventEnvelope) -> Result<()>;
}

/// A registered subscription: a subscriber + its filter.
pub struct Subscription {
    /// The actual subscriber.
    pub subscriber: Arc<dyn Subscriber>,

    /// The filter applied before delivery.
    pub filter: Arc<dyn EventFilter>,
}

impl Subscription {
    /// Build a subscription that delivers every event to `subscriber`.
    pub fn unfiltered(subscriber: Arc<dyn Subscriber>) -> Self {
        Self {
            subscriber,
            filter: Arc::new(crate::filter::AcceptAll),
        }
    }

    /// Build a subscription that uses the given filter.
    pub fn with_filter(subscriber: Arc<dyn Subscriber>, filter: Arc<dyn EventFilter>) -> Self {
        Self { subscriber, filter }
    }
}
