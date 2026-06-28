//! # pandora-loops
//!
//! Pandora cognition loop registry.
//!
//! A Loop is a named unit of cognition that takes a
//!  and produces a . Loops
//! are the runtime's mechanism for executing iterative
//! cognition: planning, reflection, repair, evolution,
//! benchmark, constitutional checks, swarm coordination,
//! memory consolidation, user-defined loops.
//!
//! Loops are registered in a  and discovered
//! at runtime. The registry is keyed by loop name. The
//! runtime resolves the loop for a given
//! by inspecting the  and asking the registry.
//!
//! ## Architecture position
//!
//! User input
//!     |
//!     v
//! NARAD -> PlanningContext
//!     |
//!     v
//! LoopRegistry::resolve
//!     |
//!     v
//! Selected Loop::run(context)
//!     |
//!     v
//! LoopOutcome
//!     |
//!     v
//! MOIRA / RAHU / Source Harnesses
//!
//! ## Design rules
//!
//! - Loops are -compatible. The registry holds
//!   .
//! - Loops are pure: they take a ,
//!   return a , and have no side effects
//!   beyond what they explicitly call. The runtime
//!   owns all I/O.
//! - Loops are  so the runtime can run
//!   them on any executor.
//! - Loops report a  so the runtime can
//!   decide whether to continue, retry, or escalate.

#![forbid(unsafe_code)]

mod error;
pub mod loops;
mod outcome;
mod registry;

pub use error::LoopError;
pub use outcome::{LoopOutcome, LoopStatus, OutcomeArtifact};
pub use registry::{Loop, LoopKind, LoopRegistry, RegistryError};

use pandora_narad::PlanningContext;

/// Top-level entry point: resolve a loop for the given
/// context and run it. Returns the loop's outcome.
///
/// This is the function the runtime calls after NARAD
/// has produced a  and before the
/// downstream stages (MOIRA, RAHU, source harnesses)
/// consume the loop's result.
pub async fn run(
    registry: &LoopRegistry,
    context: &PlanningContext,
) -> Result<LoopOutcome, LoopError> {
    let loop_kind = registry.resolve(&context.intent.kind)?;
    let loop_impl = registry
        .resolve_kind(loop_kind)
        .ok_or(LoopError::NotFound(loop_kind))?;
    Ok(loop_impl.run(context).await)
}
