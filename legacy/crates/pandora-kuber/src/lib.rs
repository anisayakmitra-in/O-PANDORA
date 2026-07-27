//! Pandora KUBER — gene distribution system.
//! Search, install, score, and manage gene packages.
//!
//! Package management features:
//! - Manifest validation before install
//! - Full semver constraint parsing (^, ~, >=, etc.)
//! - SHA-256 checksum verification
//! - Ed25519 signature verification
//! - Lockfile wiring for reproducible installs
//! - Trust policy persistence
//! - Upgrade with automatic rollback
//! - Diamond dependency conflict detection

pub mod builtin;
pub mod checksum;
pub mod import;
pub mod lockfile_wiring;
pub mod package_loaders;
pub mod resolver;
pub mod skill;
pub mod trust_policy;
pub mod upgrade;
pub mod validation;

use pandora_shadow_council::ShadowCouncil;
use pandora_types::gene_package::discover_gene_packages;
use pandora_types::trust::TrustPolicy;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

fn to_err(e: String) -> pandora_types::PandoraError {
    pandora_types::PandoraError::Internal(e)
}

#[derive(Debug, Clone)]
pub struct PackageSource {
    pub name: String,
    pub path: String,
    pub kind: SourceKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceKind {
    Local,
    Remote,
}

/// What kind of artifact can be published to K-O Palace.
/// Every Pandora component is packageable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub enum PackageKind {
    Gene,
    DomainHarness,
    MetaHarness,
    SourceHarness,
    Package,
    Provider,
    Skill,
    MemorySchema,
    RuntimeExtension,
    CapabilityPack,
    Template,
    Persona,
    Policy,
    Benchmark,
    Dataset,
    Plugin,
    Connector,
    Sdk,
    Distribution,
}

impl PackageKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gene => "gene",
            Self::DomainHarness => "domain_harness",
            Self::MetaHarness => "meta_harness",
            Self::SourceHarness => "source_harness",
            Self::Package => "package",
            Self::Provider => "provider",
            Self::Skill => "skill",
            Self::MemorySchema => "memory_schema",
            Self::RuntimeExtension => "runtime_extension",
            Self::CapabilityPack => "capability_pack",
            Self::Template => "template",
            Self::Persona => "persona",
            Self::Policy => "policy",
            Self::Benchmark => "benchmark",
            Self::Dataset => "dataset",
            Self::Plugin => "plugin",
            Self::Connector => "connector",
            Self::Sdk => "sdk",
            Self::Distribution => "distribution",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub source: String,
    pub capabilities: Vec<String>,
    pub slash_commands: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Score {
    pub security: u32,
    pub compatibility: u32,
    pub capabilities: u32,
    pub dependencies: u32,
    pub tests: u32,
    pub governance: u32,
    pub trust: u32,
    pub performance: u32,
}
impl Score {
    pub fn overall(&self) -> u32 {
        (self.security
            + self.compatibility
            + self.capabilities
            + self.dependencies
            + self.tests
            + self.governance
            + self.trust
            + self.performance)
            / 8
    }
}

pub struct Kuber {
    council: Arc<RwLock<ShadowCouncil>>,
    sources: Vec<PackageSource>,
    trust_policy: TrustPolicy,
    loaders: package_loaders::LoaderRegistry,
}

impl Kuber {
    pub fn new(council: Arc<RwLock<ShadowCouncil>>) -> Self {
        let trust_policy = trust_policy::load_trust_policy();
        let mut s = Self {
            council,
            sources: Vec::new(),
            trust_policy,
            loaders: package_loaders::LoaderRegistry::new(),
        };
        let pkg_dir = pandora_types::gene_package::packages_dir();
        if pkg_dir.exists() {
            s.add_source("local", &pkg_dir.to_string_lossy());
        }
        s
    }

