use serde::{Deserialize, Serialize};

use crate::category::EventCategory;
use crate::priority::EventPriority;

/// Metadata attached to every event envelope.
///
/// Carries everything the bus and subscribers need to route,
/// order, and trace an event without inspecting its payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// Globally-unique event id (typically a UUID or ULID).
    pub id: String,

    /// Stable, human-readable event name (e.g. `gene.loaded`).
    pub name: String,

    /// Event category.
    pub category: EventCategory,

    /// Event priority.
    pub priority: EventPriority,

    /// Originating source identifier (e.g. `pandora-runtime`,
    /// `anubis-memory`, `kuber-palace`).
    pub source: String,

    /// Time the event was emitted, in milliseconds since the UNIX
    /// epoch. Zero means "unknown".
    #[serde(default)]
    pub timestamp_ms: u64,

    /// Optional correlation id used to group related events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,

    /// Optional causation id identifying the event that caused this
    /// one (for event chains).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub causation_id: Option<String>,
}

impl EventMetadata {
    /// Create a new metadata record with a generated `id` and the
    /// current wall-clock timestamp in milliseconds.
    pub fn new(
        name: impl Into<String>,
        category: EventCategory,
        source: impl Into<String>,
    ) -> Self {
        Self {
            id: generate_event_id(),
            name: name.into(),
            category,
            priority: EventPriority::default(),
            source: source.into(),
            timestamp_ms: current_timestamp_ms(),
            correlation_id: None,
            causation_id: None,
        }
    }

    /// Override the priority.
    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Override the id.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Set the correlation id.
    pub fn with_correlation(mut self, correlation_id: impl Into<String>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }

    /// Set the causation id.
    pub fn with_causation(mut self, causation_id: impl Into<String>) -> Self {
        self.causation_id = Some(causation_id.into());
        self
    }

    /// Override the timestamp (milliseconds since UNIX epoch).
    pub fn with_timestamp_ms(mut self, ts: u64) -> Self {
        self.timestamp_ms = ts;
        self
    }
}

fn generate_event_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    // Combine nanos with a thread-local counter to make collisions
    // extremely unlikely within a single process without pulling
    // in a UUID dependency.
    thread_local! {
        static COUNTER: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    }
    let n = COUNTER.with(|c| {
        let v = c.get().wrapping_add(1);
        c.set(v);
        v
    });
    format!("evt_{:x}_{:x}", nanos, n)
}

fn current_timestamp_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
