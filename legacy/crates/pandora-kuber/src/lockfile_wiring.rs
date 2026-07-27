//! Lockfile Wiring — save/load pandora.lock during install/upgrade.
//!
//! Lockfiles ensure reproducible installs across machines.
//! On install, the resolver produces a lockfile which is saved to disk.
//! On subsequent installs, the lockfile is read first to check for changes.

use pandora_types::lockfile::Lockfile;
use std::path::PathBuf;

/// Get the lockfile path for a project or global.
pub fn lockfile_path(project_dir: Option<&str>) -> PathBuf {
    if let Some(dir) = project_dir {
        PathBuf::from(dir).join("pandora.lock")
    } else {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        PathBuf::from(home).join(".pandora").join("pandora.lock")
    }
}

/// Load existing lockfile. Returns empty lockfile if not found.
pub fn load_lockfile(project_dir: Option<&str>) -> Lockfile {
    let path = lockfile_path(project_dir);
    if !path.exists() {
        return Lockfile::new();
    }
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Lockfile::new(),
    };
    toml::from_str(&content).unwrap_or_default()
}

/// Save lockfile to disk.
pub fn save_lockfile(
    lock: &Lockfile,
    project_dir: Option<&str>,
) -> Result<(), pandora_types::PandoraError> {
    let path = lockfile_path(project_dir);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            pandora_types::PandoraError::Internal(format!("Cannot create dir: {e}"))
        })?;
    }
    let content = toml::to_string_pretty(lock)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot serialize: {e}")))?;
    std::fs::write(&path, content)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot write: {e}")))
}

/// Check if a package has changed since last lockfile.
pub fn has_changed(lock: &Lockfile, id: &str, version: &str) -> bool {
    match lock.get(id) {
        Some(entry) => entry.version != version,
        None => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lockfile_if_missing() {
        let lf = load_lockfile(Some("/tmp/nonexistent-ktest-99999"));
        assert!(lf.is_empty());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join(format!(
            "lock-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();

        let mut lf = Lockfile::new();
        lf.add("p/a", "1.0.0", "sha256:abc", "palace");
        lf.add("p/b", "2.0.0", "sha256:def", "palace");

        let dir_str = dir.to_str().unwrap();
        save_lockfile(&lf, Some(dir_str)).unwrap();

        let loaded = load_lockfile(Some(dir_str));
        assert_eq!(loaded.get("p/a").unwrap().version, "1.0.0");
        assert_eq!(loaded.get("p/b").unwrap().version, "2.0.0");

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn has_changed_detects_update() {
        let lf = Lockfile::new();
        assert!(has_changed(&lf, "p/a", "1.0.0")); // not in lock

        let mut lf = Lockfile::new();
        lf.add("p/a", "1.0.0", "sha256:abc", "palace");
        assert!(!has_changed(&lf, "p/a", "1.0.0")); // same version
        assert!(has_changed(&lf, "p/a", "2.0.0")); // different version
    }
}
