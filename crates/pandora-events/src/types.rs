use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::category::EventCategory;
use crate::error::EventError;
use crate::metadata::EventMetadata;
use crate::priority::EventPriority;
use crate::traits::Event;

/// Identifier for an event subscriber.
pub type SubscriberId = u64;

/// Identifier for an event subscription.
pub type SubscriptionId = u64;

/// Type-erased, shareable event handle.
///
/// Buses and subscribers hold `Arc<dyn Event>` rather than concrete
/// types so the same channel can carry events from every system in
/// the workspace.
pub type DynEvent = Arc<dyn Event>;

/// Newtype wrapper that adds `Serialize` + `Deserialize` to any
/// `Event` implementation.
///
/// Use this when an event needs to cross a process boundary or be
/// persisted. The inner value must itself be `Serialize +
/// DeserializeOwned + 'static`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableEvent<E>(pub E);

impl<E> SerializableEvent<E>
where
    E: Event + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    /// Wrap a concrete event in a serializable envelope.
    pub fn new(event: E) -> Self {
        Self(event)
    }

    /// Consume the wrapper and return the inner event.
    pub fn into_inner(self) -> E {
        self.0
    }

    /// Borrow the inner event.
    pub fn inner(&self) -> &E {
        &self.0
    }
}

/// Serialize any `Serialize` event value to a JSON string.
pub fn serialize_event<E: Serialize>(event: &E) -> Result<String, EventError> {
    serde_json::to_string(event).map_err(|e| EventError::Serialization(e.to_string()))
}

/// Deserialize a JSON string into the given event type.
pub fn deserialize_event<E: for<'de> Deserialize<'de>>(json: &str) -> Result<E, EventError> {
    serde_json::from_str(json).map_err(|e| EventError::Serialization(e.to_string()))
}

/// Envelope wrapping an event with its metadata.
///
/// The envelope is what actually flows through the bus; it carries
/// the typed event as `Arc<dyn Event>` together with the metadata
/// needed to route, order, and trace the event.
#[derive(Debug, Clone)]
pub struct EventEnvelope {
    /// The event payload, type-erased.
    pub event: DynEvent,

    /// Metadata for routing, ordering, and tracing.
    pub metadata: EventMetadata,
}

impl EventEnvelope {
    /// Wrap an event in an envelope with a generated metadata
    /// record. The metadata is taken from `event.metadata(source)`.
    pub fn new(event: DynEvent, source: impl Into<String>) -> Self {
        let source = source.into();
        let metadata = event.metadata(&source);
        Self { event, metadata }
    }

    /// Build an envelope with an explicit metadata record.
    pub fn with_metadata(event: DynEvent, metadata: EventMetadata) -> Self {
        Self { event, metadata }
    }

    /// The event id from the metadata.
    pub fn id(&self) -> &str {
        &self.metadata.id
    }

    /// The event name.
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// The event category.
    pub fn category(&self) -> EventCategory {
        self.metadata.category
    }

    /// The event priority.
    pub fn priority(&self) -> EventPriority {
        self.metadata.priority
    }

    /// Downcast the inner event to a concrete type, if it is one.
    pub fn downcast_ref<T: Event>(&self) -> Option<&T> {
        self.event.as_any().downcast_ref::<T>()
    }
}

/// Convenience: wrap a concrete event value in `Arc<dyn Event>`.
pub fn dyn_event<E: Event + 'static>(event: E) -> DynEvent {
    Arc::new(event)
}
