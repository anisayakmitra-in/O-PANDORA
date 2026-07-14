//! Governance runtime types — every execution passes through verification,
//! trust, policy, audit, and governance.

use serde::{Deserialize, Serialize};

/// Governance verdict for an execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceVerdict {
    pub session_id: String,
    pub approved: bool,
    pub trust_score: f64,
    pub policy_violations: Vec<PolicyViolation>,
    pub audit_entries: Vec<AuditEntry>,
    pub timestamp_ms: u64,
}

/// A policy violation detected during governance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyViolation {
    pub policy_id: String,
    pub severity: ViolationSeverity,
    pub message: String,
}

/// Severity of a policy violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// An audit entry for governance tracking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditEntry {
    pub entry_id: String,
    pub action: String,
    pub subject: String,
    pub result: String,
    pub timestamp_ms: u64,
}
