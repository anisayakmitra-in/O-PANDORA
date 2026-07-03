//! # pandora-types
//!
//! Canonical shared types for the Pandora OS.
//!
//! This crate is the canonical owner of:
//! - constitutional contracts
//! - universal contracts
//! - identity contracts
//! - execution contracts
//! - capability contracts
//! - manifest contracts
//! - lifecycle
//! - telemetry
//! - health
//! - trust
//! - versioning
//!
//! Every constitutional object composes a
//! ConstitutionalManifest (defined in constitutional.rs).
//! Its identity field is the canonical identity.
//!
//! The pandora-identity crate provides concrete identity
//! registries that compose these canonical types.

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
pub mod runtime_context;
pub mod workflow_engine;
pub mod provider_learning;
pub mod recorder;
pub mod telemetry_engine;
pub mod failure_intelligence;
pub mod knowledge_distillation;
pub mod experiment;
pub mod self_healing;
pub mod services;
pub mod universal;
