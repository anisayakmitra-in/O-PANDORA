//! Pandora Harness Framework
//!
//! Defines the contracts for Constitutional and Extension Harnesses.
//! No runtime logic, provider logic, or execution logic belongs here.

pub mod contracts;
pub mod error;
pub mod manifest;
pub mod registry;
pub mod roles;
pub mod traits;

pub use contracts::error::HarnessError;
pub use contracts::error::Result;
pub use contracts::manifest::HarnessManifest;
pub use contracts::registry::Registry;
pub use contracts::roles::HarnessRole;
pub use contracts::traits::Harness;
