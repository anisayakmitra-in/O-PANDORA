//! # pandora-types
#![allow(clippy::unnecessary_sort_by)]
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
pub use decision::{Decision, DecisionLog, RejectedOption};
pub use error::PandoraError;
pub use harness::{
    Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder, HarnessSpec, SlashCommand,
};
pub use session::{Session, SessionStatus, SessionStore};

pub mod decision;
pub mod evaluation_verdict;
pub mod error;
pub mod profile;
pub mod artifacts;
pub mod provider_db;
pub mod package_format;
pub mod signing;
pub mod permissions;
pub mod lockfile;
pub mod trust;
pub mod event_store;
pub mod provider;
pub mod connection_manager;
pub mod provenance;
pub mod events;
pub mod provider_health;
pub mod execution_plan;
pub mod gene;
pub mod gene_package;
pub mod harness_gene;
pub mod session;
pub use gene::{
    Gene, GeneKind, GeneLineage, GeneLineageEntry, GeneManifest, GeneManifestBuilder,
    SlashCommandOwner,
};

/// Architecture invariant: every executable behavior originates from
/// either a Constitutional Service or a Gene. Nothing else may execute.
/// This prevents future abstraction creep beyond the frozen 5-layer model.
pub const ARCHITECTURE_INVARIANT: &str = "execute-from-service-or-gene-only";

pub use gene_package::{discover_gene_packages, GenePackage, GenePackageManifest, SlashCommandDef};
pub use harness_gene::HarnessGene;

pub mod capability_leasing;
pub mod capability_resolution;
pub mod constitutional;
pub mod execution;
pub mod execution_memory;
pub mod failure_intelligence;
pub mod gene_context;
pub mod governance_runtime;
pub mod identity_runtime;
pub mod knowledge_distillation;
pub mod policy_engine;
pub mod recorder;
pub mod runtime_context;
pub mod self_healing;
pub mod services;
pub mod telemetry_engine;
pub mod universal;
pub mod workflow_engine;
pub mod ledger;
pub mod parliament;
