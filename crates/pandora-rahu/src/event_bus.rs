//! Universal Runtime Event Bus.
//!
//! Everything communicates through events.
//! No direct coupling.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// Event categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventCategory {
    Execution,
    Workflow,
    Capability,
    Gene,
    Memory,
    Governance,
    Evolution,
    Telemetry,
    Identity,
    Sandbox,
}

/// A runtime event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeEvent {
    pub event_id: String,
    pub category: EventCategory,
    pub payload: String,
    pub timestamp_ms: u64,
    pub metadata: BTreeMap<String, String>,
}

/// Callback type for event subscribers.
type EventCallback = Box<dyn Fn(&RuntimeEvent) + Send + Sync>;

/// Universal event bus.
pub struct EventBus {
    subscribers: Arc<Mutex<Vec<EventCallback>>>,
    history: Arc<Mutex<Vec<RuntimeEvent>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            subscribers: Arc::new(Mutex::new(Vec::new())),
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn publish(&self, event: RuntimeEvent) {
        let subs = self.subscribers.lock().unwrap();
        for cb in subs.iter() {
            cb(&event);
        }
        self.history.lock().unwrap().push(event);
    }

    pub fn subscribe<F: Fn(&RuntimeEvent) + Send + Sync + 'static>(&self, cb: F) {
        self.subscribers.lock().unwrap().push(Box::new(cb));
    }

    pub fn history(&self) -> Vec<RuntimeEvent> {
        self.history.lock().unwrap().clone()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_publish_and_history() {
        let bus = EventBus::new();
        bus.publish(RuntimeEvent {
            event_id: "e1".to_string(),
            category: EventCategory::Execution,
            payload: "test".to_string(),
            timestamp_ms: 0,
            metadata: BTreeMap::new(),
        });
        assert_eq!(bus.history().len(), 1);
    }
}
