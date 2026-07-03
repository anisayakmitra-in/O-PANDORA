//! Policy Engine — post-execution pipelines.
//!
//! Defines what happens automatically after execution completes.
//! Example: After Coding -> cargo fmt -> cargo clippy -> cargo test -> benchmark -> update dashboard -> summarize.
//!
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
            PolicyAction::Format => "format",
            PolicyAction::Lint => "lint",
            PolicyAction::Test => "test",
            PolicyAction::Benchmark => "benchmark",
            PolicyAction::UpdateDashboard => "update_dashboard",
            PolicyAction::Summarize => "summarize",
            PolicyAction::Notify => "notify",
            PolicyAction::RunCommand(c) => c,
            PolicyAction::Custom(c) => c,
        }
    }
}

/// A policy — a pipeline of actions triggered by an event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub trigger: String, // "after_coding", "after_research", "before_commit"
    pub actions: Vec<PolicyAction>,
    pub enabled: bool,
    pub domain: String, // applies to this domain, or "*" for all
    pub priority: u32,
}

impl Policy {
    pub fn new(
        name: impl Into<String>,
        trigger: impl Into<String>,
        domain: impl Into<String>,
    ) -> Self {
        Self {
            id: format!("policy-{:x}", 42u64),
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
    pub fn new() -> Self {
        Self {
            policies: Vec::new(),
        }
    }

    pub fn register(&mut self, policy: Policy) {
        self.policies.push(policy);
    }

    /// Get policies that apply to a trigger and domain.
    pub fn resolve(&self, trigger: &str, domain: &str) -> Vec<&Policy> {
        let mut result: Vec<&Policy> = self
            .policies
            .iter()
            .filter(|p| {
                p.enabled && p.trigger == trigger && (p.domain == domain || p.domain == "*")
            })
            .collect();
        result.sort_by_key(|a| a.priority);
        result
    }

    /// Execute a policy pipeline — returns the list of actions to run.
    pub fn execute(&self, trigger: &str, domain: &str) -> Vec<&PolicyAction> {
        let policies = self.resolve(trigger, domain);
        policies.iter().flat_map(|p| p.actions.iter()).collect()
    }

    pub fn list(&self) -> &[Policy] {
        &self.policies
    }
    pub fn policy_count(&self) -> usize {
        self.policies.len()
    }

    /// Build standard policies for common domains.
    pub fn build_standard(&mut self) {
        let coding = Policy::new("Coding Workflow", "after_coding", "coding")
            .action(PolicyAction::Format)
            .action(PolicyAction::Lint)
            .action(PolicyAction::Test)
            .action(PolicyAction::Benchmark)
            .action(PolicyAction::Summarize);
        self.register(coding);

        let research = Policy::new("Research Workflow", "after_research", "research")
            .action(PolicyAction::Summarize)
            .action(PolicyAction::UpdateDashboard)
            .action(PolicyAction::Notify);
        self.register(research);
    }
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_resolve() {
        let mut engine = PolicyEngine::new();
        engine.build_standard();
        let policies = engine.resolve("after_coding", "coding");
        assert!(!policies.is_empty());
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
        let coding_actions = engine.execute("after_coding", "coding");
        let research_actions = engine.execute("after_research", "research");
        assert!(!coding_actions.is_empty());
        assert!(!research_actions.is_empty());
    }

    #[test]
    fn disabled_policies_skipped() {
        let mut engine = PolicyEngine::new();
        let mut p = Policy::new("Disabled Test", "after_coding", "*");
        p.enabled = false;
        engine.register(p);
        let actions = engine.execute("after_coding", "coding");
        // Standard coding workflow should still fire
        assert!(actions.iter().any(|a| a.name() == "benchmark"));
    }
}
