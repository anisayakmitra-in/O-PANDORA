//! Upgrade & Rollback — safe package upgrades with automatic rollback.
//!
//! Upgrade: compares installed vs available versions, upgrades if newer.
//! Rollback: backs up previous version before upgrade, restores on failure.

use std::path::{Path, PathBuf};

/// Get the backup directory for a package.
fn backup_dir(package_id: &str) -> PathBuf {
    let home = std::env::var_os("PANDORA_HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join(".pandora")
        .join("backups")
        .join(package_id.replace('/', "__"))
}

/// Backup a package before upgrade. Returns the backup path.
pub fn backup_package(
    package_id: &str,
    install_dir: &Path,
) -> Result<PathBuf, pandora_types::PandoraError> {
    if !install_dir.exists() {
        return Err(pandora_types::PandoraError::Internal(format!(
            "Install directory not found: {}",
            install_dir.display()
        )));
    }

    let bak = backup_dir(package_id);
    if bak.exists() {
        std::fs::remove_dir_all(&bak).map_err(|e| {
            pandora_types::PandoraError::Internal(format!("Cannot clean backup: {e}"))
        })?;
    }

    copy_dir_recursive(install_dir, &bak)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Backup failed: {e}")))?;

    Ok(bak)
}

/// Restore a package from backup.
pub fn rollback_package(
    package_id: &str,
    install_dir: &Path,
) -> Result<(), pandora_types::PandoraError> {
    let bak = backup_dir(package_id);
    if !bak.exists() {
        return Err(pandora_types::PandoraError::Internal(format!(
            "No backup found for {package_id}"
        )));
    }

    // Remove current install
    if install_dir.exists() {
        std::fs::remove_dir_all(install_dir).map_err(|e| {
            pandora_types::PandoraError::Internal(format!("Cannot remove current install: {e}"))
        })?;
    }

    // Restore from backup
    copy_dir_recursive(&bak, install_dir)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Restore failed: {e}")))?;

    // Clean up backup
    let _ = std::fs::remove_dir_all(&bak);

    Ok(())
}

/// Clean old backups (keep last N per package).
pub fn clean_backups(package_id: &str, keep: usize) -> Result<(), pandora_types::PandoraError> {
    // For now, just remove the backup dir if it exists and keep=0
    if keep == 0 {
        let bak = backup_dir(package_id);
        if bak.exists() {
            std::fs::remove_dir_all(&bak).map_err(|e| {
                pandora_types::PandoraError::Internal(format!("Cannot remove backup: {e}"))
            })?;
        }
    }
    Ok(())
}

/// Check if a backup exists for a package.
pub fn has_backup(package_id: &str) -> bool {
    backup_dir(package_id).exists()
}

/// Determine upgrade action: returns (current_version, new_version, action).
pub fn plan_upgrade(
    _package_id: &str,
    installed_version: &str,
    available_version: &str,
) -> UpgradeAction {
    use std::cmp::Ordering;

    let cmp = compare_versions_simple(available_version, installed_version);

    match cmp {
        Ordering::Greater => UpgradeAction::Upgrade {
            from: installed_version.to_string(),
            to: available_version.to_string(),
        },
        Ordering::Less => UpgradeAction::Downgrade {
            from: installed_version.to_string(),
            to: available_version.to_string(),
        },
        Ordering::Equal => UpgradeAction::UpToDate {
            version: installed_version.to_string(),
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpgradeAction {
    Upgrade { from: String, to: String },
    Downgrade { from: String, to: String },
    UpToDate { version: String },
}

/// Simple version comparison (non-dependency — standalone for this module).
fn compare_versions_simple(a: &str, b: &str) -> std::cmp::Ordering {
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

/// Recursively copy a directory.
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), pandora_types::PandoraError> {
    std::fs::create_dir_all(dst)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot create dir: {e}")))?;

    for entry in std::fs::read_dir(src)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot read dir: {e}")))?
    {
        let entry = entry.map_err(|e| {
            pandora_types::PandoraError::Internal(format!("Cannot read entry: {e}"))
        })?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path).map_err(|e| {
                pandora_types::PandoraError::Internal(format!("Cannot copy file: {e}"))
            })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_upgrade_newer() {
        let action = plan_upgrade("p/a", "1.0.0", "2.0.0");
        assert_eq!(
            action,
            UpgradeAction::Upgrade {
                from: "1.0.0".into(),
                to: "2.0.0".into()
            }
        );
    }

    #[test]
    fn plan_upgrade_older() {
        let action = plan_upgrade("p/a", "2.0.0", "1.0.0");
        assert_eq!(
            action,
            UpgradeAction::Downgrade {
                from: "2.0.0".into(),
                to: "1.0.0".into()
            }
        );
    }

    #[test]
    fn plan_upgrade_same() {
        let action = plan_upgrade("p/a", "1.0.0", "1.0.0");
        assert_eq!(
            action,
            UpgradeAction::UpToDate {
                version: "1.0.0".into()
            }
        );
    }

    #[test]
    fn backup_and_rollback() {
        let dir = std::env::temp_dir().join(format!(
            "rollback-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let install_dir = dir.join("install");
        std::fs::create_dir_all(&install_dir).unwrap();
        std::fs::write(install_dir.join("file.txt"), "version1").unwrap();

        // Backup in an isolated test data directory.
        let pkg_id = "test-rollback-pkg";
        std::env::set_var("PANDORA_HOME", &dir);
        let bak = backup_package(pkg_id, &install_dir).unwrap();
        assert!(bak.exists());

        // Simulate upgrade
        std::fs::write(install_dir.join("file.txt"), "version2").unwrap();
        assert_eq!(
            std::fs::read_to_string(install_dir.join("file.txt")).unwrap(),
            "version2"
        );

        // Rollback
        rollback_package(pkg_id, &install_dir).unwrap();
        assert_eq!(
            std::fs::read_to_string(install_dir.join("file.txt")).unwrap(),
            "version1"
        );

        // Cleanup
        std::fs::remove_dir_all(dir).unwrap();
        std::env::remove_var("PANDORA_HOME");
    }

    #[test]
    fn no_backup_without_install() {
        assert!(!has_backup("nonexistent-pkg-xyz"));
    }
}
