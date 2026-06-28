//! Pandora Harness Framework
//!
//! Defines the contracts for Constitutional and Extension Harnesses.
//! No runtime logic, provider logic, or execution logic belongs here.

pub mod error;
pub mod manifest;
pub mod registry;
pub mod roles;
pub mod traits;

pub use error::HarnessError;
pub use error::Result;
pub use manifest::HarnessManifest;
pub use registry::Registry;
pub use roles::HarnessRole;
pub use traits::Harness;
