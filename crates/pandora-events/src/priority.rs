use serde::{Deserialize, Serialize};

/// Priority of an event.
///
/// Used by the bus to order delivery and by subscribers to filter
/// out noise. Higher priority events are delivered first within
/// the same `EventCategory`.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum EventPriority {
    /// Background / telemetry / best-effort events.
    Low = 0,
    /// Default priority for ordinary events.
    #[default]
    Normal = 1,
    /// Above-normal events (e.g. lifecycle transitions).
    High = 2,
    /// Reserved for emergency or stop-the-world events.
    Critical = 3,
}

impl EventPriority {
    /// Numeric value for ordering. Higher number = higher priority.
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// Build from a numeric value; unknown numbers clamp to `Normal`.
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => EventPriority::Low,
            1 => EventPriority::Normal,
            2 => EventPriority::High,
            3.. => EventPriority::Critical,
        }
    }
}
