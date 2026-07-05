//! Contracts module - all trait definitions and type contracts for harnesses

pub mod error;
pub mod manifest;
pub mod registry;
pub mod roles;
pub mod traits;

pub use crate::error::HarnessError;
pub use crate::error::Result;
pub use crate::manifest::HarnessManifest;
pub use crate::registry::Registry;
pub use crate::roles::HarnessRole;
pub use crate::traits::Harness;
