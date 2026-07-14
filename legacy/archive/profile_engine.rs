//! Profile Engine — runtime execution profiles.
//!
//! Profiles change execution behavior without changing prompts.
//! Each profile bundles: loop depth, verification level, checkpoint frequency,
//! provider preference, telemetry level, cost budget, retry policy.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A named runtime profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProfile {
    pub name: String,
    pub description: String,

    // Execution behavior
    pub loop_depth: u32,
    pub verification_enabled: bool,
    pub checkpoint_every_step: bool,
    pub max_retries: u32,
    pub telemetry_level: u8,
    pub reasoning_depth: u32,

    // Resource preferences
    pub prefer_local: bool,
    pub cost_budget: f64,
    pub max_latency_ms: u64,

    // Learning
    pub record_executions: bool,
    pub update_provider_learning: bool,
    pub update_benchmarks: bool,
}

impl ExecutionProfile {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            loop_depth: 3,
            verification_enabled: true,
            checkpoint_every_step: false,
            max_retries: 3,
            telemetry_level: 2,
            reasoning_depth: 3,
            prefer_local: false,
            cost_budget: 1.0,
            max_latency_ms: 30000,
            record_executions: true,
            update_provider_learning: true,
            update_benchmarks: true,
        }
    }
}

impl Default for ExecutionProfile {
    fn default() -> Self {
        Self::new("default")
    }
}

/// The Profile Engine — manages and applies execution profiles.
pub struct ProfileEngine {
    profiles: HashMap<String, ExecutionProfile>,
    active: String,
}

impl ProfileEngine {
    pub fn new() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert("default".to_string(), ExecutionProfile::new("default"));
        Self {
            profiles,
            active: "default".to_string(),
        }
    }

    pub fn register(&mut self, profile: ExecutionProfile) {
        let name = profile.name.clone();
        self.profiles.insert(name, profile);
    }

    pub fn activate(&mut self, name: &str) -> Result<(), String> {
        if self.profiles.contains_key(name) {
            self.active = name.to_string();
            Ok(())
        } else {
            Err(format!("profile '{}' not found", name))
        }
    }

    pub fn active(&self) -> &ExecutionProfile {
        &self.profiles[&self.active]
    }
    pub fn get(&self, name: &str) -> Option<&ExecutionProfile> {
        self.profiles.get(name)
    }
    pub fn list(&self) -> Vec<&str> {
        self.profiles.keys().map(|s| s.as_str()).collect()
    }

    /// Build standard profiles.
    pub fn build_standard(&mut self) {
        let dev = ExecutionProfile {
            name: "development".into(),
            description: "Fast iteration, minimal verification".into(),
            loop_depth: 2,
            verification_enabled: false,
            checkpoint_every_step: false,
            max_retries: 1,
            telemetry_level: 1,
            reasoning_depth: 2,
            prefer_local: true,
            cost_budget: 0.1,
            max_latency_ms: 10000,
            record_executions: true,
            update_provider_learning: true,
            update_benchmarks: false,
        };
        self.register(dev);

        let research = ExecutionProfile {
            name: "research".into(),
            description: "Deep exploration, high verification".into(),
            loop_depth: 10,
            verification_enabled: true,
            checkpoint_every_step: true,
            max_retries: 5,
            telemetry_level: 3,
            reasoning_depth: 5,
            prefer_local: false,
            cost_budget: 10.0,
            max_latency_ms: 120000,
            record_executions: true,
            update_provider_learning: true,
            update_benchmarks: true,
        };
        self.register(research);

        let yolo = ExecutionProfile {
            name: "yolo".into(),
            description: "Maximum speed, minimal safety".into(),
            loop_depth: 1,
            verification_enabled: false,
            checkpoint_every_step: false,
            max_retries: 0,
            telemetry_level: 0,
            reasoning_depth: 1,
            prefer_local: true,
            cost_budget: 0.01,
            max_latency_ms: 5000,
            record_executions: false,
            update_provider_learning: false,
            update_benchmarks: false,
        };
        self.register(yolo);

        let enterprise = ExecutionProfile {
            name: "enterprise".into(),
            description: "Maximum safety, governance, audit".into(),
            loop_depth: 3,
            verification_enabled: true,
            checkpoint_every_step: true,
            max_retries: 3,
            telemetry_level: 3,
            reasoning_depth: 3,
            prefer_local: false,
            cost_budget: 100.0,
            max_latency_ms: 60000,
            record_executions: true,
            update_provider_learning: false,
            update_benchmarks: false,
        };
        self.register(enterprise);
    }
}

impl Default for ProfileEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activate_profile() {
        let mut engine = ProfileEngine::new();
        engine.build_standard();
        assert!(engine.activate("research").is_ok());
        assert_eq!(engine.active().name, "research");
    }

    #[test]
    fn unknown_profile() {
        let mut engine = ProfileEngine::new();
        assert!(engine.activate("nonexistent").is_err());
    }

    #[test]
    fn yolo_profile_is_fast() {
        let mut engine = ProfileEngine::new();
        engine.build_standard();
        engine.activate("yolo").unwrap();
        let p = engine.active();
        assert_eq!(p.loop_depth, 1);
        assert_eq!(p.telemetry_level, 0);
        assert!(!p.verification_enabled);
    }

    #[test]
    fn enterprise_profile_is_safe() {
        let mut engine = ProfileEngine::new();
        engine.build_standard();
        engine.activate("enterprise").unwrap();
        let p = engine.active();
        assert!(p.verification_enabled);
        assert!(p.checkpoint_every_step);
    }

    #[test]
    fn list_profiles() {
        let mut engine = ProfileEngine::new();
        engine.build_standard();
        let list = engine.list();
        assert!(list.contains(&"development"));
        assert!(list.contains(&"research"));
        assert!(list.contains(&"yolo"));
    }
}
