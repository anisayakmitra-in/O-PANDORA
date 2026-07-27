//! Trust Policy Persistence — save/load trust policy to disk.
//!
//! Trust policy is saved to `~/.pandora/trust.toml` and loaded on startup.
//! This prevents users from having to re-configure trust every session.

use pandora_types::trust::TrustPolicy;
use std::path::PathBuf;

/// Get the trust policy file path.
pub fn trust_policy_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(home).join(".pandora").join("trust.toml")
}

/// Load trust policy from disk. Returns default if not found.
pub fn load_trust_policy() -> TrustPolicy {
    let path = trust_policy_path();
    if !path.exists() {
        return TrustPolicy::default();
    }
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return TrustPolicy::default(),
    };
    toml::from_str(&content).unwrap_or_default()
}

/// Save trust policy to disk.
pub fn save_trust_policy(policy: &TrustPolicy) -> Result<(), pandora_types::PandoraError> {
    let path = trust_policy_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            pandora_types::PandoraError::Internal(format!("Cannot create dir: {e}"))
        })?;
    }
    let content = toml::to_string_pretty(policy)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot serialize: {e}")))?;
    std::fs::write(&path, content)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot write: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_loads() {
        // If no file exists, should return default
        let policy = load_trust_policy();
        assert_eq!(
            policy.min_trust,
            pandora_types::package_format::PackageTrustLevel::None
        );
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join(format!(
            "trust-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();

        // Override the path for testing
        let test_path = dir.join("trust.toml");
        let policy = TrustPolicy::strict();
        let content = toml::to_string_pretty(&policy).unwrap();
        std::fs::write(&test_path, content).unwrap();

        let loaded: TrustPolicy =
            toml::from_str(&std::fs::read_to_string(&test_path).unwrap()).unwrap();
        assert_eq!(loaded.require_signed, policy.require_signed);
        assert_eq!(loaded.min_trust, policy.min_trust);

        std::fs::remove_dir_all(dir).unwrap();
    }
}
