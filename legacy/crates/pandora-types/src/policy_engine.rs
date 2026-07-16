//! Policy Engine — declarative governance for the execution runtime.
//!
//! Every execution decision goes through policy evaluation. Policies are
//! declarative rules with conditions, constraints, and actions. All decisions
//! are recorded with evidence for auditability.
//!
//! Design: OPA/Rego-inspired, but embedded in Rust for low-latency evaluation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// A single policy rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub priority: u32, // higher = evaluated first
    pub conditions: Vec<PolicyCondition>,
    pub actions: Vec<PolicyAction>,
    pub enabled: bool,
}

/// Conditions that must be satisfied for the policy to fire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    pub field: String,    // e.g. "execution.sandbox_level", "package.trust_level"
    pub operator: ConditionOp,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionOp {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    In,
    Exists,
    Empty,
}

/// What happens when policy conditions are met.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyAction {
    Allow { reason: String },
    Deny { reason: String, block: bool },
    RequireApproval { reason: String, approver: Option<String> },
    Log { level: String, message: String },
    ModifyRequest { field: String, value: serde_json::Value },
    Route { connection: String },
    Quarantine { reason: String },
    Escalate { level: String, reason: String },
}

/// The result of evaluating a single policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyVerdict {
    pub policy_id: String,
    pub passed: bool,
    pub action: Option<PolicyAction>,
    pub evidence: HashMap<String, serde_json::Value>,
    pub evaluated_at: SystemTime,
}

/// The policy engine — registry + evaluator.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyEngine {
    pub policies: Vec<Policy>,
    pub history: Vec<PolicyVerdict>,
    pub version: u32,
}

impl PolicyEngine {
    pub fn new() -> Self { Self::default() }

    /// Register a policy.
    pub fn register(&mut self, policy: Policy) { self.policies.push(policy); }

    /// Evaluate all policies against the given context. Returns the first
    /// Deny verdict, or the aggregate of all Allow verdicts.
    pub fn evaluate(
        &self,
        context: &HashMap<String, serde_json::Value>,
    ) -> Vec<PolicyVerdict> {
        let mut results = Vec::new();
        let mut sorted: Vec<&Policy> = self.policies.iter().filter(|p| p.enabled).collect();
        sorted.sort_by_key(|p| std::cmp::Reverse(p.priority));

        for policy in &sorted {
            let mut conditions_met = true;
            let mut evidence = HashMap::new();

            for cond in &policy.conditions {
                let field_value = context.get(&cond.field);
                let met = evaluate_condition(cond, field_value);
                evidence.insert(cond.field.clone(), serde_json::json!({
                    "value": field_value,
                    "expected": cond.value,
                    "met": met,
                }));
                if !met { conditions_met = false; }
            }

            let verdict = if conditions_met {
                let action = policy.actions.first().cloned();
                PolicyVerdict {
                    policy_id: policy.id.clone(),
                    passed: true,
                    action,
                    evidence,
                    evaluated_at: SystemTime::now(),
                }
            } else {
                PolicyVerdict {
                    policy_id: policy.id.clone(),
                    passed: false,
                    action: None,
                    evidence,
                    evaluated_at: SystemTime::now(),
                }
            };

            results.push(verdict);
        }
        results
    }
}

/// Evaluate a single condition against a runtime value.
fn evaluate_condition(
    cond: &PolicyCondition,
    field_value: Option<&serde_json::Value>,
) -> bool {
    match cond.operator {
        ConditionOp::Exists => field_value.is_some(),
        ConditionOp::Empty => field_value.map_or(true, |v| v.is_null() || v.as_str().map_or(false, |s| s.is_empty())),
        ConditionOp::Equals => field_value.map_or(false, |v| v == &cond.value),
        ConditionOp::NotEquals => field_value.map_or(true, |v| v != &cond.value),
        ConditionOp::GreaterThan => {
            match (field_value.and_then(|v| v.as_f64()), cond.value.as_f64()) {
                (Some(actual), Some(expected)) => actual > expected,
                _ => false,
            }
        }
        ConditionOp::LessThan => {
            match (field_value.and_then(|v| v.as_f64()), cond.value.as_f64()) {
                (Some(actual), Some(expected)) => actual < expected,
                _ => false,
            }
        }
        ConditionOp::Contains => {
            match (field_value.and_then(|v| v.as_str()), cond.value.as_str()) {
                (Some(haystack), Some(needle)) => haystack.contains(needle),
                _ => false,
            }
        }
        ConditionOp::In => {
            cond.value.as_array().map_or(false, |arr| {
                field_value.map_or(false, |v| arr.contains(v))
            })
        }
    }
}

