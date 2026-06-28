//! Compatibility shim for the pre-refactor public API.
//!
//! ⚠️ TEMPORARY: This module preserves the old synchronous
//! `RuntimeEvent` enum and `EventBus` struct so existing callers
//! (if any) can be migrated to the new async contract without a
//! breaking change. New code should NOT use this module — it will
//! be removed once all callers have been migrated.
//!
//! Prefer the new abstractions:
//! - [`crate::Event`] for event types
//! - [`crate::EventBus`] for async, multi-subscriber fan-out
//! - [`crate::traits::Event`] for the trait
//! - [`crate::category::EventCategory`] for categorization
//! - [`crate::priority::EventPriority`] for prioritization

use std::any::Any;
use std::sync::mpsc::{channel, Receiver, Sender};

use serde::{Deserialize, Serialize};

use crate::category::EventCategory;
use crate::traits::Event;

/// Pre-refactor runtime event enum.
///
/// Each variant is a `String` payload for backward compatibility.
/// New code should define a dedicated `Event` impl per logical
/// event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeEvent {
    Boot(String),
    GeneLoaded(String),
    MemoryStored(String),
    MemoryRetrieved(usize),
    Telemetry(String),
    Harness(String),
    Mutation(String),
    Runtime(String),
}

impl Event for RuntimeEvent {
    fn name(&self) -> &str {
        match self {
            RuntimeEvent::Boot(_) => "runtime.boot",
            RuntimeEvent::GeneLoaded(_) => "gene.loaded",
            RuntimeEvent::MemoryStored(_) => "memory.stored",
            RuntimeEvent::MemoryRetrieved(_) => "memory.retrieved",
            RuntimeEvent::Telemetry(_) => "telemetry",
            RuntimeEvent::Harness(_) => "harness",
            RuntimeEvent::Mutation(_) => "gene.mutation",
            RuntimeEvent::Runtime(_) => "runtime",
        }
    }

    fn category(&self) -> EventCategory {
        match self {
            RuntimeEvent::Boot(_) | RuntimeEvent::Runtime(_) => EventCategory::Runtime,
            RuntimeEvent::GeneLoaded(_) | RuntimeEvent::Mutation(_) => EventCategory::Gene,
            RuntimeEvent::MemoryStored(_) | RuntimeEvent::MemoryRetrieved(_) => {
                EventCategory::Memory
            }
            RuntimeEvent::Telemetry(_) => EventCategory::Telemetry,
            RuntimeEvent::Harness(_) => EventCategory::Harness,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Pre-refactor synchronous single-producer/single-consumer
/// event bus.
///
/// ⚠️ This is a thin wrapper over `std::sync::mpsc` and is kept
/// only for backward compatibility. New code should use the async
/// [`crate::EventBus`] instead.
pub struct EventBus {
    pub sender: Sender<RuntimeEvent>,
    pub receiver: Receiver<RuntimeEvent>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    /// Create a new synchronous single-producer/single-consumer
    /// bus.
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self { sender, receiver }
    }
}
