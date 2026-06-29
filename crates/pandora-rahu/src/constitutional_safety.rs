//! Constitutional Safety.
//!
//! Nothing evolves unless:
//! GEPA completes -> DSR completes -> Shadow Council approves
//! -> PANOPTES approves -> Checkpoint exists -> Rollback exists
//! -> Benchmark succeeds -> Apply.
//!
//! This pipeline is mandatory for official Pandora objects.
//! Community objects may opt out.

use serde::{Deserialize, Serialize};

/// Safety gate status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SafetyGateStatus {
    Pending,
    Passed,
    Failed,
    Skipped,
}

/// A safety gate in the evolution pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyGate {
    pub name: String,
    pub status: SafetyGateStatus,
    pub required: bool,
}

/// Constitutional safety report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyReport {
    pub target_id: String,
    pub gates: Vec<SafetyGate>,
    pub approved: bool,
    pub is_official: bool,
    pub timestamp_ms: u64,
}

/// Constitutional safety runtime.
pub struct ConstitutionalSafety;

impl ConstitutionalSafety {
    pub fn new() -> Self {
        ConstitutionalSafety
    }

    pub fn evaluate(&self, target_id: &str, is_official: bool) -> SafetyReport {
        let gate_names = [
            "gepa_complete",
            "dsr_complete",
            "shadow_council_approved",
            "panoptes_approved",
            "checkpoint_exists",
            "rollback_exists",
            "benchmark_succeeded",
        ];
        let gates: Vec<SafetyGate> = gate_names
            .iter()
            .map(|name| SafetyGate {
                name: name.to_string(),
                status: SafetyGateStatus::Pending,
                required: is_official,
            })
            .collect();
        SafetyReport {
            target_id: target_id.to_string(),
            gates,
            approved: !is_official,
            is_official,
            timestamp_ms: 0,
        }
    }
}

impl Default for ConstitutionalSafety {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_object_requires_all_gates() {
        let cs = ConstitutionalSafety::new();
        let report = cs.evaluate("gene-1", true);
        assert!(!report.approved);
        assert_eq!(report.gates.len(), 7);
    }

    #[test]
    fn community_object_may_opt_out() {
        let cs = ConstitutionalSafety::new();
        let report = cs.evaluate("gene-2", false);
        assert!(report.approved);
    }
}
