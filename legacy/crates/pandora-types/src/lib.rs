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

pub mod approval_store;
pub mod harness;
pub use approval_store::{ApprovalStatus, ApprovalStore, PendingApproval};
pub use decision::{Decision, DecisionLog, Outcome, RejectedOption};
pub use error::PandoraError;
pub use harness::{
    generate_harness_toml, Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder,
    HarnessPackage, HarnessSpec, SlashCommand,
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
pub mod sqlite_session;
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

pub mod prelude;

pub mod capability_leasing;
pub mod capability_resolution;
pub mod config;
pub mod constitutional;
pub mod execution;
pub mod execution_memory;
pub mod failure_intelligence;
pub mod gene_context;
pub mod governance_runtime;
pub mod identity_runtime;
pub mod knowledge_distillation;
pub mod ledger;
pub mod lock;
pub mod parliament;
pub mod policy_engine;
pub mod provider_intel;
pub mod recorder;
pub mod resource;
pub mod runtime_context;
pub mod scheduler;
pub mod self_healing;
pub mod services;
pub mod telemetry_engine;
pub mod universal;
pub mod workflow_engine;

pub mod artifact_store;
pub mod auth_manager;
pub mod capability_registry;
pub mod checkpoint;
pub mod compatibility;
pub mod connection_lifecycle;
pub mod context_strategy;
pub mod event_bus;
pub mod hierarchical_memory;
pub mod intent_router;
pub mod lifecycle;
pub mod lifecycle_hooks;
pub mod model_registry;
pub mod package_health;
pub mod permissions_manifest;
pub mod plugin_manifest;
pub mod quality;
pub mod risk_engine;
pub mod runtime_node;
pub mod universal_registry;
pub mod verifier;
pub mod workflow_lifecycle;

#[cfg(test)]
pub(crate) static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
pub(crate) struct EnvVarGuard {
    name: &'static str,
    previous: Option<std::ffi::OsString>,
}

#[cfg(test)]
impl EnvVarGuard {
    pub(crate) fn set(name: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let previous = std::env::var_os(name);
        std::env::set_var(name, value);
        Self { name, previous }
    }
}

#[cfg(test)]
impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(value) = self.previous.take() {
            std::env::set_var(self.name, value);
        } else {
            std::env::remove_var(self.name);
        }
    }
}
