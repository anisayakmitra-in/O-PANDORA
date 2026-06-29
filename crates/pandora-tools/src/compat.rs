//! Constitutional compatibility layer for tools.
//!
//! The tools crate has its own ToolCapability, ToolManifest,
//! and ToolVersion types. The constitutional types in
//! pandora-types are the canonical source of truth. This
//! module provides From conversions from the tool types to
//! the constitutional types.
//!
//! Existing tool types are NOT modified. New code can use
//! the From conversions to migrate to the constitutional
//! types.

use pandora_types::constitutional::{
    ConstitutionalManifest, ManifestCapability, ManifestKind, ManifestVersion,
};

use crate::capability::{ToolCapability, ToolCapabilitySet};
use crate::manifest::ToolManifest;
use crate::types::ToolVersion;

impl From<&ToolCapability> for ManifestCapability {
    fn from(c: &ToolCapability) -> Self {
        ManifestCapability::new(c.id.clone(), c.id.clone())
    }
}

impl From<&ToolCapabilitySet> for Vec<ManifestCapability> {
    fn from(set: &ToolCapabilitySet) -> Self {
        set.all().iter().map(ManifestCapability::from).collect()
    }
}

fn parse_version(s: &str) -> ManifestVersion {
    // The ToolVersion::version field is a free-form
    // String (e.g. "1.2.3", "1.0", "v2"). The
    // constitutional ManifestVersion requires three
    // numbers. We parse what we can and default the rest.
    let trimmed = s.trim().trim_start_matches('v');
    let parts: Vec<&str> = trimmed.split('.').collect();
    let major = parts.first().and_then(|p| p.parse().ok()).unwrap_or(0);
    let minor = parts.get(1).and_then(|p| p.parse().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|p| p.parse().ok()).unwrap_or(0);
    ManifestVersion::new(major, minor, patch)
}

impl From<&ToolVersion> for ManifestVersion {
    fn from(v: &ToolVersion) -> Self {
        parse_version(&v.version)
    }
}

impl From<&ToolManifest> for ConstitutionalManifest {
    fn from(m: &ToolManifest) -> Self {
        let version: ManifestVersion = parse_version(&m.version);
        let mut cm = ConstitutionalManifest::new(
            m.name.clone(),
            ManifestKind::Tool,
            version,
            m.description.clone(),
        );
        for cap in m.capabilities.all() {
            cm.capabilities.push(ManifestCapability::from(cap));
        }
        cm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cap(id: &str) -> ToolCapability {
        ToolCapability {
            id: id.to_string(),
            required: false,
        }
    }

    fn cap_required(id: &str) -> ToolCapability {
        ToolCapability {
            id: id.to_string(),
            required: true,
        }
    }

    fn manifest(id: &str, name: &str, version: &str) -> ToolManifest {
        ToolManifest {
            id: id.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            description: format!("tool {}", name),
            input_schema: serde_json::Value::Null,
            output_schema: serde_json::Value::Null,
            mode: Default::default(),
            capabilities: ToolCapabilitySet::new(),
            permissions: Default::default(),
        }
    }

    #[test]
    fn tool_capability_to_manifest() {
        let c = cap("filesystem.read");
        let mc: ManifestCapability = (&c).into();
        assert_eq!(mc.name, "filesystem.read");
        // ToolCapability has no description field; the
        // constitutional ManifestCapability description
        // is the capability id itself.
        assert_eq!(mc.description, "filesystem.read");
    }

    #[test]
    fn tool_capability_set_to_vec() {
        let set = ToolCapabilitySet::new().with(cap("a")).with(cap("b"));
        let v: Vec<ManifestCapability> = (&set).into();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].name, "a");
        assert_eq!(v[1].name, "b");
    }

    #[test]
    fn tool_version_semver() {
        let tv = ToolVersion {
            name: "x".to_string(),
            version: "1.2.3".to_string(),
        };
        let mv: ManifestVersion = (&tv).into();
        assert_eq!(mv.major, 1);
        assert_eq!(mv.minor, 2);
        assert_eq!(mv.patch, 3);
    }

    #[test]
    fn tool_version_v_prefix() {
        let tv = ToolVersion {
            name: "x".to_string(),
            version: "v2.0.0".to_string(),
        };
        let mv: ManifestVersion = (&tv).into();
        assert_eq!(mv.major, 2);
        assert_eq!(mv.minor, 0);
        assert_eq!(mv.patch, 0);
    }

    #[test]
    fn tool_version_two_parts() {
        let tv = ToolVersion {
            name: "x".to_string(),
            version: "1.0".to_string(),
        };
        let mv: ManifestVersion = (&tv).into();
        assert_eq!(mv.major, 1);
        assert_eq!(mv.minor, 0);
        assert_eq!(mv.patch, 0);
    }

    #[test]
    fn tool_manifest_to_constitutional() {
        let m = manifest("fs.read", "FileRead", "1.0.0");
        let cm: ConstitutionalManifest = (&m).into();
        assert_eq!(cm.identity.name, "FileRead");
        assert_eq!(cm.identity.kind, ManifestKind::Tool);
        assert_eq!(cm.identity.version.major, 1);
    }

    #[test]
    fn tool_manifest_with_capabilities() {
        let mut m = manifest("fs.read", "FileRead", "1.0.0");
        m = m.with_capabilities(ToolCapabilitySet::new().with(cap_required("filesystem")));
        let cm: ConstitutionalManifest = (&m).into();
        assert_eq!(cm.capabilities.len(), 1);
        assert_eq!(cm.capabilities[0].name, "filesystem");
    }
}
