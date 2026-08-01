//! Package loaders — one loader per package kind.
//!
//! Each loader knows how to validate and install a specific kind of
//! package (Gene, Harness, Skill, Provider) into the Shadow Council.
//!

use pandora_shadow_council::ShadowCouncil;
use pandora_types::PandoraError;
use std::path::Path;

/// Trait for package loaders.
pub trait PackageLoader: Send + Sync {
    fn kind(&self) -> crate::PackageKind;
    fn can_load(&self, path: &Path) -> bool;
    fn load(&self, council: &mut ShadowCouncil, path: &Path) -> Result<(), PandoraError>;
    fn unload(&self, council: &mut ShadowCouncil, id: &str) -> Result<(), PandoraError>;
}

// ── GeneLoader ──

pub struct GeneLoader;

impl PackageLoader for GeneLoader {
    fn kind(&self) -> crate::PackageKind {
        crate::PackageKind::Gene
    }

    fn can_load(&self, path: &Path) -> bool {
        path.join("gene.toml").exists()
            || (path.is_file() && path.extension().is_some_and(|e| e == "toml"))
    }

    fn load(&self, council: &mut ShadowCouncil, path: &Path) -> Result<(), PandoraError> {
        let source_dir = if path.is_dir() {
            path.to_path_buf()
        } else {
            path.parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| Path::new(".").to_path_buf())
        };
        let source_str = source_dir.to_string_lossy().to_string();
        council.load_gene_packages(&source_str).map(|_| ())
    }

    fn unload(&self, council: &mut ShadowCouncil, id: &str) -> Result<(), PandoraError> {
        council.uninstall_gene(id)
    }
}

// ── HarnessLoader ──

pub struct HarnessLoader {
    pub harness_kind: pandora_types::harness::HarnessKind,
}

impl HarnessLoader {
    pub fn new(kind: pandora_types::harness::HarnessKind) -> Self {
        Self { harness_kind: kind }
    }
    fn package_kind(&self) -> crate::PackageKind {
        match self.harness_kind {
            pandora_types::harness::HarnessKind::Domain => crate::PackageKind::DomainHarness,
            pandora_types::harness::HarnessKind::Meta => crate::PackageKind::MetaHarness,
            pandora_types::harness::HarnessKind::Source => crate::PackageKind::SourceHarness,
            _ => crate::PackageKind::DomainHarness,
        }
    }
}

impl PackageLoader for HarnessLoader {
    fn kind(&self) -> crate::PackageKind {
        self.package_kind()
    }

    fn can_load(&self, path: &Path) -> bool {
        let has = path.join("harness.toml").exists()
            || (path.is_file() && path.extension().is_some_and(|e| e == "toml"));
        if !has {
            return false;
        }
        let mp = if path.is_dir() {
            path.join("harness.toml")
        } else {
            path.to_path_buf()
        };
        if let Ok(c) = std::fs::read_to_string(&mp) {
            let expected = self.harness_kind.as_str();
            c.contains(&format!("kind = \"{}\"", expected))
                || c.contains(&format!("kind = '{expected}'"))
        } else {
            false
        }
    }

    fn load(&self, council: &mut ShadowCouncil, path: &Path) -> Result<(), PandoraError> {
        let mp = if path.is_dir() {
            path.join("harness.toml")
        } else {
            path.to_path_buf()
        };
        let content = std::fs::read_to_string(&mp)
            .map_err(|e| PandoraError::io(format!("Cannot read harness.toml: {e}")))?;
        let manifest: toml::Value = toml::from_str(&content)
            .map_err(|e| PandoraError::validation(format!("Invalid harness.toml: {e}")))?;
        let id = manifest
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| PandoraError::validation(String::from("harness.toml missing 'id'")))?;
        crate::validation::validate_package_id(id)
            .map_err(|error| PandoraError::validation(error.to_string()))?;

        let staging = pandora_types::gene_package::packages_dir()
            .join("harnesses")
            .join(id);
        std::fs::create_dir_all(&staging)
            .map_err(|e| PandoraError::io(format!("Cannot create staging dir: {e}")))?;
        if path.is_dir() {
            copy_dir_contents(path, &staging)?;
        }
        council
            .load_gene_packages(&staging.to_string_lossy())
            .map(|_| ())
    }

    fn unload(&self, council: &mut ShadowCouncil, id: &str) -> Result<(), PandoraError> {
        council.uninstall(id)
    }
}

