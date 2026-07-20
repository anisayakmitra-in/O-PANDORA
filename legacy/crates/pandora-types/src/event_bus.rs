//! Event Bus — pub/sub for real-time runtime events.
//!
//! Everything subscribes — TUI, API, Fleet, telemetry, plugins.
//! Nothing polls. The runtime publishes events; subscribers react.
//!
//! This complements EventStore (which persists events). The bus
//! is for live in-process notification.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

/// A runtime event — what happened in the pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusEvent {
    pub kind: BusEventKind,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub source: String,
}

/// Event types — extensible via Custom variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusEventKind {
    // Execution lifecycle
    ExecutionStarted,
    ExecutionCompleted,
    ExecutionFailed,
    ExecutionCancelled,
    StageCompleted,
    StageFailed,
    // Provider events
    ProviderSelected,
    ProviderFailed,
    ProviderRecovered,
    // Governance
    PolicyEvaluated,
    PolicyBlocked,
    // Council
    HarnessDispatched,
    GeneExecuted,
    // Session
    SessionCreated,
    SessionClosed,
    // Fleet
    NodeJoined,
    NodeLeft,
    NodeStale,
    // Packages
    PackageInstalled,
    PackageUninstalled,
    PackagePublished,
    // Artifacts
    ArtifactCreated,
    ArtifactVerified,
    // Custom — extensible without modifying this enum
    Custom(String),
}

impl BusEventKind {
    pub fn label(&self) -> &str {
        match self {
            Self::ExecutionStarted => "execution.started",
            Self::ExecutionCompleted => "execution.completed",
            Self::ExecutionFailed => "execution.failed",
            Self::ExecutionCancelled => "execution.cancelled",
            Self::StageCompleted => "stage.completed",
            Self::StageFailed => "stage.failed",
            Self::ProviderSelected => "provider.selected",
            Self::ProviderFailed => "provider.failed",
            Self::ProviderRecovered => "provider.recovered",
            Self::PolicyEvaluated => "policy.evaluated",
            Self::PolicyBlocked => "policy.blocked",
            Self::HarnessDispatched => "council.harness_dispatched",
            Self::GeneExecuted => "council.gene_executed",
            Self::SessionCreated => "session.created",
            Self::SessionClosed => "session.closed",
            Self::NodeJoined => "fleet.node_joined",
            Self::NodeLeft => "fleet.node_left",
            Self::NodeStale => "fleet.node_stale",
            Self::PackageInstalled => "package.installed",
            Self::PackageUninstalled => "package.uninstalled",
            Self::PackagePublished => "package.published",
            Self::ArtifactCreated => "artifact.created",
            Self::ArtifactVerified => "artifact.verified",
            Self::Custom(name) => name,
        }
    }
}

/// The event bus — tokio broadcast channel under the hood.
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<BusEvent>,
}

impl EventBus {
    /// Create a new bus with a configurable buffer size.
    pub fn new(buffer_size: usize) -> Self {
        let (sender, _) = broadcast::channel(buffer_size);
        Self { sender }
    }

    /// Default buffer size (256 events).
    pub fn default_capacity() -> Self {
        Self::new(256)
    }

    /// Publish an event. Subscribers receive it asynchronously.
    pub fn publish(&self, kind: BusEventKind, payload: serde_json::Value, source: &str) {
        let event = BusEvent {
            kind,
            payload,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            source: source.into(),
        };
        // send errors when there are no subscribers — that's fine
        let _ = self.sender.send(event);
    }

    /// Subscribe to events. Returns a receiver.
    pub fn subscribe(&self) -> broadcast::Receiver<BusEvent> {
        self.sender.subscribe()
    }

    /// Get a clone of the sender (for passing to async tasks).
    pub fn sender(&self) -> broadcast::Sender<BusEvent> {
        self.sender.clone()
    }
}

/// A shared event bus — Arc-wrapped for sharing across tasks.
pub type SharedEventBus = Arc<EventBus>;

/// Create a shared event bus.
pub fn shared_bus() -> SharedEventBus {
    Arc::new(EventBus::default_capacity())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_and_subscribe() {
        let bus = EventBus::default_capacity();
        let mut rx = bus.subscribe();
        bus.publish(
            BusEventKind::ExecutionStarted,
            serde_json::json!({"task": "build API"}),
            "test",
        );
        // In sync context, use try_recv
        std::thread::sleep(std::time::Duration::from_millis(10));
        let event = rx.try_recv().expect("should receive event");
        assert_eq!(event.kind.label(), "execution.started");
        assert_eq!(event.source, "test");
    }

    #[test]
    fn custom_event_kind() {
        let kind = BusEventKind::Custom("eda.simulation.complete".into());
        assert_eq!(kind.label(), "eda.simulation.complete");
    }

    #[test]
    fn multiple_subscribers() {
        let bus = EventBus::default_capacity();
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();
        bus.publish(
            BusEventKind::StageCompleted,
            serde_json::json!({"stage": "plan"}),
            "test",
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(rx1.try_recv().is_ok());
        assert!(rx2.try_recv().is_ok());
    }
}
