use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryArtifact {
    pub memory_id: String,

    pub lineage_depth: usize,

    pub survivability: f64,

    pub corruption_risk: f64,

    pub continuity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceDirective {
    pub memory_id: String,

    pub action: String,

    pub archive_required: bool,

    pub quarantine: bool,
}

pub struct AnubisMemoryGovernor;

impl AnubisMemoryGovernor {
    pub fn govern(artifacts: &[MemoryArtifact]) -> Vec<PersistenceDirective> {
        let mut directives = Vec::new();

        for artifact in artifacts {
            println!("[ANUBIS] evaluating {}", artifact.memory_id);

            let quarantine = artifact.corruption_risk > 0.80;

            let action = if quarantine {
                "quarantine-memory"
            } else if artifact.survivability > 0.90 && artifact.continuity_score > 0.88 {
                "persist-sovereign"
            } else if artifact.lineage_depth > 10 {
                "archive-lineage"
            } else if artifact.continuity_score < 0.60 {
                "repair-continuity"
            } else {
                "maintain-active"
            };

            directives.push(PersistenceDirective {
                memory_id: artifact.memory_id.clone(),

                action: action.into(),

                archive_required: artifact.lineage_depth > 8,

                quarantine,
            });
        }

        directives
    }
}