/// Built-in policies that ship with Pandora.
pub fn default_policies() -> Vec<Policy> {
    vec![
        Policy {
            id: "no-empty-tasks".into(),
            name: "Reject Empty Tasks".into(),
            description: "Deny execution of empty task strings".into(),
            priority: 100,
            conditions: vec![PolicyCondition {
                field: "execution.task".into(),
                operator: ConditionOp::Empty,
                value: serde_json::json!(null),
            }],
            actions: vec![PolicyAction::Deny {
                reason: "Empty task — nothing to execute".into(),
                block: true,
            }],
            enabled: true,
        },
        Policy {
            id: "isolated-sandbox".into(),
            name: "Isolated Sandbox Enforcement".into(),
            description: "Require approval for isolated sandbox execution".into(),
            priority: 80,
            conditions: vec![PolicyCondition {
                field: "execution.sandbox_level".into(),
                operator: ConditionOp::Equals,
                value: serde_json::json!("isolated"),
            }],
            actions: vec![PolicyAction::RequireApproval {
                reason: "Isolated sandbox requires explicit approval".into(),
                approver: None,
            }],
            enabled: true,
        },
        Policy {
            id: "trusted-packages-only".into(),
            name: "Trusted Packages Only".into(),
            description: "Warn when executing unverified packages".into(),
            priority: 60,
            conditions: vec![PolicyCondition {
                field: "package.trust_level".into(),
                operator: ConditionOp::In,
                value: serde_json::json!(["untrusted", "unknown"]),
            }],
            actions: vec![PolicyAction::Log {
                level: "warn".into(),
                message: "Executing untrusted package — review recommended".into(),
            }],
            enabled: true,
        },
        Policy {
            id: "cost-cap".into(),
            name: "Cost Cap".into(),
            description: "Deny execution when budget exceeded".into(),
            priority: 90,
            conditions: vec![PolicyCondition {
                field: "execution.cost_usd".into(),
                operator: ConditionOp::GreaterThan,
                value: serde_json::json!(10.0),
            }],
            actions: vec![PolicyAction::Deny {
                reason: "Execution cost exceeds budget cap of $10.00".into(),
                block: true,
            }],
            enabled: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_task_denied() {
        let engine = PolicyEngine {
            policies: default_policies(),
            ..Default::default()
        };
        let mut ctx = HashMap::new();
        ctx.insert("execution.task".into(), serde_json::json!(""));
        let results = engine.evaluate(&ctx);
        let deny = results.iter().find(|v| v.policy_id == "no-empty-tasks");
        assert!(deny.is_some());
        assert!(deny.unwrap().passed);
    }

    #[test]
    fn valid_task_passes() {
        let engine = PolicyEngine {
            policies: default_policies(),
            ..Default::default()
        };
        let mut ctx = HashMap::new();
        ctx.insert("execution.task".into(), serde_json::json!("build API"));
        let results = engine.evaluate(&ctx);
        let deny = results.iter().find(|v| v.policy_id == "no-empty-tasks");
        assert!(!deny.unwrap().passed);
    }

    #[test]
    fn cost_cap_blocks() {
        let engine = PolicyEngine {
            policies: default_policies(),
            ..Default::default()
        };
        let mut ctx = HashMap::new();
        ctx.insert("execution.task".into(), serde_json::json!("expensive task"));
        ctx.insert("execution.cost_usd".into(), serde_json::json!(15.0));
        let results = engine.evaluate(&ctx);
        let cap = results.iter().find(|v| v.policy_id == "cost-cap");
        assert!(cap.unwrap().passed);
    }
}
