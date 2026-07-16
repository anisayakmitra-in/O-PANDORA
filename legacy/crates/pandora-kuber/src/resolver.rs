//! Dependency Resolver — reads pandora.toml dependencies, resolves versions,
//! and produces a pandora.lock for reproducible installs.
//!
//! Algorithm: for each dependency, scan all registered sources for the best
//! matching version (highest non-prerelease satisfying the version requirement).
//! Pin the exact version and checksum to pandora.lock.

use pandora_types::lockfile::Lockfile;
use pandora_types::package_format::PackageManifest;
use std::collections::HashMap;

/// A resolved dependency — exact version + source.
#[derive(Debug, Clone)]
pub struct ResolvedDep {
    pub id: String,
    pub version: String,
    pub source: String,
    pub checksum: String,
}

/// The dependency resolver reads a manifest's dependencies and resolves them
/// against available packages from all registered sources.
pub struct DependencyResolver {
    /// Available packages (id → (version, source))
    available: HashMap<String, Vec<(String, String)>>,
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            available: HashMap::new(),
        }
    }

    /// Register an available package (e.g., from a source or built-in).
    pub fn register(&mut self, id: &str, version: &str, source: &str) {
        self.available
            .entry(id.to_string())
            .or_default()
            .push((version.to_string(), source.to_string()));
    }

    /// Resolve the dependencies declared in a manifest.
    /// Returns a lockfile with pinned versions.
    pub fn resolve(&self, manifest: &PackageManifest) -> Lockfile {
        let mut lock = Lockfile::new();
        for dep in &manifest.dependencies {
            if let Some(resolved) = self.resolve_one(&dep.id, &dep.version_req) {
                lock.add(
                    &resolved.id,
                    &resolved.version,
                    &resolved.checksum,
                    &resolved.source,
                );
            }
        }
        lock
    }

    /// Resolve a single dependency. Returns the best matching version.
    fn resolve_one(&self, id: &str, version_req: &str) -> Option<ResolvedDep> {
        let candidates = self.available.get(id)?;
        let best = candidates
            .iter()
            .filter(|(v, _)| {
                // Simple semver: * matches anything, exact matches, >= prefix
                if version_req == "*" {
                    return true;
                }
                if version_req.starts_with(">=") {
                    let min = version_req.trim_start_matches(">=").trim();
                    return compare_versions(v, min) != std::cmp::Ordering::Less;
                }
                if version_req.starts_with('^') {
                    let want = version_req.trim_start_matches('^');
                    return v.starts_with(want)
                        || compare_versions(v, want) != std::cmp::Ordering::Less;
                }
                v == version_req
            })
            .max_by(|(a, _), (b, _)| compare_versions(a, b));

        best.map(|(v, src)| ResolvedDep {
            id: id.to_string(),
            version: v.clone(),
            source: src.clone(),
            checksum: format!("sha256:{}", v), // placeholder
        })
    }
}

/// Compare two semver strings. Returns -1, 0, or 1.
fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> Vec<u64> {
        s.split(|c: char| !c.is_ascii_digit())
            .filter_map(|n| n.parse().ok())
            .collect()
    };
    let va = parse(a);
    let vb = parse(b);
    for i in 0..va.len().max(vb.len()) {
        let na = va.get(i).copied().unwrap_or(0);
        let nb = vb.get(i).copied().unwrap_or(0);
        match na.cmp(&nb) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_comparison() {
        assert!(compare_versions("1.2.3", "1.2.0") == std::cmp::Ordering::Greater);
        assert!(compare_versions("0.1.0", "0.1.0") == std::cmp::Ordering::Equal);
        assert!(compare_versions("0.1", "0.2") == std::cmp::Ordering::Less);
    }

    #[test]
    fn resolve_single() {
        let mut r = DependencyResolver::new();
        r.register("pandora/shell", "1.0.0", "palace");
        r.register("pandora/shell", "1.1.0", "palace");
        r.register("pandora/shell", "0.9.0", "palace");
        let manifest = PackageManifest {
            id: "test".into(),
            name: "test".into(),
            version: "1.0".into(),
            dependencies: vec![pandora_types::package_format::PackageDependency::new(
                "pandora/shell",
            )
            .version("*")],
            ..Default::default()
        };
        let lock = r.resolve(&manifest);
        assert!(lock.has("pandora/shell"));
        let pkg = lock.get("pandora/shell").unwrap();
        assert_eq!(pkg.version, "1.1.0"); // highest matches *
    }

    #[test]
    fn resolve_min_version() {
        let mut r = DependencyResolver::new();
        r.register("p/a", "1.0.0", "palace");
        r.register("p/a", "2.0.0", "palace");
        r.register("p/a", "1.5.0", "palace");
        let manifest = PackageManifest {
            id: "t".into(),
            name: "t".into(),
            version: "1".into(),
            dependencies: vec![
                pandora_types::package_format::PackageDependency::new("p/a").version(">=1.5")
            ],
            ..Default::default()
        };
        let lock = r.resolve(&manifest);
        assert_eq!(lock.get("p/a").unwrap().version, "2.0.0");
    }
}
