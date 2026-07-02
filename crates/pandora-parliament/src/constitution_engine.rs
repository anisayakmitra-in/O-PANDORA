use std::collections::HashMap;

/// The outcome of a policy evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyEvaluation {
    /// The action is permitted.
    Allowed,
    /// The action is denied with a reason.
    Denied(String),
    /// The action requires additional approval.
    PendingApproval(String),
}

/// A constitutional policy. Implementations determine
/// whether an action is allowed, denied, or requires review.
pub trait Policy: Send + Sync {
    fn name(&self) -> &str;
    fn evaluate(&self, action: &str, context: &HashMap<String, String>) -> PolicyEvaluation;
}

/// A simple allow-all policy. Useful as a default or
/// during development. Never use in production without
/// additional governance layers.
pub struct AllowAllPolicy;

impl Policy for AllowAllPolicy {
    fn name(&self) -> &str {
        "allow-all"
    }
    fn evaluate(&self, _action: &str, _context: &HashMap<String, String>) -> PolicyEvaluation {
        PolicyEvaluation::Allowed
    }
}

/// A deny-all policy. Useful for lockdown or maintenance mode.
pub struct DenyAllPolicy;

impl Policy for DenyAllPolicy {
    fn name(&self) -> &str {
        "deny-all"
    }
    fn evaluate(&self, _action: &str, _context: &HashMap<String, String>) -> PolicyEvaluation {
        PolicyEvaluation::Denied("deny-all policy active".to_string())
    }
}

/// The constitutional state. This captures the current
/// state of the Parliament's constitution: which policies
/// are active, what phase the system is in, etc.
#[derive(Debug, Clone)]
pub struct ConstitutionalState {
    /// The current constitutional phase.
    pub phase: String,
    /// Arbitrary key-value state for constitutional extensions.
    pub state: HashMap<String, String>,
}

impl ConstitutionalState {
    pub fn new() -> Self {
        Self {
            phase: "bootstrap".to_string(),
            state: HashMap::new(),
        }
    }

    pub fn set_phase(&mut self, phase: impl Into<String>) {
        self.phase = phase.into();
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.state.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.state.get(key).map(|s| s.as_str())
    }
}

impl Default for ConstitutionalState {
    fn default() -> Self {
        Self::new()
    }
}

/// The Constitution Engine.
///
/// Owns the constitutional state and evaluates all actions
/// against active policies. Every decision in Pandora flows
/// through this engine.
///
/// In the full architecture, this integrates with:
/// - PANOPTES (governance scoring)
/// - Policy Registry (dynamic policy management)
/// - Decision Log (audit trail)
/// - Shadow Council (emergency overrides)
pub struct ConstitutionEngine {
    state: ConstitutionalState,
    policies: Vec<Box<dyn Policy + Send + Sync>>,
}

impl ConstitutionEngine {
    pub fn new() -> Self {
        Self {
            state: ConstitutionalState::new(),
            policies: Vec::new(),
        }
    }

    /// Add a policy to the engine. Policies are evaluated
    /// in order. If any policy denies an action, the action
    /// is denied.
    pub fn add_policy(&mut self, policy: Box<dyn Policy + Send + Sync>) {
        self.policies.push(policy);
    }

    /// Evaluate an action against all active policies.
    /// Returns the most severe result (Denied > Pending > Allowed).
    pub fn evaluate(&self, action: &str, context: &HashMap<String, String>) -> PolicyEvaluation {
        for policy in &self.policies {
            let result = policy.evaluate(action, context);
            match result {
                PolicyEvaluation::Denied(_) => return result,
                PolicyEvaluation::PendingApproval(_) => return result,
                PolicyEvaluation::Allowed => continue,
            }
        }
        PolicyEvaluation::Allowed
    }

    /// Access the constitutional state.
    pub fn state(&self) -> &ConstitutionalState {
        &self.state
    }

    /// Mutably access the constitutional state.
    pub fn state_mut(&mut self) -> &mut ConstitutionalState {
        &mut self.state
    }

    /// List all registered policy names.
    pub fn policy_names(&self) -> Vec<&str> {
        self.policies.iter().map(|p| p.name()).collect()
    }
}

impl Default for ConstitutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allow_all_policy_by_default() {
        let engine = ConstitutionEngine::new();
        let ctx = HashMap::new();
        assert_eq!(
            engine.evaluate("any.action", &ctx),
            PolicyEvaluation::Allowed
        );
    }

    #[test]
    fn deny_all_policy_blocks() {
        let mut engine = ConstitutionEngine::new();
        engine.add_policy(Box::new(DenyAllPolicy));

        let ctx = HashMap::new();
        let result = engine.evaluate("any.action", &ctx);
        assert!(matches!(result, PolicyEvaluation::Denied(_)));
    }

    #[test]
    fn custom_policy() {
        struct AllowOnlyPolicy;
        impl Policy for AllowOnlyPolicy {
            fn name(&self) -> &str {
                "allow-only-test"
            }
            fn evaluate(&self, action: &str, _ctx: &HashMap<String, String>) -> PolicyEvaluation {
                if action == "allowed.action" {
                    PolicyEvaluation::Allowed
                } else {
                    PolicyEvaluation::Denied("not allowed".to_string())
                }
            }
        }

        let mut engine = ConstitutionEngine::new();
        engine.add_policy(Box::new(AllowOnlyPolicy));

        let ctx = HashMap::new();
        assert_eq!(
            engine.evaluate("allowed.action", &ctx),
            PolicyEvaluation::Allowed
        );
        assert!(matches!(
            engine.evaluate("forbidden.action", &ctx),
            PolicyEvaluation::Denied(_)
        ));
    }

    #[test]
    fn constitutional_state_management() {
        let mut engine = ConstitutionEngine::new();
        engine.state_mut().set_phase("operation");
        engine.state_mut().set("provider_lock", "ollama");

        assert_eq!(engine.state().phase, "operation");
        assert_eq!(engine.state().get("provider_lock"), Some("ollama"));
    }
}
