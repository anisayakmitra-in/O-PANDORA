//! # pandora-rahu
//!
//! RAHU: orchestration / source-harness resolution.
//!
//! RAHU sits between the loop registry and execution. It
//! receives a  (from NARAD) and produces
//! an  that the runtime can route to a
//! source harness.
//!
//! RAHU never executes work itself. It resolves:
//!
//! - which **Source Harness** should handle the request
//!   (Phoenix, ANUBIS, MOIRA, HADES, SHANI, Provider)
//! - which **Meta Harness** within that source
//!   should be invoked
//! - which **Gene** (the smallest unit of evolution) the
//!   meta harness should run
//! - which **Capabilities** the runtime must lease
//!
//! All resolution is through registries. RAHU does not
//! know concrete implementations.
//!
//! ## Architecture position
//!
//! User
//!     |
//!     v
//! NARAD
//!     |
//!     v
//! Loop Registry
//!     |
//!     v
//! RAHU          <-- this crate
//!     |
//!     v
//! ExecutionRoute
//!     |
//!     v
//! Capability Leasing
//!     |
//!     v
//! Source Harness
//!
//! ## Constitutional lifecycle (KETU)
//!
//! Every Meta Harness that receives a RAHU plan must run
//! the full RAHU -> Core -> KETU lifecycle. KETU is the
//! validation phase. See .
//!
//! ## Design rules
//!
//! - RAHU is pure. It takes a  and
//!   returns an . No side effects.
//! - All resolution is dynamic through registries.
//! - Concrete harnesses (Phoenix, ANUBIS, ...) are
//!   registered at runtime. RAHU only knows the
//!    enum.

#![forbid(unsafe_code)]

mod capability;
pub mod constitution;
mod context;
mod harness;
pub mod lifecycle;
mod plan;
mod registry;
mod resolver;
pub mod runtime;
mod selection;

pub use capability::{CapabilityKind, CapabilityLeaseRequest, CapabilityRequest};
pub use context::RequestContext;
pub use harness::{
    Gene, GeneKind, GeneManifest, MetaHarness, MetaHarnessKind, MetaHarnessManifest, SourceHarness,
    SourceHarnessKind, SourceHarnessManifest,
};
pub use lifecycle::{
    run_lifecycle, Confidence, CorePhase, CorePhaseSnapshot, KetuPhase, KetuStatus, KetuValidation,
    LifecycleOutcome, MetaHarnessLifecycle, RahuPhase,
};

pub use constitution::{
    run as run_constitutional, ConstitutionalHarness, CoreContext, FindingSeverity, KetuContext,
    LifecycleResult, RahuContext, ValidationFinding, ValidationReport,
};
pub use plan::{ExecutionMode, ExecutionPlan, ExecutionRoute};
pub use registry::{GeneRegistry, MetaHarnessRegistry, RahuError, SourceHarnessRegistry};
pub use resolver::{populated_registries, resolve};
pub use selection::{GeneSelection, MetaHarnessSelection, SourceHarnessSelection};

use pandora_loops::LoopOutcome;
use pandora_narad::PlanningContext;

/// Top-level entry: resolve a planning context into an
/// execution plan. The plan tells the runtime which
/// source harness, meta harness, gene, and capabilities
/// are needed to satisfy the request.
pub fn plan(
    source_registry: &SourceHarnessRegistry,
    meta_registry: &MetaHarnessRegistry,
    gene_registry: &GeneRegistry,
    context: &PlanningContext,
) -> Result<ExecutionPlan, RahuError> {
    resolve(source_registry, meta_registry, gene_registry, context)
}

/// The full RAHU output: the execution plan plus the
/// loop outcome that produced it. The runtime can use
///  to drive capability leasing and
/// dispatch;  reports what the loop did.
#[derive(Debug, Clone)]
pub struct RahuOutput {
    pub plan: ExecutionPlan,
    pub loop_outcome: LoopOutcome,
}

impl RahuOutput {
    pub fn new(plan: ExecutionPlan, loop_outcome: LoopOutcome) -> Self {
        RahuOutput { plan, loop_outcome }
    }
}
