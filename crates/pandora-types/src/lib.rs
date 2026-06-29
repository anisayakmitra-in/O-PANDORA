//! # pandora-types
//!
//! Shared types for the Pandora OS.
//!
//! This crate hosts the constitutional metadata
//! foundation (see  module), the harness
//! spec, and the gene spec. Concrete runtime crates
//! depend on these types.

pub mod harness;
pub use harness::HarnessSpec;

pub mod harness_gene;
pub use harness_gene::HarnessGene;

pub mod capability_leasing;
pub mod constitutional;
pub mod engine_registry;
pub mod evolution_runtime;
pub mod execution;
pub mod execution_memory;
pub mod gene_context;
pub mod governance_runtime;
pub mod identity_runtime;
pub mod self_healing;
pub mod universal;
