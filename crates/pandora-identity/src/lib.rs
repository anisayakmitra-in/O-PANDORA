//! # pandora-identity
//!
//! Pandora's Constitutional Identity Framework.
//!
//! Every manifest-driven object in Pandora possesses a
//! stable identity. This crate defines the **contract**
//! that every such object implements. It is metadata
//! only: no I/O, no runtime state, no business logic.
//!
//! ## What has an Identity
//!
//! - Source Harness
//! - Meta Harness
//! - Gene
//! - Loop
//! - Provider
//! - Tool
//! - Capability
//! - Sandbox Backend
//! - Memory Backend
//! - Execution Session
//! - Engineering Session
//! - Workflow
//! - Agent
//! - Plugin
//! - MCP
//! - Package
//! - Marketplace Asset
//!
//! ## Architecture position
//!
//! Identity is the foundation beneath manifests,
//! registries, and governance. Every constitutional
//! object in Pandora declares its identity; the runtime
//! uses identity to resolve, version, sign, and govern
//! the object.
//!
//! ## Design rules
//!
//! - Identity is **data**, not behavior.
//! - All identity types are  so
//!   they can flow through the runtime freely.
//! - Identities are  so they
//!   can be persisted, transmitted, and stored in
//!   ANUBIS.
//! - Identities are **stable**: the same
//!   always refers to the same logical object across
//!   runs.

#![forbid(unsafe_code)]

mod kind;
mod manifest;
mod registry;
mod relationships;
mod version;

pub use kind::IdentityKind;
pub use manifest::{
    Identity, IdentityCapabilities, IdentityDependencies, IdentityHealth, IdentityLifecycleStage,
    IdentityLineage, IdentityManifest, IdentityMetadata, IdentityProvenance, IdentitySignature,
    IdentityStatus, IdentityTelemetry, IdentityTrust,
};
pub use registry::{IdentityEntry, IdentityError, IdentityRegistry};
pub use relationships::{Relationship, RelationshipKind};
pub use version::IdentityVersion;
