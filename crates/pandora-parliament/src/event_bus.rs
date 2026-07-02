use std::fmt::Debug;
use tokio::sync::broadcast;
use uuid::Uuid;

/// A constitutional event. Every event has a unique id,
/// a type string, and optional payload data.
#[derive(Debug, Clone)]
pub struct Event {
    pub event_id: String,
    pub event_type: String,
    pub source: String,
    pub payload: Option<String>,
}

impl Event {
    pub fn new(event_type: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            event_id: Uuid::new_v4().to_string(),
            event_type: event_type.into(),
            source: source.into(),
            payload: None,
        }
    }

    pub fn with_payload(mut self, payload: impl Into<String>) -> Self {
        self.payload = Some(payload.into());
        self
    }
}

/// Errors from the Event Bus.
#[derive(Debug, thiserror::Error)]
pub enum EventBusError {
    #[error("failed to publish event: {0}")]
    PublishFailed(String),
    #[error("failed to subscribe: {0}")]
    SubscribeFailed(String),
}

/// The constitutional Event Bus.
///
/// All inter-harness communication goes through this bus.
/// No harness talks directly to another harness.
/// Components publish events; subscribers react.
///
/// Uses `tokio::sync::broadcast` for fan-out delivery.
pub struct EventBus {
    tx: broadcast::Sender<Event>,
}

impl EventBus {
    /// Create a new EventBus with a given channel capacity.
    /// If the channel is full, the oldest events are dropped
    /// (lagging subscribers miss them — appropriate for
    /// observational events, not transactional ones).
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    /// Publish an event to all subscribers.
    pub fn publish(&self, event: Event) -> Result<(), EventBusError> {
        self.tx
            .send(event)
            .map_err(|e| EventBusError::PublishFailed(e.to_string()))?;
        Ok(())
    }

    /// Subscribe to all events on the bus.
    /// Returns a receiver that can be awaited.
    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.tx.subscribe()
    }

    /// Returns the number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_and_receive() {
        let bus = EventBus::new(16);
        let mut rx = bus.subscribe();

        let event = Event::new("test.event", "test-source");
        bus.publish(event.clone()).unwrap();

        // Use try_recv since we're not in an async context
        let received = rx.try_recv().unwrap();
        assert_eq!(received.event_type, "test.event");
        assert_eq!(received.source, "test-source");
    }

    #[test]
    fn multiple_subscribers() {
        let bus = EventBus::new(16);
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();

        let event = Event::new("multi.test", "src");
        bus.publish(event).unwrap();

        let r1 = rx1.try_recv().unwrap();
        let r2 = rx2.try_recv().unwrap();
        assert_eq!(r1.event_id, r2.event_id);
    }

    #[test]
    fn event_with_payload() {
        let event = Event::new("test", "src").with_payload("{\"key\":\"value\"}");
        assert_eq!(event.payload, Some(String::from("{\"key\":\"value\"}")));
    }
}