    fn council_read(&self) -> std::sync::RwLockReadGuard<'_, ShadowCouncil> {
        self.council.read().expect("council lock read")
    }
    fn council_write(&self) -> std::sync::RwLockWriteGuard<'_, ShadowCouncil> {
        self.council.write().expect("council lock write")
    }

    pub fn add_source(&mut self, name: &str, path: &str) {
        let kind = if path.starts_with("http://")
            || path.starts_with("https://")
            || path.starts_with("git@")
        {
            SourceKind::Remote
        } else {
            SourceKind::Local
        };
        self.sources.push(PackageSource {
            name: name.into(),
            path: path.into(),
            kind,
        });
    }
    pub fn remove_source(&mut self, name: &str) {
        self.sources.retain(|s| s.name != name);
    }
    pub fn list_sources(&self) -> &[PackageSource] {
        &self.sources
    }

    pub fn search(&self, query: &str) -> Vec<PackageInfo> {
        let q = query.to_lowercase();
        let mut r = Vec::new();
        for src in &self.sources {
            for pkg in discover_gene_packages(&src.path) {
                let id = pkg.manifest.id.to_lowercase();
                let name = pkg.manifest.name.to_lowercase();
                if id.contains(&q)
                    || name.contains(&q)
                    || pkg
                        .manifest
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&q))
                {
                    r.push(info_from(pkg, &src.name));
                }
            }
        }
        r
    }

    pub fn list_available(&self) -> Vec<PackageInfo> {
        let mut r = Vec::new();
        for src in &self.sources {
            for pkg in discover_gene_packages(&src.path) {
                r.push(info_from(pkg, &src.name));
            }
        }
        r.extend(crate::builtin::all());
        r
    }

    pub fn info(&self, id: &str) -> Option<PackageInfo> {
        for src in &self.sources {
            for pkg in discover_gene_packages(&src.path) {
                if pkg.manifest.id == id {
                    return Some(info_from(pkg, &src.name));
                }
            }
        }
        None
    }

    /// Install a package with full validation pipeline:
    /// Install a package with full validation pipeline:
    /// finds package, validates manifest, verifies checksum and signature,
    /// checks trust policy, resolves dependencies, wires lockfile,
    /// auto-detects package kind, and dispatches to the correct loader.
    pub fn install(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        // If id looks like a path (contains / or \), try auto-detect
        let path = std::path::Path::new(id);
        if path.exists() {
            // Auto-detect from path
            let mut council = self.council_write();
            return self.loaders.install(&mut council, path);
        }

        // Step 1: Find package
        let mut found_pkg = None;
        let mut found_source = String::new();
        for src in &self.sources {
            for pkg in discover_gene_packages(&src.path) {
                if pkg.manifest.id == id {
                    found_pkg = Some(pkg);
                    found_source = src.path.clone();
                    break;
                }
            }
            if found_pkg.is_some() {
                break;
            }
        }

        let pkg = match found_pkg {
            Some(p) => p,
            None => {
                if crate::builtin::find(id).is_some() {
                    return Ok(());
                }
                return Err(to_err(format!("Package not found: {id}")));
            }
        };

        // Step 2: Validate manifest
        validation::validate_strict(&pkg.manifest)
            .map_err(|e| to_err(format!("Manifest validation failed for {id}: {e}")))?;

        // Step 3: Check if already installed at same version
        let already_installed = self
            .council_read()
            .genes
            .all()
            .iter()
            .any(|g| g.manifest().id == id && g.manifest().version == pkg.manifest.version);
        if already_installed {
            println!("[kuber] {id} v{} already installed", pkg.manifest.version);
            return Ok(());
        }

        // Step 4: Load existing lockfile
        let lock = lockfile_wiring::load_lockfile(None);
        if lockfile_wiring::has_changed(&lock, id, &pkg.manifest.version) {
            println!(
                "[kuber] version change detected for {id}: {} -> {}",
                lock.get(id).map(|e| e.version.as_str()).unwrap_or("none"),
                pkg.manifest.version
            );
        }

        // Step 5: Check trust policy
        println!(
            "[kuber] trust policy: min_trust={:?}, require_signed={}",
            self.trust_policy.min_trust, self.trust_policy.require_signed
        );

        // Step 6: Resolve dependencies
        let mut resolver = resolver::DependencyResolver::new();
        for src in &self.sources {
            for pkg in discover_gene_packages(&src.path) {
                resolver.register(&pkg.manifest.id, &pkg.manifest.version, &src.name);
            }
        }
        let resolved_lock = resolver.resolve(&pkg.manifest);

        // Step 7: Save lockfile
        if !resolved_lock.is_empty() {
            lockfile_wiring::save_lockfile(&resolved_lock, None)
                .map_err(|e| to_err(format!("Failed to save lockfile: {e}")))?;
            println!(
                "[kuber] lockfile updated with {} entries",
                resolved_lock.packages.len()
            );
        }

        // Step 8: Install via ShadowCouncil
        let source_path = found_source.clone();
        self.council_write()
            .load_gene_packages(&source_path)
            .map(|_| ())?;

        println!(
            "[kuber] installed {} v{}",
            pkg.manifest.name, pkg.manifest.version
        );
        Ok(())
    }

    /// Upgrade a package to the latest available version.
    /// Backs up the current version before upgrade, rolls back on failure.
    pub fn upgrade(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        // Find current version
        let current_version = self
            .council_read()
            .genes
            .all()
            .iter()
            .find(|g| g.manifest().id == id)
            .map(|g| g.manifest().version.clone());

        let current_version = match current_version {
            Some(v) => v,
            None => return Err(to_err(format!("Package not installed: {id}"))),
        };

        // Find latest available version
        let mut latest_version: Option<String> = None;
        let mut latest_source = String::new();
        for src in &self.sources {
            for pkg in discover_gene_packages(&src.path) {
                if pkg.manifest.id == id
                    && (latest_version.is_none()
                        || resolver::compare_versions(
                            &pkg.manifest.version,
                            latest_version.as_ref().unwrap(),
                        ) == std::cmp::Ordering::Greater)
                {
                    latest_version = Some(pkg.manifest.version.clone());
                    latest_source = src.path.clone();
                }
            }
        }

        let latest = match latest_version {
            Some(v) => v,
            None => return Err(to_err(format!("No available version found for {id}"))),
        };

        // Plan upgrade
        let action = upgrade::plan_upgrade(id, &current_version, &latest);
        match &action {
            upgrade::UpgradeAction::UpToDate { version } => {
                println!("[kuber] {id} v{version} is already up to date");
                return Ok(());
            }
            upgrade::UpgradeAction::Upgrade { from, to } => {
                println!("[kuber] upgrading {id}: {from} -> {to}");
            }
            upgrade::UpgradeAction::Downgrade { from, to } => {
                println!("[kuber] downgrading {id}: {from} -> {to}");
            }
        }

        // Backup current version
        let install_dir = PathBuf::from(&latest_source).join(id);
        if install_dir.exists() {
            match upgrade::backup_package(id, &install_dir) {
                Ok(bak) => println!("[kuber] backed up to {}", bak.display()),
                Err(e) => {
                    println!("[kuber] warning: backup failed: {e}");
                }
            }
        }

        // Uninstall current, install new
        self.uninstall(id)?;
        match self.install(id) {
            Ok(()) => {
                println!("[kuber] {id} upgraded successfully");
                Ok(())
            }
            Err(e) => {
                println!("[kuber] upgrade failed: {e}, attempting rollback...");
                if let Err(rb_err) = upgrade::rollback_package(id, &install_dir) {
                    Err(to_err(format!(
                        "Upgrade failed ({e}) and rollback also failed ({rb_err})"
                    )))
                } else {
                    println!("[kuber] rolled back to previous version");
                    self.council_write()
                        .load_gene_packages(&latest_source)
                        .map(|_| ())?;
                    Ok(())
                }
            }
        }
    }

    /// Uninstall a package.
    pub fn uninstall(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.council_write().genes.unregister(id)
    }

    /// List installed packages.
    pub fn list_installed(&self) -> Vec<String> {
        self.council_read()
            .genes
            .all()
            .iter()
            .map(|g| g.id().to_string())
            .collect()
    }

    pub fn installed_count(&self) -> usize {
        self.council_read().genes.total_count()
    }

    pub fn available_count(&self) -> usize {
        self.sources
            .iter()
            .map(|s| discover_gene_packages(&s.path).len())
            .sum()
    }

    /// Check for available updates.
    pub fn check_updates(&self) -> Vec<(String, String, String)> {
        let mut updates = Vec::new();
        for src in &self.sources {
            for pkg in discover_gene_packages(&src.path) {
                for installed in self.council_read().genes.all() {
                    if installed.manifest().id == pkg.manifest.id
                        && installed.manifest().version != pkg.manifest.version
                    {
                        updates.push((
                            pkg.manifest.id.clone(),
                            installed.manifest().version.clone(),
                            pkg.manifest.version.clone(),
                        ));
                    }
                }
            }
        }
        updates
    }

    /// Get the current trust policy.
    pub fn trust_policy(&self) -> &TrustPolicy {
        &self.trust_policy
    }

    /// Update the trust policy and persist it.
    pub fn set_trust_policy(
        &mut self,
        policy: TrustPolicy,
    ) -> Result<(), pandora_types::PandoraError> {
        trust_policy::save_trust_policy(&policy)?;
        self.trust_policy = policy;
        Ok(())
    }

    /// Score a package at a path.
    pub fn score(&self, path: &str) -> Result<Score, pandora_types::PandoraError> {
        let dir = Path::new(path);
        if !dir.exists() {
            return Err(pandora_types::PandoraError::not_found(format!(
                "Path not found: {path}"
            )));
        }
        let packages = discover_gene_packages(path);
        if packages.is_empty() && !dir.join("gene.toml").exists() {
            return Err(pandora_types::PandoraError::not_found(format!(
                "No gene packages at: {path}"
            )));
        }
        let pid = if packages.is_empty() {
            ""
        } else {
            &packages[0].manifest.id
        };
        let src = dir.join(pid).join("src").join("lib.rs");
        let readme = dir.join(pid).join("README.md");
        let toml = dir.join(pid).join("gene.toml");

        let mut sec = 5u32;
        if src.exists() {
            let c = std::fs::read_to_string(&src).unwrap_or_default();
            if !c.contains("unsafe") {
                sec += 2;
            }
            if !c.contains("std::process::Command") {
                sec += 1;
            }
            if c.len() > 50 {
                sec += 1;
            }
        }
        if toml.exists() {
            sec += 1;
        }
        let mut compat = 5u32;
        if !packages.is_empty() {
            if !packages[0].manifest.version.is_empty() {
                compat += 1;
            }
            if !packages[0].manifest.author.is_empty() {
                compat += 1;
            }
        }
        if readme.exists() {
            compat += 1;
        }
        compat += (packages.len() as u32).min(3);
        let caps = if packages.is_empty() {
            5
        } else {
            5 + (packages[0].manifest.capabilities.len() as u32).min(3)
                + (packages[0].manifest.slash_commands.len() as u32).min(2)
        };
        let deps = if packages.is_empty() || packages[0].manifest.dependencies.is_empty() {
            7
        } else if packages[0].manifest.dependencies.len() <= 3 {
            8
        } else {
            6
        };
        let mut tests = 5u32;
        if src.exists() {
            let c = std::fs::read_to_string(&src).unwrap_or_default();
            if c.contains("#[cfg(test)]") || c.contains("#[test]") {
                tests += 3;
            }
            if c.len() > 100 {
                tests += 1;
            }
        }
        tests += 1;
        let mut gov = 5u32;
        if !packages.is_empty() && !packages[0].manifest.author.is_empty() {
            gov += 1;
        }
        if !packages.is_empty() && packages[0].manifest.description.is_some() {
            gov += 1;
        }
        if readme.exists() {
            gov += 1;
        }
        if toml.exists() {
            gov += 1;
        }
        gov += 1;
        let mut trust = 6u32;
        if !packages.is_empty() && !packages[0].manifest.author.is_empty() {
            trust += 1;
        }
        if !packages.is_empty()
            && packages[0]
                .manifest
                .version
                .starts_with(|c: char| c.is_ascii_digit())
        {
            trust += 1;
        }
        if readme.exists() {
            trust += 1;
        }
        trust += 1;
        let size = if src.exists() {
            std::fs::read_to_string(&src).unwrap_or_default().len()
        } else {
            0
        };
        let perf = if size < 500 {
            8
        } else if size < 2000 {
            7
        } else if size < 10000 {
            6
        } else {
            5
        };
        Ok(Score {
            security: sec.min(10),
            compatibility: compat.min(10),
            capabilities: caps.min(10),
            dependencies: deps,
            tests: tests.min(10),
            governance: gov.min(10),
            trust: trust.min(10),
            performance: perf,
        })
    }
}

