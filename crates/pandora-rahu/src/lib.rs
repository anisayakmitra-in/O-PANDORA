//! # pandora-rahu
//!
//! RAHU: orchestration and source-harness resolution.
//!
//! RAHU sits between the loop registry and execution. It
//! receives a request from NARAD and produces an execution
//! route that the runtime can dispatch to a source harness.
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
//! ```text
//! User
//!   |
//!   v
//! NARAD
//!   |
//!   v
//! Loop Registry
//!   |
//!   v
//! RAHU          <-- this crate
//!   |
//!   v
//! ExecutionRoute
//!   |
//!   v
//! Capability Leasing
//!   |
//!   v
//! Source Harness
//! ```
//!
//! ## Constitutional lifecycle (KETU)
//!
//! Every Meta Harness that receives a RAHU plan must run
//! the full RAHU -> Core -> KETU lifecycle. KETU is the
//! validation phase.
//!
//! ## Design rules
//!
//! - RAHU is pure. It takes a request and returns an
//!   execution route. No side effects.
//! - All resolution is dynamic through registries.
//! - Concrete harnesses (Phoenix, ANUBIS, ...) are
//!   registered at runtime. RAHU only knows the
//!   `SourceHarnessKind` enum.
//!
//! ## Module organization
//!
//! - **Core pipeline**: `pipeline`, `constitutional_pipeline`,
//!   `kernel`, `runtime_integration`
//! - **Registries**: `registry`, `resolver`, `selection`
//! - **Harnesses**: `harness`, `capability`, `plan`, `lifecycle`
//! - **Source/Meta harnesses**: `phoenix`, `anubis`, `moira`,
//!   `hades`, `shani`, `panoptes`, `hephaestus`
//! - **Evolution**: `gepa`, `dsr`, `gepa_runtime`, `dsr_runtime`,
//!   `shani_runtime`
//! - **Cognition**: `autonomous_loop`, `cognition_scheduler`,
//!   `long_running`, `reflection_runtime`, `self_healing`
//! - **Governance**: `governance_gate`, `constitutional_safety`,
//!   `shadow_council`
//! - **Infrastructure**: `event_bus`, `execution_history`,
//!   `memory_consolidation`, `debug_pipeline`, `performance`,
//!   `adaptive_budget`
//! - **Compat**: `compat`, `runtime`

#![forbid(unsafe_code)]

pub mod adaptive_budget;
pub mod anubis;
pub mod autonomous_loop;
pub mod capability;
pub mod capability_manager;
pub mod cognition_scheduler;
pub mod compat;
pub mod constitution;
pub mod constitutional_pipeline;
pub mod constitutional_safety;
mod context;
pub mod debug_pipeline;
pub mod dsr;
pub mod dsr_runtime;
pub mod event_bus;
pub mod execution_history;
pub mod execution_manager;
pub mod gene_executor;
pub mod gepa;
pub mod gepa_runtime;
pub mod governance_gate;
pub mod hades;
pub mod harness;
pub mod hephaestus;
pub mod identity_tracker;
pub mod kernel;
pub mod lifecycle;
pub mod long_running;
pub mod memory_consolidation;
pub mod memory_store;
pub mod moira;
pub mod panoptes;
pub mod performance;
pub mod phoenix;
pub mod pipeline;
pub mod plan;
pub mod reflection_runtime;
pub mod registry;
pub mod resolver;
pub mod runtime;
pub mod sandbox_orchestrator;
pub mod selection;
pub mod shadow_council;
pub mod shani;
pub mod shani_runtime;
pub mod workflow_executor;

pub use capability::{CapabilityKind, CapabilityLeaseRequest, CapabilityRequest};
pub use context::RequestContext;
pub use harness::{Gene, GeneKind};
pub use harness::{MetaHarness, MetaHarnessKind, SourceHarness, SourceHarnessKind};
pub use lifecycle::{run_lifecycle, LifecycleOutcome};
pub use plan::{rahu_plan, ExecutionMode, ExecutionPlan, ExecutionRoute};
pub use registry::{GeneRegistry, MetaHarnessRegistry, RahuError, SourceHarnessRegistry};
pub use resolver::populated_registries;
pub use selection::{GeneSelection, MetaHarnessSelection, SourceHarnessSelection};
