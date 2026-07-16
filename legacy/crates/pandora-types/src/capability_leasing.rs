//! Capability leasing — every capability is leased, never directly invoked.
//! Defines the leasing lifecycle: request → resolve → lease → use → release.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::universal::{ExecutionProfile, Health};

/// A lease on a capability — grants exclusive or shared access for a duration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityLease {
    pub lease_id: String,
    pub capability: String,
    pub provider: String,
    pub session_id: String,
    pub priority: CapabilityPriority,
    pub granted_at_ms: u64,
    pub expires_at_ms: u64,
    pub status: LeaseStatus,
}

/// Status of a capability lease.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeaseStatus {
    Pending,
    Active,
    Expired,
    Revoked,
    Released,
    Failed,
}

/// Priority levels for capability requests.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum CapabilityPriority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
    Emergency,
}

/// A request to lease a capability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub capability: String,
    pub priority: CapabilityPriority,
    pub budget: CapabilityBudget,
    pub timeout_ms: u64,
    pub preferred_provider: Option<String>,
}

/// Budget constraints for a capability lease.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityBudget {
    pub max_duration_ms: u64,
    pub max_cost_cents: u64,
    pub max_memory_mb: u64,
    pub max_cpu_ms: u64,
}

impl Default for CapabilityBudget {
    fn default() -> Self {
        Self {
            max_duration_ms: 60_000,
            max_cost_cents: 100,
            max_memory_mb: 1024,
            max_cpu_ms: 30_000,
        }
    }
}

/// Result of resolving a capability request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityResolution {
    pub lease: CapabilityLease,
    pub provider: String,
    pub execution_profile: ExecutionProfile,
    pub conflicts: Vec<String>,
}

/// A session that holds multiple capability leases.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilitySession {
    pub session_id: String,
    pub leases: Vec<CapabilityLease>,
    pub created_at_ms: u64,
    pub health: Health,
    pub total_cost_cents: u64,
}

/// A pool of available capabilities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityPool {
    pub capabilities: BTreeMap<String, Vec<String>>,
    pub health: Health,
}

/// Failure modes for capability leasing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityFailure {
    pub lease_id: String,
    pub reason: FailureReason,
    pub timestamp_ms: u64,
    pub retryable: bool,
}

/// Reasons a capability lease can fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FailureReason {
    Timeout,
    BudgetExhausted,
    ProviderUnavailable,
    Conflict,
    Revoked,
    InternalError,
}

/// Statistics for capability usage.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CapabilityStatistics {
    pub total_leases: u64,
    pub active_leases: u64,
    pub failed_leases: u64,
    pub total_cost_cents: u64,
    pub avg_duration_ms: u64,
}

/// Telemetry for capability leasing.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CapabilityTelemetry {
    pub metrics: BTreeMap<String, u64>,
    pub events: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lease_serde() {
        let l = CapabilityLease {
            lease_id: "l1".into(),
            capability: "filesystem".into(),
            provider: "native".into(),
            session_id: "s1".into(),
            priority: CapabilityPriority::Normal,
            granted_at_ms: 0,
            expires_at_ms: 60_000,
            status: LeaseStatus::Active,
        };
        let json = serde_json::to_string(&l).unwrap();
        let _: CapabilityLease = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn priority_ordering() {
        assert!(CapabilityPriority::Emergency > CapabilityPriority::Critical);
        assert!(CapabilityPriority::Critical > CapabilityPriority::High);
        assert!(CapabilityPriority::High > CapabilityPriority::Normal);
    }

    #[test]
    fn budget_default() {
        assert_eq!(CapabilityBudget::default().max_duration_ms, 60_000);
    }

    #[test]
    fn session_serde() {
        let s = CapabilitySession {
            session_id: "s1".into(),
            leases: vec![],
            created_at_ms: 0,
            health: Health::Healthy,
            total_cost_cents: 0,
        };
        let json = serde_json::to_string(&s).unwrap();
        let _: CapabilitySession = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn failure_reasons() {
        let f = CapabilityFailure {
            lease_id: "l1".into(),
            reason: FailureReason::Timeout,
            timestamp_ms: 0,
            retryable: true,
        };
        assert!(f.retryable);
    }
}