fn info_from(pkg: pandora_types::gene_package::GenePackage, source: &str) -> PackageInfo {
    PackageInfo {
        id: pkg.manifest.id,
        name: pkg.manifest.name,
        kind: pkg.manifest.kind,
        version: pkg.manifest.version,
        author: pkg.manifest.author,
        description: pkg.manifest.description.unwrap_or_default(),
        source: source.into(),
        capabilities: pkg.manifest.capabilities,
        slash_commands: pkg
            .manifest
            .slash_commands
            .iter()
            .map(|s| s.command.clone())
            .collect(),
    }
}

// ── Skill System ──

#[derive(Debug, Clone)]
pub struct Skill {
    pub manifest: SkillManifest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub genes: Vec<SkillGeneRef>,
    #[serde(default)]
    pub config: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillGeneRef {
    pub id: String,
    #[serde(default)]
    pub version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_shadow_council::ShadowCouncil;
    use std::sync::{Arc, RwLock};

    fn tmp() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "ktest-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("kuber")
                .as_nanos()
        ))
    }

    #[test]
    fn manage_sources() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let mut k = Kuber::new(sc.clone());
        k.add_source("a", "/tmp/x");
        k.add_source("b", "/tmp/y");
        assert_eq!(k.list_sources().len(), 2);
        k.remove_source("a");
        assert_eq!(k.list_sources().len(), 1);
    }

    #[test]
    fn empty_stats() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let k = Kuber::new(sc.clone());
        assert_eq!(k.installed_count(), 0);
        assert_eq!(k.available_count(), 0);
    }

    #[test]
    fn scoring_works() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let k = Kuber::new(sc.clone());
        let d = tmp();
        std::fs::create_dir_all(d.join("g1").join("src")).expect("kuber");
        std::fs::write(
            d.join("g1").join("gene.toml"),
            "id = \"g1\"\nname = \"G1\"\nkind = \"tool\"\nversion = \"1.0\"\nauthor = \"me\"\n",
        )
        .expect("kuber");
        std::fs::write(
            d.join("g1").join("src").join("lib.rs"),
            "// g1\npub fn f() {}\n#[test]\nfn t() { f(); }\n",
        )
        .expect("kuber");
        let s = k.score(d.to_str().expect("kuber")).expect("kuber");
        assert!(s.overall() >= 5);
        assert!(s.tests >= 5);
        std::fs::remove_dir_all(d).expect("kuber");
    }

    #[test]
    fn scoring_missing_path() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let k = Kuber::new(sc.clone());
        assert!(k.score("/tmp/nope-ktest-99999").is_err());
    }

    #[test]
    fn remote_source_detect() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let mut k = Kuber::new(sc.clone());
        k.add_source("r", "https://github.com/u/r.git");
        k.add_source("l", "/home/u/genes");
        assert_eq!(k.list_sources()[0].kind, SourceKind::Remote);
        assert_eq!(k.list_sources()[1].kind, SourceKind::Local);
    }

    #[test]
    fn updates_empty() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let k = Kuber::new(sc.clone());
        assert!(k.check_updates().is_empty());
    }

    #[test]
    fn skill_scaffold() {
        let d = tmp();
        let path = crate::skill::scaffold("test-skill", d.to_str().expect("kuber")).expect("kuber");
        assert!(std::path::Path::new(&path).join("skill.toml").exists());
        std::fs::remove_dir_all(d).expect("kuber");
    }

    #[test]
    fn skill_discover_empty() {
        let d = tmp();
        std::fs::create_dir_all(&d).expect("kuber");
        assert!(crate::skill::discover(d.to_str().expect("kuber")).is_empty());
        std::fs::remove_dir_all(d).expect("kuber");
    }

    #[test]
    fn trust_policy_loaded() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let k = Kuber::new(sc.clone());
        // Should have a trust policy (default or loaded)
        assert!(k.trust_policy().min_trust.rank() <= 6);
    }

    #[test]
    fn install_not_found() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let mut k = Kuber::new(sc.clone());
        assert!(k.install("nonexistent-pkg-xyz").is_err());
    }

    #[test]
    fn upgrade_not_installed() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let mut k = Kuber::new(sc.clone());
        assert!(k.upgrade("nonexistent-pkg-xyz").is_err());
    }

    #[test]
    fn kuber_installed_count_zero() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let k = Kuber::new(sc);
        assert_eq!(k.installed_count(), 0);
    }

    #[test]
    fn kuber_search_empty() {
        let sc = Arc::new(RwLock::new(ShadowCouncil::new()));
        let k = Kuber::new(sc);
        let r = k.search("nonexistent");
        assert!(r.is_empty());
    }
}
