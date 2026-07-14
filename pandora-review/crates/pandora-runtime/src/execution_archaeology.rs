use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchaeologyRecord {
    pub execution_id: String,

    pub domain: String,

    pub substrate: String,

    pub governance_interventions: usize,

    pub mutation_depth: usize,

    pub replay_integrity: f64,

    pub telemetry_fidelity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchaeologyDirective {
    pub execution_id: String,

    pub preserve: bool,

    pub replayable: bool,

    pub archive_priority: String,

    pub governance_review: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchaeologyState {
    pub archaeology_integrity: f64,

    pub replay_stability: f64,

    pub sovereign_archive_ready: bool,

    pub directives: Vec<ArchaeologyDirective>,
}

pub struct ExecutionArchaeologyEngine;

impl ExecutionArchaeologyEngine {
    pub fn preserve(records: &[ArchaeologyRecord]) -> ArchaeologyState {
        let mut directives = Vec::new();

        let mut integrity = 0.0;

        let mut replay = 0.0;

        for record in records {
            println!("[ARCHAEOLOGY] execution={}", record.execution_id);

            let preserve = record.telemetry_fidelity > 0.80 && record.replay_integrity > 0.82;

            let replayable = record.replay_integrity > 0.85;

            let governance_review = record.governance_interventions > 3;

            let archive_priority = if record.mutation_depth > 8 {
                "critical-lineage"
            } else if replayable {
                "high-value"
            } else {
                "standard"
            };

            directives.push(ArchaeologyDirective {
                execution_id: record.execution_id.clone(),

                preserve,

                replayable,

                archive_priority: archive_priority.into(),

                governance_review,
            });

            integrity += record.telemetry_fidelity;

            replay += record.replay_integrity;
        }

        let count = records.len() as f64;

        let archaeology_integrity = integrity / count;

        let replay_stability = replay / count;

        let sovereign_archive_ready = archaeology_integrity > 0.85 && replay_stability > 0.84;

        ArchaeologyState {
            archaeology_integrity,

            replay_stability,

            sovereign_archive_ready,

            directives,
        }
    }
}
