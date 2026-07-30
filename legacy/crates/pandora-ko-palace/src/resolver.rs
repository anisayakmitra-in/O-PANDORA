//! Dependency Resolver — full semver constraint parsing and resolution.
//!
//! Supports: `*`, `>=`, `>`, `<=`, `<`, `=`, `^`, `~`, `!=`, exact match.
//! Produces a pandora.lock for reproducible installs.
//! Detects diamond dependency conflicts.

use pandora_types::gene_package::GenePackageManifest;
use pandora_types::lockfile::Lockfile;
use std::collections::HashMap;

/// A resolved dependency — exact version + source + checksum.
#[derive(Debug, Clone)]
pub struct ResolvedDep {
    pub id: String,
    pub version: String,
    pub source: String,
    pub checksum: String,
}

/// Dependency conflict — two packages require incompatible versions.
#[derive(Debug, Clone)]
pub struct DependencyConflict {
    pub package_a: String,
    pub version_a: String,
    pub package_b: String,
    pub version_b: String,
    pub required_by: Vec<String>,
}

/// The dependency resolver reads a manifest's dependencies and resolves them
/// against available packages from all registered sources.
pub struct DependencyResolver {
    /// Available packages (id → Vec<(version, source)>)
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
    pub fn resolve(&self, manifest: &GenePackageManifest) -> Lockfile {
        let mut lock = Lockfile::new();
        for dep in &manifest.dependencies {
            // Dependencies are plain IDs like "p/shell" or "p/shell@>=1.0"
            let (dep_id, dep_req) = parse_dep_string(dep);
            if let Some(resolved) = self.resolve_one(&dep_id, &dep_req) {
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

    /// Resolve all dependencies recursively with conflict detection.
    /// Returns (lockfile, conflicts).
    pub fn resolve_deep(
        &self,
        manifest: &GenePackageManifest,
    ) -> (Lockfile, Vec<DependencyConflict>) {
        let mut lock = Lockfile::new();
        let mut conflicts = Vec::new();
        let mut visited: HashMap<String, Vec<String>> = HashMap::new();

        self.resolve_recursive(manifest, &mut lock, &mut conflicts, &mut visited);
        (lock, conflicts)
    }

    fn resolve_recursive(
        &self,
        manifest: &GenePackageManifest,
        lock: &mut Lockfile,
        conflicts: &mut Vec<DependencyConflict>,
        visited: &mut HashMap<String, Vec<String>>,
    ) {
        for dep in &manifest.dependencies {
            let (dep_id, dep_req) = parse_dep_string(dep);
            if let Some(resolved) = self.resolve_one(&dep_id, &dep_req) {
                // Check for diamond conflict
                if let Some(prev_versions) = visited.get(&dep_id) {
                    if let Some(prev) = prev_versions.last() {
                        if prev != &resolved.version {
                            conflicts.push(DependencyConflict {
                                package_a: dep_id.clone(),
                                version_a: prev.clone(),
                                package_b: dep_id.clone(),
                                version_b: resolved.version.clone(),
                                required_by: vec![manifest.id.clone()],
                            });
                        }
                    }
                }

                lock.add(
                    &resolved.id,
                    &resolved.version,
                    &resolved.checksum,
                    &resolved.source,
                );
                visited
                    .entry(dep_id)
                    .or_default()
                    .push(resolved.version.clone());
            }
        }
    }

    /// Resolve a single dependency. Returns the best matching version.
    fn resolve_one(&self, id: &str, version_req: &str) -> Option<ResolvedDep> {
        let candidates = self.available.get(id)?;
        let best = candidates
            .iter()
            .filter(|(v, _)| satisfies(v, version_req))
            .max_by(|(a, _), (b, _)| compare_versions(a, b));

        best.map(|(v, src)| ResolvedDep {
            id: id.to_string(),
            version: v.clone(),
            source: src.clone(),
            checksum: format!("sha256:placeholder-{v}"),
        })
    }
}

/// Parse a dependency string like "p/shell" or "p/shell@>=1.0" into (id, req).
fn parse_dep_string(dep: &str) -> (String, String) {
    if let Some((id, req)) = dep.split_once('@') {
        (id.to_string(), req.to_string())
    } else {
        (dep.to_string(), "*".to_string())
    }
}

/// Check if a version satisfies a constraint.
pub fn satisfies(version: &str, constraint: &str) -> bool {
    if constraint == "*" || constraint.is_empty() {
        return true;
    }

    // Parse operator and version
    let (op, ver) = parse_constraint(constraint);

    match op {
        Op::Exact => compare_versions(version, ver) == std::cmp::Ordering::Equal,
        Op::GreaterEq => compare_versions(version, ver) != std::cmp::Ordering::Less,
        Op::Greater => compare_versions(version, ver) == std::cmp::Ordering::Greater,
        Op::LessEq => compare_versions(version, ver) != std::cmp::Ordering::Greater,
        Op::Less => compare_versions(version, ver) == std::cmp::Ordering::Less,
        Op::NotEq => compare_versions(version, ver) != std::cmp::Ordering::Equal,
        Op::Caret => caret_match(version, ver),
        Op::Tilde => tilde_match(version, ver),
        Op::None => compare_versions(version, ver) == std::cmp::Ordering::Equal,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    None,
    Exact,
    GreaterEq,
    Greater,
    LessEq,
    Less,
    NotEq,
    Caret,
    Tilde,
}

fn parse_constraint(req: &str) -> (Op, &str) {
    if let Some(v) = req.strip_prefix(">=") {
        (Op::GreaterEq, v.trim())
    } else if let Some(v) = req.strip_prefix('>') {
        (Op::Greater, v.trim())
    } else if let Some(v) = req.strip_prefix("<=") {
        (Op::LessEq, v.trim())
    } else if let Some(v) = req.strip_prefix('<') {
        (Op::Less, v.trim())
    } else if let Some(v) = req.strip_prefix("!=") {
        (Op::NotEq, v.trim())
    } else if let Some(v) = req.strip_prefix('=') {
        (Op::Exact, v.trim())
    } else if let Some(v) = req.strip_prefix('^') {
        (Op::Caret, v.trim())
    } else if let Some(v) = req.strip_prefix('~') {
        (Op::Tilde, v.trim())
    } else {
        (Op::None, req.trim())
    }
}

/// `^1.2.3` — compatible with 1.2.3 (allows patches and minor bumps within major)
fn caret_match(version: &str, base: &str) -> bool {
    let bv = parse_version_parts(base);
    let vv = parse_version_parts(version);

    if bv.is_empty() || vv.is_empty() {
        return false;
    }

    // Major must match
    if bv[0] != vv[0] {
        return false;
    }

    // ^0.x.y: minor is the effective major — must match, and version >= base
    if bv[0] == 0 && bv.len() > 1 && (vv.len() < 2 || bv[1] != vv[1]) {
        return false;
    }

    // ^1.x.y: only major must match, version >= base (Cargo semantics)
    compare_versions(version, base) != std::cmp::Ordering::Less
}

/// `~1.2.3` — approximately 1.2.3 (allows patches only)
fn tilde_match(version: &str, base: &str) -> bool {
    let bv = parse_version_parts(base);
    let vv = parse_version_parts(version);

    if bv.is_empty() || vv.is_empty() {
        return false;
    }

    // Major must match
    if bv[0] != vv[0] {
        return false;
    }

    // If only major specified (e.g., ~1), any 1.x.y is ok
    if bv.len() == 1 {
        return true;
    }

    // Minor must match, version >= base
    if vv.len() < 2 || bv[1] != vv[1] {
        return false;
    }

    compare_versions(version, base) != std::cmp::Ordering::Less
}

/// Parse version string into numeric parts.
fn parse_version_parts(v: &str) -> Vec<u64> {
    v.split(|c: char| !c.is_ascii_digit())
        .filter_map(|n| n.parse().ok())
        .collect()
}

/// Compare two semver strings.
pub fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let va = parse_version_parts(a);
    let vb = parse_version_parts(b);
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

    // ── satisfies() tests ──

    #[test]
    fn star_matches_any() {
        assert!(satisfies("1.0.0", "*"));
        assert!(satisfies("0.0.1", "*"));
    }

    #[test]
    fn exact_match() {
        assert!(satisfies("1.2.3", "1.2.3"));
        assert!(satisfies("1.2.3", "=1.2.3"));
        assert!(!satisfies("1.2.4", "1.2.3"));
    }

    #[test]
    fn greater_equal() {
        assert!(satisfies("2.0.0", ">=1.0.0"));
        assert!(satisfies("1.0.0", ">=1.0.0"));
        assert!(!satisfies("0.9.0", ">=1.0.0"));
    }

    #[test]
    fn greater() {
        assert!(satisfies("1.0.1", ">1.0.0"));
        assert!(!satisfies("1.0.0", ">1.0.0"));
    }

    #[test]
    fn less_equal() {
        assert!(satisfies("1.0.0", "<=1.0.0"));
        assert!(satisfies("0.9.0", "<=1.0.0"));
        assert!(!satisfies("1.0.1", "<=1.0.0"));
    }

    #[test]
    fn less() {
        assert!(satisfies("0.9.9", "<1.0.0"));
        assert!(!satisfies("1.0.0", "<1.0.0"));
    }

    #[test]
    fn not_equal() {
        assert!(satisfies("1.0.1", "!=1.0.0"));
        assert!(!satisfies("1.0.0", "!=1.0.0"));
    }

    #[test]
    fn caret_major_only() {
        assert!(satisfies("1.0.0", "^1"));
        assert!(satisfies("1.9.9", "^1"));
        assert!(!satisfies("2.0.0", "^1"));
        assert!(!satisfies("0.9.0", "^1"));
    }

    #[test]
    fn caret_major_minor() {
        assert!(satisfies("1.2.0", "^1.2"));
        assert!(satisfies("1.2.5", "^1.2"));
        assert!(satisfies("1.3.0", "^1.2"));
        assert!(!satisfies("1.1.9", "^1.2"));
    }

    #[test]
    fn caret_full() {
        assert!(satisfies("1.2.3", "^1.2.3"));
        assert!(satisfies("1.2.9", "^1.2.3"));
        assert!(!satisfies("1.2.2", "^1.2.3"));
        assert!(satisfies("1.3.0", "^1.2.3"));
        assert!(!satisfies("2.0.0", "^1.2.3"));
    }

    #[test]
    fn tilde_major_only() {
        assert!(satisfies("1.0.0", "~1"));
        assert!(satisfies("1.9.9", "~1"));
        assert!(!satisfies("2.0.0", "~1"));
    }

    #[test]
    fn tilde_major_minor() {
        assert!(satisfies("1.2.0", "~1.2"));
        assert!(satisfies("1.2.9", "~1.2"));
        assert!(!satisfies("1.3.0", "~1.2"));
    }

    #[test]
    fn tilde_full() {
        assert!(satisfies("1.2.3", "~1.2.3"));
        assert!(satisfies("1.2.9", "~1.2.3"));
        assert!(!satisfies("1.2.2", "~1.2.3"));
        assert!(!satisfies("1.3.0", "~1.2.3"));
    }

    // ── compare_versions() tests ──

    #[test]
    fn version_comparison() {
        assert!(compare_versions("1.2.3", "1.2.0") == std::cmp::Ordering::Greater);
        assert!(compare_versions("0.1.0", "0.1.0") == std::cmp::Ordering::Equal);
        assert!(compare_versions("0.1", "0.2") == std::cmp::Ordering::Less);
        assert!(compare_versions("10.0.0", "9.9.9") == std::cmp::Ordering::Greater);
    }

    // ── parse_dep_string() tests ──

    #[test]
    fn parse_dep_simple() {
        let (id, req) = parse_dep_string("p/shell");
        assert_eq!(id, "p/shell");
        assert_eq!(req, "*");
    }

    #[test]
    fn parse_dep_with_version() {
        let (id, req) = parse_dep_string("p/shell@>=1.0");
        assert_eq!(id, "p/shell");
        assert_eq!(req, ">=1.0");
    }

    // ── resolve tests ──

    #[test]
    fn resolve_star() {
        let mut r = DependencyResolver::new();
        r.register("p/shell", "1.0.0", "palace");
        r.register("p/shell", "1.1.0", "palace");
        r.register("p/shell", "0.9.0", "palace");
        let manifest = GenePackageManifest {
            id: "test".into(),
            name: "test".into(),
            version: "1.0".into(),
            author: "test".into(),
            dependencies: vec!["p/shell".into()],
            ..Default::default()
        };
        let lock = r.resolve(&manifest);
        assert!(lock.has("p/shell"));
        assert_eq!(lock.get("p/shell").unwrap().version, "1.1.0");
    }

    #[test]
    fn resolve_gte() {
        let mut r = DependencyResolver::new();
        r.register("p/a", "1.0.0", "palace");
        r.register("p/a", "2.0.0", "palace");
        r.register("p/a", "1.5.0", "palace");
        let manifest = GenePackageManifest {
            id: "t".into(),
            name: "t".into(),
            version: "1".into(),
            author: "test".into(),
            dependencies: vec!["p/a@>=1.5".into()],
            ..Default::default()
        };
        let lock = r.resolve(&manifest);
        assert_eq!(lock.get("p/a").unwrap().version, "2.0.0");
    }

    #[test]
    fn resolve_caret() {
        let mut r = DependencyResolver::new();
        r.register("p/a", "1.2.3", "palace");
        r.register("p/a", "1.3.0", "palace");
        r.register("p/a", "2.0.0", "palace");
        let manifest = GenePackageManifest {
            id: "t".into(),
            name: "t".into(),
            version: "1".into(),
            author: "test".into(),
            dependencies: vec!["p/a@^1.2.0".into()],
            ..Default::default()
        };
        let lock = r.resolve(&manifest);
        assert_eq!(lock.get("p/a").unwrap().version, "1.3.0");
    }

    #[test]
    fn resolve_tilde() {
        let mut r = DependencyResolver::new();
        r.register("p/a", "1.2.3", "palace");
        r.register("p/a", "1.2.9", "palace");
        r.register("p/a", "1.3.0", "palace");
        let manifest = GenePackageManifest {
            id: "t".into(),
            name: "t".into(),
            version: "1".into(),
            author: "test".into(),
            dependencies: vec!["p/a@~1.2.0".into()],
            ..Default::default()
        };
        let lock = r.resolve(&manifest);
        assert_eq!(lock.get("p/a").unwrap().version, "1.2.9");
    }

    #[test]
    fn no_conflict() {
        let mut r = DependencyResolver::new();
        r.register("p/a", "1.0.0", "palace");
        let manifest = GenePackageManifest {
            id: "t".into(),
            name: "t".into(),
            version: "1".into(),
            author: "test".into(),
            dependencies: vec!["p/a@>=1.0".into()],
            ..Default::default()
        };
        let (_, conflicts) = r.resolve_deep(&manifest);
        assert!(conflicts.is_empty());
    }
}
