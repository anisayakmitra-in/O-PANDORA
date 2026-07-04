//! Pandora KUBER — minimal gene distribution system.

use pandora_shadow_council::ShadowCouncil;
use pandora_types::gene_package::discover_gene_packages;

#[derive(Debug, Clone)]
pub struct PackageSource {
    pub name: String,
    pub path: String,
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
        self.sources.push(PackageSource {
            name: name.to_string(),
            path: path.to_string(),
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
            let packages = discover_gene_packages(&source.path);
            for pkg in packages {
                let id = pkg.manifest.id.to_lowercase();
                if id.contains(&q) || pkg.manifest.name.to_lowercase().contains(&q) {
                    results.push(PackageInfo {
                        id: pkg.manifest.id,
                        name: pkg.manifest.name,
                        kind: pkg.manifest.kind,
                        version: pkg.manifest.version,
                        author: pkg.manifest.author,
                        description: pkg.manifest.description.unwrap_or_default(),
                        source: source.name.clone(),
                        capabilities: pkg.manifest.capabilities,
                        slash_commands: pkg
                            .manifest
                            .slash_commands
                            .iter()
                            .map(|s| s.command.clone())
                            .collect(),
                    });
                }
            }
        }
        results
    }

    pub fn list_available(&self) -> Vec<PackageInfo> {
        let mut results = Vec::new();
        for source in &self.sources {
            let packages = discover_gene_packages(&source.path);
            for pkg in packages {
                results.push(PackageInfo {
                    id: pkg.manifest.id,
                    name: pkg.manifest.name,
                    kind: pkg.manifest.kind,
                    version: pkg.manifest.version,
                    author: pkg.manifest.author,
                    description: pkg.manifest.description.unwrap_or_default(),
                    source: source.name.clone(),
                    capabilities: pkg.manifest.capabilities,
                    slash_commands: pkg
                        .manifest
                        .slash_commands
                        .iter()
                        .map(|s| s.command.clone())
                        .collect(),
                });
            }
        }
        results
    }

    pub fn info(&self, id: &str) -> Option<PackageInfo> {
        self.list_available().into_iter().find(|p| p.id == id)
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

    pub fn available_count(&self) -> usize {
        self.sources
            .iter()
            .map(|s| discover_gene_packages(&s.path).len())
            .sum()
    }

    pub fn installed_count(&self) -> usize {
        self.council().genes.total_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_shadow_council::ShadowCouncil;

    #[test]
    fn kuber_source_management() {
        let mut sc = ShadowCouncil::new();
        let mut kuber = Kuber::new(&mut sc);
        kuber.add_source("src1", "/tmp/genes1");
        kuber.add_source("src2", "/tmp/genes2");
        assert_eq!(kuber.list_sources().len(), 2);
        kuber.remove_source("src1");
        assert_eq!(kuber.list_sources().len(), 1);
        assert_eq!(kuber.list_sources()[0].name, "src2");
    }

    #[test]
    fn kuber_empty_state() {
        let mut sc = ShadowCouncil::new();
        let kuber = Kuber::new(&mut sc);
        assert_eq!(kuber.installed_count(), 0);
        assert!(kuber.list_installed().is_empty());
        assert_eq!(kuber.available_count(), 0);
    }
}