// ── SkillLoader ──

pub struct SkillLoader;

impl PackageLoader for SkillLoader {
    fn kind(&self) -> crate::PackageKind {
        crate::PackageKind::Skill
    }

    fn can_load(&self, path: &Path) -> bool {
        path.join("SKILL.md").exists()
    }

    fn load(&self, _council: &mut ShadowCouncil, path: &Path) -> Result<(), PandoraError> {
        let skill_file = path.join("SKILL.md");
        if !skill_file.exists() {
            return Err(PandoraError::validation(String::from("No SKILL.md found")));
        }
        let skill_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        let skills_dir = pandora_types::gene_package::packages_dir()
            .parent()
            .map(|p| p.join("skills"))
            .unwrap_or_else(|| Path::new(".pandora/skills").to_path_buf());
        std::fs::create_dir_all(&skills_dir)
            .map_err(|e| PandoraError::io(format!("Cannot create skills dir: {e}")))?;
        let dest = skills_dir.join(skill_name);
        std::fs::create_dir_all(&dest)
            .map_err(|e| PandoraError::io(format!("Cannot create skill dir: {e}")))?;
        if path.is_dir() {
            copy_dir_contents(path, &dest)?;
        } else {
            std::fs::copy(path, dest.join("SKILL.md"))
                .map_err(|e| PandoraError::io(format!("Cannot copy skill: {e}")))?;
        }
        Ok(())
    }

    fn unload(&self, _council: &mut ShadowCouncil, id: &str) -> Result<(), PandoraError> {
        let skills_dir = pandora_types::gene_package::packages_dir()
            .parent()
            .map(|p| p.join("skills"))
            .unwrap_or_else(|| Path::new(".pandora/skills").to_path_buf());
        let sp = skills_dir.join(id);
        if sp.exists() {
            std::fs::remove_dir_all(&sp)
                .map_err(|e| PandoraError::io(format!("Cannot remove skill: {e}")))?;
        }
        Ok(())
    }
}

// ── ProviderLoader ──

pub struct ProviderLoader;

impl PackageLoader for ProviderLoader {
    fn kind(&self) -> crate::PackageKind {
        crate::PackageKind::Provider
    }
    fn can_load(&self, path: &Path) -> bool {
        path.join("provider.toml").exists()
    }
    fn load(&self, _council: &mut ShadowCouncil, _path: &Path) -> Result<(), PandoraError> {
        Err(PandoraError::governance(String::from(
            "Provider packages are installed via pandora connection add.",
        )))
    }
    fn unload(&self, _council: &mut ShadowCouncil, _id: &str) -> Result<(), PandoraError> {
        Err(PandoraError::governance(String::from(
            "Provider packages are managed via pandora connection remove.",
        )))
    }
}

// ── LoaderRegistry ──

pub struct LoaderRegistry {
    loaders: Vec<Box<dyn PackageLoader>>,
}

impl LoaderRegistry {
    pub fn new() -> Self {
        let mut r = Self {
            loaders: Vec::new(),
        };
        r.register(Box::new(HarnessLoader::new(
            pandora_types::harness::HarnessKind::Source,
        )));
        r.register(Box::new(HarnessLoader::new(
            pandora_types::harness::HarnessKind::Meta,
        )));
        r.register(Box::new(HarnessLoader::new(
            pandora_types::harness::HarnessKind::Domain,
        )));
        r.register(Box::new(SkillLoader));
        r.register(Box::new(ProviderLoader));
        r.register(Box::new(GeneLoader));
        r
    }

    pub fn register(&mut self, loader: Box<dyn PackageLoader>) {
        self.loaders.push(loader);
    }

    pub fn detect(&self, path: &Path) -> Option<&dyn PackageLoader> {
        self.loaders
            .iter()
            .find(|l| l.can_load(path))
            .map(|l| l.as_ref())
    }

