#![allow(clippy::possible_missing_else)]
//! Pandora KUBER — gene distribution system.
//! Search, install, score, and manage gene packages.

use pandora_shadow_council::ShadowCouncil;
use pandora_types::gene_package::discover_gene_packages;
use serde::{Deserialize, Serialize};
use std::path::Path;
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
}
// Kuber is now Send+Sync via Arc<RwLock<_>>

impl Kuber {
    pub fn new(council: Arc<RwLock<ShadowCouncil>>) -> Self {
        let mut s = Self {
            council,
            sources: Vec::new(),
        };
        let pkg_dir = pandora_types::gene_package::packages_dir();
        if pkg_dir.exists() {
            s.add_source("local", &pkg_dir.to_string_lossy());
        }
        s
    }
    fn council_read(&self) -> std::sync::RwLockReadGuard<'_, ShadowCouncil> {
        self.council.read().unwrap()
    }
    fn council_write(&self) -> std::sync::RwLockWriteGuard<'_, ShadowCouncil> {
        self.council.write().unwrap()
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

    pub fn install(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        if let Ok(home) = std::env::var("HOME") {
            let lock = std::path::PathBuf::from(home).join(".pandora/pandora.lock");
            if lock.exists() {
                println!("[lockfile] pandora.lock found");
            }
        }
        let paths: Vec<String> = self.sources.iter().map(|s| s.path.clone()).collect();
        for path in &paths {
            let packages = discover_gene_packages(path);
            if packages.iter().any(|p| p.manifest.id == id) {
                self.council_write().load_gene_packages(path).map(|_| ())?;
            }
        }
        if crate::builtin::find(id).is_some() {
            return Ok(());
        }
        Err(to_err(format!("Package not found: {id}")))
    }

    pub fn uninstall(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.council_write()
            .genes
            .unregister(id)
            .map_err(pandora_types::PandoraError::Internal)
    }
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

    #[allow(clippy::possible_missing_else)]
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

pub mod builtin;
pub mod skill;

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
                .unwrap()
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
        std::fs::create_dir_all(d.join("g1").join("src")).unwrap();
        std::fs::write(
            d.join("g1").join("gene.toml"),
            "id = \"g1\"\nname = \"G1\"\nkind = \"tool\"\nversion = \"1.0\"\nauthor = \"me\"\n",
        )
        .unwrap();
        std::fs::write(
            d.join("g1").join("src").join("lib.rs"),
            "// g1\npub fn f() {}\n#[test]\nfn t() { f(); }\n",
        )
        .unwrap();
        let s = k.score(d.to_str().unwrap()).unwrap();
        assert!(s.overall() >= 5);
        assert!(s.tests >= 5);
        std::fs::remove_dir_all(d).unwrap();
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
        let path = crate::skill::scaffold("test-skill", d.to_str().unwrap()).unwrap();
        assert!(std::path::Path::new(&path).join("skill.toml").exists());
        std::fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn skill_discover_empty() {
        let d = tmp();
        std::fs::create_dir_all(&d).unwrap();
        assert!(crate::skill::discover(d.to_str().unwrap()).is_empty());
        std::fs::remove_dir_all(d).unwrap();
    }
}
pub mod resolver;
