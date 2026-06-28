//! # pandora-events
//!
//! Canonical owner of Pandora's event infrastructure.
//!
//! This crate defines the **contracts** that any event in the
//! system must satisfy. It does not implement runtime, memory,
//! cognition, governance, provider, harness, model, scheduling,
//! or telemetry logic — those are higher layers that emit events
//! through this crate.
//!
//! ## Crate layout
//!
//! | Module        | Responsibility                                |
//! |---------------|-----------------------------------------------|
//! | [`traits`]    | The single object-safe `Event` trait          |
//! | [`types`]     | `EventEnvelope`, `SerializableEvent`, helpers |
//! | [`metadata`]  | `EventMetadata` (id, timestamp, source, …)    |
//! | [`priority`]  | `EventPriority` (Low / Normal / High / Crit.) |
//! | [`category`]  | `EventCategory` (Runtime, Memory, …)          |
//! | [`bus`]       | `EventBus` — async, multi-subscriber fan-out  |
//! | [`publisher`] | `Publisher` trait + `FnPublisher` adapter     |
//! | [`subscriber`]| `Subscriber` trait + `Subscription` handle    |
//! | [`filter`]    | `EventFilter` trait + composable filters      |
//! | [`registry`]  | `EventRegistry` for cross-crate introspection|
//! | [`error`]     | `EventError` + `Result` alias                 |
//! | [`compat`]    | ⚠️ TEMPORARY backward-compat shim             |
//!
//! ## Future compatibility
//!
//! The contracts are designed so the following systems can emit
//! events **without changes to this crate**:
//!
//! - Runtime events
//! - ANUBIS memory events
//! - MOIRA state events
//! - NARAD planning events
//! - Capability Leasing events
//! - KUBER Palace installation events
//! - Provider lifecycle events
//! - Gene evolution events
//! - Harness lifecycle events
//! - Governance decision events
//!
//! No plugin loader, capability-leasing engine, or system
//! scheduler lives here.
//!
//! ## Serialization
//!
//! Every event is **serializable** in practice: each concrete
//! event type should derive or implement `Serialize` and
//! `Deserialize`, then be wrapped in
//! [`types::SerializableEvent`] for boundary-crossing
//! (process, network, persistence). The `Event` trait itself
//! stays object-safe by not requiring `Serialize`/`Deserialize`
//! as super-traits.

pub mod bus;
pub mod category;
pub mod compat;
pub mod error;
pub mod filter;
pub mod metadata;
pub mod priority;
pub mod publisher;
pub mod registry;
pub mod subscriber;
pub mod traits;
pub mod types;

// Re-export the core abstractions for ergonomic use at call sites.
pub use bus::{EventBus, DEFAULT_SUBSCRIBER_BUFFER};
pub use category::EventCategory;
pub use error::{EventError, Result};
pub use filter::{
    AcceptAll, AllOf, AnyOf, CategoryFilter, EventFilter, NamePrefixFilter, PriorityFilter,
};
pub use metadata::EventMetadata;
pub use priority::EventPriority;
pub use publisher::{FnPublisher, Publisher};
pub use registry::{EventFactory, EventRegistration, EventRegistry};
pub use subscriber::{Subscriber, Subscription};
pub use traits::Event;
pub use types::{
    deserialize_event, dyn_event, serialize_event, DynEvent, EventEnvelope, SerializableEvent,
    SubscriberId, SubscriptionId,
};
