//! Configuration profiles — named execution configurations.
//!
//! Profiles are TOML files in `~/.pandora/profiles/` or
//! `PANDORA_PROFILES_DIR`. Each profile bundles provider, strategy,
//! sandbox, goal, and evaluator settings into a single named file.

use crate::config::config_dir;
use crate::execution_plan::{ControlStrategy, EvaluatorKind, ExecutionPlan, SandboxLevel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Domain-level role metadata for a profile.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct DomainProfile {
    /// The domain role presented by this profile, such as "design" or "coding".
    pub role: Option<String>,
}

/// A provider connection and model selected for one domain role.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ModelBinding {
    pub connection: String,
    pub model: String,
}
/// A named execution profile loaded from TOML.
///
/// All fields are optional — profiles are partial configurations that
/// override defaults rather than replacing them entirely.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Profile {
    /// Provider to use, e.g. "ollama", "openai".
    pub provider: Option<String>,
    /// Control strategy, e.g. "closed", "single".
    pub strategy: Option<String>,
    /// Sandbox level (0 = none, 1 = restricted, 2 = isolated).
    pub sandbox: Option<u8>,
    /// Goal string passed to the evaluator.
    pub goal: Option<String>,
    /// Evaluator to use, e.g. "rust-tests".
    pub evaluator: Option<String>,
    /// Whether to require manual approval.
    pub approval: Option<bool>,
    /// Maximum execution attempts.
    pub max_attempts: Option<u32>,
    /// Optional domain role metadata.
    pub domain: Option<DomainProfile>,
    /// Named provider/model bindings for domain roles.
    #[serde(default)]
    pub models: HashMap<String, ModelBinding>,
}

impl Profile {
    pub fn apply_to(&self, plan: &mut ExecutionPlan) {
        if let Some(provider) = &self.provider {
            plan.provider_policy = provider.clone();
        }
        if let Some(strategy) = self.strategy.as_deref() {
            plan.control_strategy = match strategy {
                "closed" => ControlStrategy::Closed,
                "open" => ControlStrategy::Open,
                "human" => ControlStrategy::Human,
                "autonomous" => ControlStrategy::Autonomous,
                _ => ControlStrategy::SingleShot,
            };
        }
        if let Some(evaluator) = self.evaluator.as_deref() {
            plan.evaluator = match evaluator {
                "rust-tests" => EvaluatorKind::RustTests,
                "python-tests" => EvaluatorKind::PythonTests,
                value => EvaluatorKind::Custom(value.to_string()),
            };
        }
        if let Some(approval) = self.approval {
            plan.approval_required = approval;
        }
        if let Some(max_attempts) = self.max_attempts {
            plan.budget.max_retries = max_attempts.saturating_sub(1);
        }
        if let Some(sandbox) = self.sandbox {
            plan.budget.sandbox_level = match sandbox {
                0 => SandboxLevel::None,
                1 => SandboxLevel::Restricted,
                _ => SandboxLevel::Isolated,
            };
        }
    }

    /// Return the named model binding for a role, if configured.
    pub fn model_binding(&self, role: &str) -> Option<&ModelBinding> {
        self.models.get(role)
    }

    /// Validate role bindings against the configured connection registry.
    pub fn validate_model_bindings(
        &self,
        connections: &crate::connection_manager::ConnectionRegistry,
    ) -> Result<(), String> {
        for (role, binding) in &self.models {
            if role.trim().is_empty() {
                return Err("model binding role cannot be empty".into());
            }
            if binding.connection.trim().is_empty() {
                return Err(format!("model binding '{role}' has no connection"));
            }
            if binding.model.trim().is_empty() {
                return Err(format!("model binding '{role}' has no model"));
            }
            if connections.find(&binding.connection).is_none() {
                return Err(format!(
                    "model binding '{role}' references unknown connection '{}'",
                    binding.connection
                ));
            }
        }
        Ok(())
    }
}

/// Path to the profiles directory.
///
/// Checks `PANDORA_PROFILES_DIR` first, then falls back to
/// `~/.pandora/profiles/`.
pub fn profiles_dir() -> PathBuf {
    std::env::var("PANDORA_PROFILES_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| config_dir().join("profiles"))
}

fn validate_profile_name(name: &str) -> Result<(), crate::PandoraError> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.contains('/')
        || name.contains('\\')
        || name.contains("..")
        || name.chars().any(|character| character.is_control())
    {
        return Err(crate::PandoraError::Internal(format!(
            "Invalid profile name: {name}"
        )));
    }
    Ok(())
}

/// Load a profile by name.
///
/// Reads `<profiles_dir>/<name>.toml` and deserializes it.
/// Returns a descriptive error if the name, file, or profile is invalid.
pub fn load_profile(name: &str) -> Result<Profile, crate::PandoraError> {
    validate_profile_name(name)?;
    let path = profiles_dir().join(format!("{name}.toml"));
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Profile '{name}' not found ({}): {e}", path.display()))?;
    toml::from_str(&content).map_err(|e| {
        crate::PandoraError::Internal(format!("Failed to parse profile '{name}': {e}"))
    })
}

/// List all available profiles (files with `.toml` extension).
pub fn list_profiles() -> Result<Vec<String>, crate::PandoraError> {
    let dir = profiles_dir();
    let mut profiles = Vec::new();
    if !dir.exists() {
        return Ok(profiles);
    }
    for entry in std::fs::read_dir(&dir).map_err(|e| {
        crate::PandoraError::Internal(format!("Cannot read profiles dir {}: {}", dir.display(), e))
    })? {
        let entry = entry.map_err(|e| crate::PandoraError::Internal(e.to_string()))?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                profiles.push(name.to_string());
            }
        }
    }
    profiles.sort();
    Ok(profiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_model_roles_are_deserialized() {
        let profile: Profile = toml::from_str(
            r#"
            [domain]
            role = "design"

            [models.planner]
            connection = "controller"
            model = "controller-model"

            [models.execution]
            connection = "designer"
            model = "design-model"
            "#,
        )
        .unwrap();

        assert_eq!(
            profile.domain.as_ref().unwrap().role.as_deref(),
            Some("design")
        );
        assert_eq!(profile.models["planner"].connection, "controller");
        assert_eq!(profile.models["execution"].model, "design-model");
        assert_eq!(profile.model_binding("review"), None);
        assert_eq!(
            profile.model_binding("planner").unwrap().model,
            "controller-model"
        );

        let connections = crate::connection_manager::ConnectionRegistry {
            connections: vec![
                crate::connection_manager::Connection::new(
                    "controller",
                    crate::connection_manager::ConnectionKind::OpenAI,
                    "https://example.com/controller",
                ),
                crate::connection_manager::Connection::new(
                    "designer",
                    crate::connection_manager::ConnectionKind::OpenAI,
                    "https://example.com/designer",
                ),
            ],
            default_connection: None,
        };
        assert!(profile.validate_model_bindings(&connections).is_ok());
    }

    #[test]
    fn profile_names_cannot_escape_directory() {
        let _guard = crate::test_support::process_env_lock();
        for name in ["../outside", r"..\\outside", "nested/name", "..hidden"] {
            assert!(validate_profile_name(name).is_err());
        }
    }

    #[test]
    fn missing_profile_directory_is_empty() {
        let directory =
            std::env::temp_dir().join(format!("pandora-profiles-{}", rand::random::<u64>()));
        std::env::set_var("PANDORA_PROFILES_DIR", &directory);
        assert!(list_profiles().unwrap().is_empty());
        std::env::remove_var("PANDORA_PROFILES_DIR");
    }
}
