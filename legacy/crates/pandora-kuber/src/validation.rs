//! Manifest Validation — validates gene.toml before install.
//!
//! Every package must pass validation before it can be installed.
//! This prevents broken, malicious, or malformed packages from entering
//! the local registry.

use pandora_types::gene_package::GenePackageManifest;

/// Validation error for a package manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for ManifestError {}

/// Validate a package manifest. Returns a list of errors (empty = valid).
pub fn validate_manifest(manifest: &GenePackageManifest) -> Vec<ManifestError> {
    let mut errors = Vec::new();

    // ── Required fields ──

    if manifest.id.is_empty() {
        errors.push(ManifestError {
            field: "id".into(),
            message: "package id is required".into(),
        });
    } else if !manifest
        .id
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        errors.push(ManifestError {
            field: "id".into(),
            message: "id must be alphanumeric with hyphens/underscores only".into(),
        });
    }

    if manifest.name.is_empty() {
        errors.push(ManifestError {
            field: "name".into(),
            message: "package name is required".into(),
        });
    }

    if manifest.version.is_empty() {
        errors.push(ManifestError {
            field: "version".into(),
            message: "version is required".into(),
        });
    } else if !is_valid_semver(&manifest.version) {
        errors.push(ManifestError {
            field: "version".into(),
            message: format!("invalid semver: {}", manifest.version),
        });
    }

    if manifest.author.is_empty() {
        errors.push(ManifestError {
            field: "author".into(),
            message: "author is required".into(),
        });
    }

    // ── Dependencies ──

    for dep in &manifest.dependencies {
        if dep.is_empty() {
            errors.push(ManifestError {
                field: "dependencies".into(),
                message: "dependency id cannot be empty".into(),
            });
        }
    }

    // ── Gene entries (capabilities) ──

    for cap in &manifest.capabilities {
        if cap.is_empty() {
            errors.push(ManifestError {
                field: "capabilities".into(),
                message: "capability cannot be empty".into(),
            });
        }
    }

    errors
}

/// Validate and return Ok(()) or Err with all errors joined.
pub fn validate_strict(manifest: &GenePackageManifest) -> Result<(), pandora_types::PandoraError> {
    let errors = validate_manifest(manifest);
    if errors.is_empty() {
        Ok(())
    } else {
        let msgs: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        Err(pandora_types::PandoraError::Validation(format!(
            "Manifest validation failed:\n  {}",
            msgs.join("\n  ")
        )))
    }
}

/// Check if a string is valid semver (major.minor.patch, optional pre-release/build).
fn is_valid_semver(v: &str) -> bool {
    // Strip pre-release and build metadata: 1.0.0-beta.1 -> 1.0.0
    let core = v.split('-').next().unwrap_or(v);
    let parts: Vec<&str> = core.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    for part in &parts {
        if part.is_empty() || !part.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
    }
    true
}

/// Check if a version requirement string is valid.
pub fn is_valid_version_req(req: &str) -> bool {
    if req == "*" {
        return true;
    }
    // Strip operators: >=, >, <=, <, =, ^, ~, !=
    let mut trimmed = req;
    for prefix in &["!=", ">=", "<=", ">", "<", "=", "^", "~"] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            trimmed = rest;
            break;
        }
    }
    is_valid_semver(trimmed.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_types::gene_package::GenePackageManifest;

    fn valid_manifest() -> GenePackageManifest {
        GenePackageManifest {
            id: "test-pkg".into(),
            name: "Test Package".into(),
            version: "1.0.0".into(),
            author: "test-author".into(),
            ..Default::default()
        }
    }

    #[test]
    fn valid_manifest_passes() {
        let m = valid_manifest();
        assert!(validate_manifest(&m).is_empty());
    }

    #[test]
    fn empty_id_fails() {
        let mut m = valid_manifest();
        m.id = "".into();
        let errors = validate_manifest(&m);
        assert!(errors.iter().any(|e| e.field == "id"));
    }

    #[test]
    fn invalid_id_chars_fail() {
        let mut m = valid_manifest();
        m.id = "has spaces!".into();
        let errors = validate_manifest(&m);
        assert!(errors.iter().any(|e| e.field == "id"));
    }

    #[test]
    fn empty_version_fails() {
        let mut m = valid_manifest();
        m.version = "".into();
        let errors = validate_manifest(&m);
        assert!(errors.iter().any(|e| e.field == "version"));
    }

    #[test]
    fn invalid_version_fails() {
        let mut m = valid_manifest();
        m.version = "not-a-version".into();
        let errors = validate_manifest(&m);
        assert!(errors.iter().any(|e| e.field == "version"));
    }

    #[test]
    fn two_part_version_passes() {
        let mut m = valid_manifest();
        m.version = "1.0".into();
        assert!(validate_manifest(&m).is_empty());
    }

    #[test]
    fn prerelease_version_passes() {
        let mut m = valid_manifest();
        m.version = "1.0.0-beta.1".into();
        assert!(validate_manifest(&m).is_empty());
    }

    #[test]
    fn empty_author_fails() {
        let mut m = valid_manifest();
        m.author = "".into();
        let errors = validate_manifest(&m);
        assert!(errors.iter().any(|e| e.field == "author"));
    }

    #[test]
    fn validate_strict_ok() {
        assert!(validate_strict(&valid_manifest()).is_ok());
    }

    #[test]
    fn validate_strict_err() {
        let mut m = valid_manifest();
        m.id = "".into();
        m.version = "".into();
        assert!(validate_strict(&m).is_err());
    }

    #[test]
    fn semver_valid() {
        assert!(is_valid_semver("1.0.0"));
        assert!(is_valid_semver("0.1"));
        assert!(is_valid_semver("1.0.0-beta.1"));
        assert!(is_valid_semver("1.0.0-rc.1+build.123"));
    }

    #[test]
    fn semver_invalid() {
        assert!(!is_valid_semver(""));
        assert!(!is_valid_semver("1"));
        assert!(!is_valid_semver("abc"));
        assert!(!is_valid_semver("1.2.3.4"));
    }

    #[test]
    fn version_req_valid() {
        assert!(is_valid_version_req("*"));
        assert!(is_valid_version_req(">=1.0.0"));
        assert!(is_valid_version_req(">1.0.0"));
        assert!(is_valid_version_req("<=2.0.0"));
        assert!(is_valid_version_req("<2.0.0"));
        assert!(is_valid_version_req("^1.0"));
        assert!(is_valid_version_req("~1.2.3"));
        assert!(is_valid_version_req("!=1.0.0"));
        assert!(is_valid_version_req("1.0.0"));
    }
}
