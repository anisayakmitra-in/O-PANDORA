use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageType {
    MetaHarness,

    Gene,

    Plugin,

    Skill,

    Evaluator,

    RuntimeExtension,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencySpec {
    pub name: String,

    pub minimum_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSpec {
    pub permission: String,

    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    pub package_name: String,

    pub package_type: PackageType,

    pub version: String,

    pub description: String,

    pub author: String,

    pub dependencies: Vec<DependencySpec>,

    pub permissions: Vec<PermissionSpec>,

    pub capabilities: Vec<String>,

    pub compatible_runtimes: Vec<String>,
}

pub struct ManifestValidator;

impl ManifestValidator {
    pub fn validate(manifest: &PackageManifest) -> bool {
        !manifest.package_name.is_empty() && !manifest.version.is_empty()
    }

    pub fn requires_permission(manifest: &PackageManifest, permission: &str) -> bool {
        manifest
            .permissions
            .iter()
            .any(|entry| entry.permission == permission && entry.required)
    }
}
