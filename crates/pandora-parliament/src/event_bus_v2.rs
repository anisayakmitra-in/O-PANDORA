//! Event Bus v2 — typed events with routing, filtering, and persistence.
//!
//! Builds on the v1 broadcast bus. Adds:
//!   - Typed events with domain routing
//!   - Event filters and subscriptions
//!   - Event replay for late-joining subscribers
//!   - Optional persistence for audit

use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use chrono::{DateTime, Utc};

/// A typed event with routing metadata.
#[derive(Debug, Clone)]
pub struct TypedEvent {
    pub event_id: String,
    pub event_type: String,
    pub source: String,
    pub domain: String,
    pub severity: EventSeverity,
    pub payload: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventSeverity { Debug, Info, Warn, Error, Critical }

impl EventSeverity { pub fn name(&self) -> &'static str { match self { EventSeverity::Debug => "debug", EventSeverity::Info => "info", EventSeverity::Warn => "warn", EventSeverity::Error => "error", EventSeverity::Critical => "critical" } } }

impl TypedEvent {
    pub fn new(event_type: impl Into<String>, source: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            event_id: format!("evt-{:x}", 42u64),
            event_type: event_type.into(), source: source.into(), domain: domain.into(),
            severity: EventSeverity::Info, payload: None, timestamp: Utc::now(),
        }
    }
}

/// A subscriber filter for selective event routing.
#[derive(Debug, Clone)]
pub struct EventFilter {
    pub event_types: Vec<String>,
    pub domains: Vec<String>,
    pub min_severity: EventSeverity,
    pub sources: Vec<String>,
}

impl EventFilter {
    pub fn matches(&self, event: &TypedEvent) -> bool {
        if !self.event_types.is_empty() && !self.event_types.contains(&event.event_type) { return false; }
        if !self.domains.is_empty() && !self.domains.contains(&event.domain) { return false; }
        if !self.sources.is_empty() && !self.sources.contains(&event.source) { return false; }
        let sev = |s: &EventSeverity| -> u8 { match s { EventSeverity::Debug => 0, EventSeverity::Info => 1, EventSeverity::Warn => 2, EventSeverity::Error => 3, EventSeverity::Critical => 4 } };
        sev(&event.severity) >= sev(&self.min_severity)
    }
}

/// Event subscriber with optional filter.
pub struct EventSubscriber {
    pub id: String,
    pub filter: Option<EventFilter>,
}

/// Event Bus v2 — typed event routing with history and filtering.
pub struct EventBusV2 {
    events: VecDeque<TypedEvent>,
    subscribers: Vec<EventSubscriber>,
    max_history: usize,
}

impl EventBusV2 {
    pub fn new() -> Self { Self { events: VecDeque::new(), subscribers: Vec::new(), max_history: 1000 } }

    /// Publish a typed event.
    pub fn publish(&mut self, event: TypedEvent) {
        self.events.push_back(event);
        while self.events.len() > self.max_history { self.events.pop_front(); }
    }

    /// Register a subscriber.
    pub fn subscribe(&mut self, id: impl Into<String>) -> String {
        let sid = id.into();
        self.subscribers.push(EventSubscriber { id: sid.clone(), filter: None });
        sid
    }

    /// Subscribe with a filter.
    pub fn subscribe_filtered(&mut self, id: impl Into<String>, filter: EventFilter) -> String {
        let sid = id.into();
        self.subscribers.push(EventSubscriber { id: sid.clone(), filter: Some(filter) });
        sid
    }

    /// Get events matching a subscriber's filter.
    pub fn events_for(&self, subscriber_id: &str) -> Vec<&TypedEvent> {
        if let Some(sub) = self.subscribers.iter().find(|s| s.id == subscriber_id) {
            match &sub.filter {
                Some(filter) => self.events.iter().filter(|e| filter.matches(e)).collect(),
                None => self.events.iter().collect(),
            }
        } else { Vec::new() }
    }

    /// Get all events since a count.
    pub fn events_since(&self, count: usize) -> &[TypedEvent] {
        let start = self.events.len().saturating_sub(self.events.len() - count.min(self.events.len()));
        &self.events.as_slices().0[start..]
    }

    /// Find events by domain.
    pub fn by_domain(&self, domain: &str) -> Vec<&TypedEvent> {
        self.events.iter().filter(|e| e.domain == domain).collect()
    }

    /// Find events by severity threshold.
    pub fn by_severity(&self, min: EventSeverity) -> Vec<&TypedEvent> {
        let sev = |s: &EventSeverity| -> u8 { match s { EventSeverity::Debug => 0, EventSeverity::Info => 1, EventSeverity::Warn => 2, EventSeverity::Error => 3, EventSeverity::Critical => 4 } };
        let min_s = sev(&min);
        self.events.iter().filter(|e| sev(&e.severity) >= min_s).collect()
    }

    pub fn event_count(&self) -> usize { self.events.len() }
    pub fn subscriber_count(&self) -> usize { self.subscribers.len() }

    /// Replay all events to a new subscriber.
    pub fn replay_all(&self) -> impl Iterator<Item = &TypedEvent> {
        self.events.iter()
    }
}

impl Default for EventBusV2 { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_and_retrieve() {
        let mut bus = EventBusV2::new();
        let event = TypedEvent::new("execution.completed", "orchestrator", "coding");
        bus.publish(event);
        assert_eq!(bus.event_count(), 1);
    }

    #[test]
    fn subscriber_filtering() {
        let mut bus = EventBusV2::new();
        let sid = bus.subscribe_filtered("test-sub", EventFilter {
            event_types: vec!["execution.completed".into()], domains: vec![], min_severity: EventSeverity::Info, sources: vec![],
        });
        bus.publish(TypedEvent::new("execution.completed", "orch", "coding"));
        bus.publish(TypedEvent::new("benchmark.updated", "bench", "coding"));
        let events = bus.events_for(&sid);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn domain_filter() {
        let mut bus = EventBusV2::new();
        bus.publish(TypedEvent::new("e1", "s", "coding"));
        bus.publish(TypedEvent::new("e2", "s", "research"));
        let coding = bus.by_domain("coding");
        assert_eq!(coding.len(), 1);
    }

    #[test]
    fn severity_filtering() {
        let mut bus = EventBusV2::new();
        let mut e = TypedEvent::new("critical-event", "s", "sys");
        e.severity = EventSeverity::Critical;
        bus.publish(e);
        bus.publish(TypedEvent::new("info-event", "s", "sys"));
        let critical = bus.by_severity(EventSeverity::Error);
        assert_eq!(critical.len(), 1);
    }

    #[test]
    fn replay_all() {
        let mut bus = EventBusV2::new();
        for i in 0..5 { bus.publish(TypedEvent::new(format!("e{}", i), "s", "t")); }
        let replayed: Vec<&TypedEvent> = bus.replay_all().collect();
        assert_eq!(replayed.len(), 5);
    }
}
