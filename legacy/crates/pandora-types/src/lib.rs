#![allow(clippy::empty_line_after_doc_comments)]
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

pub mod artifacts;
pub mod connection_manager;
pub mod decision;
pub mod error;
pub mod evaluation_verdict;
pub mod event_store;
pub mod events;
pub mod execution_plan;
pub mod gene;
pub mod gene_package;
pub mod harness_gene;
pub mod lockfile;
pub mod package_format;
pub mod permissions;
pub mod profile;
pub mod provenance;
pub mod provider;
pub mod provider_db;
pub mod provider_health;
pub mod session;
pub mod signing;
pub mod trust;
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
pub mod ledger;
pub mod parliament;
pub mod policy_engine;
pub mod recorder;
pub mod runtime_context;
pub mod self_healing;
pub mod services;
pub mod telemetry_engine;
pub mod universal;
pub mod workflow_engine;
pub mod config;
pub mod resource;
pub mod provider_intel;
pub mod lock;
pub mod scheduler;

pub mod artifact_store;
pub mod checkpoint;
pub mod verifier;
pub mod lifecycle;
pub mod compatibility;
pub mod package_health;
pub mod quality;
pub mod model_registry;
pub mod permissions_manifest;
pub mod runtime_node;
pub mod event_bus;
pub mod risk_engine;
pub mod context_strategy;
pub mod auth_manager;
