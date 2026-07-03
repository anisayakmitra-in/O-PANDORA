//! Dependency — consolidated into pandora-discovery.
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyRequirement {
    pub package: String,

    pub minimum_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionResult {
    pub resolved: bool,

    pub missing: Vec<String>,

    pub resolved_packages: Vec<String>,
}

use crate::loader::RuntimePackageLoader;

pub struct DependencyResolver;

impl DependencyResolver {
    pub fn resolve(
        loader: &RuntimePackageLoader,

        requirements: &[DependencyRequirement],
    ) -> ResolutionResult {
        let mut missing = Vec::new();

        let mut resolved = Vec::new();

        for requirement in requirements {
            let found = loader
                .active_packages()
                .iter()
                .any(|package| package.manifest.package_name == requirement.package);

            if found {
                resolved.push(requirement.package.clone());
            } else {
                missing.push(requirement.package.clone());
            }
        }

        ResolutionResult {
            resolved: missing.is_empty(),

            missing,

            resolved_packages: resolved,
        }
    }
}
