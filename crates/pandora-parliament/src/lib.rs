//! Pandora Parliament — Constitutional Cognition Kernel.
//!
//! Parliament owns constitutional cognition and orchestration.
//! It does not execute. Execution is delegated through the
//! Capability Resolution Engine to Source Harnesses.
//!
//! ## Owned Subsystems
//!
//! - Service Registry: resolve services by contract, not by name
//! - Constitutional Event Bus: all inter-harness communication
//! - Lease Manager: temporary, revocable resource ownership
//! - Constitution Engine: policy evaluation and constitutional state
//! - Runtime Registry: track all registered runtimes and their capabilities
//! - Dependency Graph: track dependencies between constitutional objects
//! - Loop Engine: closed and open loop orchestration (future)
//! - Capability Resolution Engine: data-driven provider selection (future)

pub mod constitution_engine;
pub mod event_bus;
pub mod lease_manager;
pub mod service_registry;

pub use constitution_engine::{ConstitutionEngine, ConstitutionalState, Policy, PolicyEvaluation};
pub use event_bus::{Event, EventBus, EventBusError};
pub use lease_manager::{Lease, LeaseId, LeaseManager, LeaseManagerError, LeaseState};
pub use service_registry::{ServiceRegistry, ServiceRegistryError};
