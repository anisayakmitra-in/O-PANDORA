//! Pandora KUBER — gene distribution system.
//!
//! Search, install, score, and manage gene packages.

use pandora_shadow_council::ShadowCouncil;
use pandora_types::gene_package::discover_gene_packages;
use serde::{Deserialize, Serialize};
use std::path::Path;

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
    council: *mut ShadowCouncil,
    sources: Vec<PackageSource>,
}

unsafe impl Send for Kuber {}
unsafe impl Sync for Kuber {}

impl Kuber {
    pub fn new(council: &mut ShadowCouncil) -> Self {
        Self {
            council,
            sources: Vec::new(),
        }
    }

    fn council(&self) -> &ShadowCouncil {
        unsafe { &*self.council }
    }
    fn council_mut(&mut self) -> &mut ShadowCouncil {
        unsafe { &mut *self.council }
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
        let mut results = Vec::new();
        for source in &self.sources {
            for pkg in discover_gene_packages(&source.path) {
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
                    results.push(info_from(pkg, &source.name));
                }
            }
        }
        results
    }

    pub fn list_available(&self) -> Vec<PackageInfo> {
        let mut results = Vec::new();
        for source in &self.sources {
            for pkg in discover_gene_packages(&source.path) {
                results.push(info_from(pkg, &source.name));
            }
        }
        results
    }

    pub fn info(&self, id: &str) -> Option<PackageInfo> {
        for source in &self.sources {
            for pkg in discover_gene_packages(&source.path) {
                if pkg.manifest.id == id {
                    return Some(info_from(pkg, &source.name));
                }
            }
        }
        None
    }

    pub fn install(&mut self, id: &str) -> Result<(), String> {
        let paths: Vec<String> = self.sources.iter().map(|s| s.path.clone()).collect();
        for path in &paths {
            let packages = discover_gene_packages(path);
            if packages.iter().any(|p| p.manifest.id == id) {
                self.council_mut().load_gene_packages(path)?;
                return Ok(());
            }
        }
        Err(format!("Package not found: {}", id))
    }

    pub fn uninstall(&mut self, id: &str) -> Result<(), String> {
        self.council_mut().genes.unregister(id)
    }

    pub fn list_installed(&self) -> Vec<String> {
        self.council()
            .genes
            .all()
            .iter()
            .map(|g| g.id().to_string())
            .collect()
    }

    pub fn installed_count(&self) -> usize {
        self.council().genes.total_count()
    }

    pub fn available_count(&self) -> usize {
        self.sources
            .iter()
            .map(|s| discover_gene_packages(&s.path).len())
            .sum()
    }

    pub fn score(&self, path: &str) -> Result<Score, String> {
        let dir = Path::new(path);
        if !dir.exists() {
            return Err(format!("Path not found: {}", path));
        }
        let packages = discover_gene_packages(path);
        if packages.is_empty() && !dir.join("gene.toml").exists() {
            return Err(format!("No gene packages at: {}", path));
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
            if c.contains(
                "

// ── Skill System (P5) ──

/// A skill bundles multiple gene references with config.
#[derive(Debug, Clone)]
pub struct Skill {
    pub manifest: SkillManifest,
}

/// skill.toml manifest — references genes to install together.
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
#[cfg(test)]",
            ) || c.contains("#[test]")
            {
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
        for source in &self.sources {
            for pkg in discover_gene_packages(&source.path) {
                for installed in self.council().genes.all() {
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
        source: source.to_string(),
        capabilities: pkg.manifest.capabilities,
        slash_commands: pkg
            .manifest
            .slash_commands
            .iter()
            .map(|s| s.command.clone())
            .collect(),
    }
}

// ── Skill System (P5) ──

/// A skill bundles multiple gene references with config.
#[derive(Debug, Clone)]
pub struct Skill {
    pub manifest: SkillManifest,
}

/// skill.toml manifest — references genes to install together.
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
        let mut sc = ShadowCouncil::new();
        let mut k = Kuber::new(&mut sc);
        k.add_source("a", "/tmp/x");
        k.add_source("b", "/tmp/y");
        assert_eq!(k.list_sources().len(), 2);
        k.remove_source("a");
        assert_eq!(k.list_sources().len(), 1);
    }

    #[test]
    fn empty_stats() {
        let mut sc = ShadowCouncil::new();
        let k = Kuber::new(&mut sc);
        assert_eq!(k.installed_count(), 0);
        assert_eq!(k.available_count(), 0);
    }

    #[test]
    fn scoring_works() {
        let mut sc = ShadowCouncil::new();
        let k = Kuber::new(&mut sc);
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
        let mut sc = ShadowCouncil::new();
        let k = Kuber::new(&mut sc);
        assert!(k.score("/tmp/nope-ktest-99999").is_err());
    }

    #[test]
    fn remote_source_detect() {
        let mut sc = ShadowCouncil::new();
        let mut k = Kuber::new(&mut sc);
        k.add_source("r", "https://github.com/u/r.git");
        k.add_source("l", "/home/u/genes");
        assert_eq!(k.list_sources()[0].kind, SourceKind::Remote);
        assert_eq!(k.list_sources()[1].kind, SourceKind::Local);
    }

    #[test]
    fn updates_empty_when_nothing_installed() {
        let mut sc = ShadowCouncil::new();
        let k = Kuber::new(&mut sc);
        let updates = k.check_updates();
        assert!(updates.is_empty());
    }
    // ── Skill tests ──

    #[test]
    fn skill_scaffold_creates_file() {
        let d = tmp();
        let path = crate::skill::scaffold("test-skill", d.to_str().unwrap()).unwrap();
        assert!(std::path::Path::new(&path).join("skill.toml").exists());
        std::fs::remove_dir_all(d).unwrap();
    }

    #[test]
    fn skill_discover_finds_nothing_in_empty_dir() {
        let d = tmp();
        std::fs::create_dir_all(&d).unwrap();
        let skills = crate::skill::discover(d.to_str().unwrap());
        assert!(skills.is_empty());
        std::fs::remove_dir_all(d).unwrap();
    }
}