    pub fn install(&self, council: &mut ShadowCouncil, path: &Path) -> Result<(), PandoraError> {
        let loader = self.detect(path).ok_or_else(|| {
            PandoraError::validation(format!(
                "Cannot detect package kind for {}. Expected gene.toml, harness.toml, SKILL.md, or provider.toml.",
                path.display()
            ))
        })?;
        loader.load(council, path)
    }

    pub fn uninstall(
        &self,
        council: &mut ShadowCouncil,
        id: &str,
        hint: Option<crate::PackageKind>,
    ) -> Result<(), PandoraError> {
        if let Some(k) = hint {
            for l in &self.loaders {
                if l.kind() == k {
                    return l.unload(council, id);
                }
            }
        }
        let mut last = None;
        for l in &self.loaders {
            match l.unload(council, id) {
                Ok(()) => return Ok(()),
                Err(e) => last = Some(e),
            }
        }
        Err(last.unwrap_or_else(|| PandoraError::not_found(format!("Cannot uninstall {id}"))))
    }
}

impl Default for LoaderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ── Utility ──

fn copy_dir_contents(src: &Path, dst: &Path) -> Result<(), PandoraError> {
    for entry in std::fs::read_dir(src)
        .map_err(|e| PandoraError::io(format!("Cannot read dir {}: {e}", src.display())))?
    {
        let entry = entry.map_err(|e| PandoraError::io(format!("Dir entry error: {e}")))?;
        let sp = entry.path();
        let dp = dst.join(entry.file_name());
        if sp.is_dir() {
            std::fs::create_dir_all(&dp)
                .map_err(|e| PandoraError::io(format!("Cannot create dir: {e}")))?;
            copy_dir_contents(&sp, &dp)?;
        } else {
            std::fs::copy(&sp, &dp)
                .map_err(|e| PandoraError::io(format!("Cannot copy {}: {e}", sp.display())))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_types::harness::HarnessKind;
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct EnvGuard {
        name: &'static str,
        previous: Option<OsString>,
    }

    impl EnvGuard {
        fn set(name: &'static str, value: &Path) -> Self {
            let previous = std::env::var_os(name);
            std::env::set_var(name, value);
            Self { name, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.previous {
                std::env::set_var(self.name, value);
            } else {
                std::env::remove_var(self.name);
            }
        }
    }

    fn test_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "pandora-ko-palace-{label}-{}-{nonce}",
            std::process::id()
        ))
    }

    fn write_harness(root: &Path, id: &str) -> PathBuf {
        let package = root.join("package");
        std::fs::create_dir_all(&package).expect("create package fixture");
        std::fs::write(
            package.join("harness.toml"),
            format!("id = {id:?}\nkind = \"domain\"\n"),
        )
        .expect("write harness manifest");
        package
    }

    #[test]
    fn harness_loader_rejects_traversal_id_before_staging() {
        let _lock = crate::TEST_ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let root = test_root("traversal");
        let pandora_home = root.join("home");
        let package = write_harness(&root, "../../outside");
        let _pandora_home = EnvGuard::set("PANDORA_HOME", &pandora_home);
        let mut council = ShadowCouncil::new();

        let error = HarnessLoader::new(HarnessKind::Domain)
            .load(&mut council, &package)
            .expect_err("traversal identifier must be rejected");

        assert!(matches!(error, PandoraError::Validation(_)));
        assert!(!pandora_home.join("outside").exists());
        std::fs::remove_dir_all(root).expect("remove package fixture");
    }

    #[test]
    fn harness_loader_stages_valid_identifier() {
        let _lock = crate::TEST_ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let root = test_root("valid");
        let pandora_home = root.join("home");
        let package = write_harness(&root, "safe-harness");
        let _pandora_home = EnvGuard::set("PANDORA_HOME", &pandora_home);
        let mut council = ShadowCouncil::new();

        HarnessLoader::new(HarnessKind::Domain)
            .load(&mut council, &package)
            .expect("valid harness should load");

        assert!(pandora_home
            .join("packages/harnesses/safe-harness/harness.toml")
            .is_file());
        std::fs::remove_dir_all(root).expect("remove package fixture");
    }
}
