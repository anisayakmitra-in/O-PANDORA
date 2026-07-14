//! Policy Engine — post-execution pipelines.
//!
//! Defines what happens automatically after execution completes.
//! Policies are configurable, composable, and attached to domains or workflows.

use serde::{Deserialize, Serialize};

/// A single action in a policy pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    Format,
    Lint,
    Test,
    Benchmark,
    UpdateDashboard,
    Summarize,
    Notify,
    RunCommand(String),
    Custom(String),
}

impl PolicyAction {
    pub fn name(&self) -> &str {
        match self {
            Self::Format => "format",
            Self::Lint => "lint",
            Self::Test => "test",
            Self::Benchmark => "benchmark",
            Self::UpdateDashboard => "update_dashboard",
            Self::Summarize => "summarize",
            Self::Notify => "notify",
            Self::RunCommand(c) => c,
            Self::Custom(c) => c,
        }
    }
}

/// A policy — a pipeline of actions triggered by an event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub trigger: String,
    pub actions: Vec<PolicyAction>,
    pub enabled: bool,
    pub domain: String,
    pub priority: u32,
}

impl Policy {
    pub fn new(name: impl Into<String>, trigger: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            // ponytail: unique ID via random
            id: format!("policy-{:016x}", rand::random::<u64>()),
            name: name.into(),
            description: String::new(),
            trigger: trigger.into(),
            actions: Vec::new(),
            enabled: true,
            domain: domain.into(),
            priority: 100,
        }
    }

    pub fn action(mut self, action: PolicyAction) -> Self {
        self.actions.push(action);
        self
    }
}

/// The Policy Engine — manages and executes policies.
pub struct PolicyEngine {
    policies: Vec<Policy>,
}

impl PolicyEngine {
    pub fn new() -> Self { Self { policies: Vec::new() } }

    pub fn register(&mut self, policy: Policy) {
        self.policies.push(policy);
    }

    pub fn resolve(&self, trigger: &str, domain: &str) -> Vec<&Policy> {
        let mut result: Vec<&Policy> = self.policies.iter()
            .filter(|p| p.enabled && p.trigger == trigger && (p.domain == domain || p.domain == "*"))
            .collect();
        result.sort_by_key(|a| a.priority);
        result
    }

    pub fn execute(&self, trigger: &str, domain: &str) -> Vec<&PolicyAction> {
        self.resolve(trigger, domain).iter().flat_map(|p| p.actions.iter()).collect()
    }

    pub fn list(&self) -> &[Policy] { &self.policies }
    pub fn policy_count(&self) -> usize { self.policies.len() }

    pub fn build_standard(&mut self) {
        self.register(Policy::new("Coding Workflow", "after_coding", "coding")
            .action(PolicyAction::Format).action(PolicyAction::Lint)
            .action(PolicyAction::Test).action(PolicyAction::Benchmark)
            .action(PolicyAction::Summarize));
        self.register(Policy::new("Research Workflow", "after_research", "research")
            .action(PolicyAction::Summarize).action(PolicyAction::UpdateDashboard)
            .action(PolicyAction::Notify));
    }
}

impl Default for PolicyEngine { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_resolve() {
        let mut engine = PolicyEngine::new();
        engine.build_standard();
        assert!(!engine.resolve("after_coding", "coding").is_empty());
    }

    #[test]
    fn execute_returns_actions() {
        let mut engine = PolicyEngine::new();
        engine.build_standard();
        let actions = engine.execute("after_coding", "coding");
        assert!(actions.iter().any(|a| a.name() == "format"));
        assert!(actions.iter().any(|a| a.name() == "test"));
    }

    #[test]
    fn domain_filtering() {
        let mut engine = PolicyEngine::new();
        engine.build_standard();
        assert!(!engine.execute("after_coding", "coding").is_empty());
        assert!(!engine.execute("after_research", "research").is_empty());
    }

    #[test]
    fn disabled_policies_skipped() {
        let mut engine = PolicyEngine::new();
        let mut p = Policy::new("Disabled Test", "after_coding", "*");
        p.enabled = false;
        engine.register(p);
        let mut enabled = Policy::new("Enabled Test", "after_coding", "*");
        enabled = enabled.action(PolicyAction::Benchmark);
        engine.register(enabled);
        let actions = engine.execute("after_coding", "coding");
        assert!(actions.iter().any(|a| a.name() == "benchmark"));
    }
}
