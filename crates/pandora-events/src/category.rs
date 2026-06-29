use serde::{Deserialize, Serialize};

/// Coarse-grained category for an event.
///
/// New variants can be added to support future systems (MOIRA state,
/// NARAD planning, KUBER installation, capability leasing, etc.)
/// without breaking the contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum EventCategory {
    /// Runtime lifecycle events (boot, shutdown, restart).
    Runtime,

    /// ANUBIS memory events (store, retrieve, consolidate, forget).
    Memory,

    /// Cognition events (signatures, modules, optimizers, evaluation).
    Cognition,

    /// Provider lifecycle events (register, unregister, request, response).
    Provider,

    /// Model events (load, unload, capability change).
    Model,

    /// Harness lifecycle events (load, unload, execute).
    Harness,

    /// Governance decision events (approve, reject, quarantine).
    Governance,

    /// Scheduling events (queue, dispatch, complete).
    Scheduling,

    /// Telemetry events (metric, trace, log).
    Telemetry,

    /// Capability leasing events (acquire, renew, release).
    Capability,

    /// Installation events (KUBER palace install, uninstall, upgrade).
    Installation,

    /// Gene evolution events (mutation, selection, lineage).
    Gene,

    /// Catch-all for events that do not fit any other category.
    #[default]
    Other,
}
