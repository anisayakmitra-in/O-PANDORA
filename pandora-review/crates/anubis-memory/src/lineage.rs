use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationLineage {
    pub branch_id: String,

    pub parent_branch: Option<String>,

    pub mutation_epoch: u64,

    pub checkpoint_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackCheckpoint {
    pub checkpoint_id: String,

    pub branch_id: String,

    pub timestamp: u64,

    pub description: String,
}

pub struct LineageEngine;

impl LineageEngine {
    pub fn ancestry<'a>(
        lineages: &'a [MutationLineage],

        branch_id: &str,
    ) -> Vec<&'a MutationLineage> {
        let mut collected = Vec::new();

        let mut current = Some(branch_id.to_string());

        while let Some(id) = current {
            if let Some(lineage) = lineages.iter().find(|l| l.branch_id == id) {
                collected.push(lineage);

                current = lineage.parent_branch.clone();
            } else {
                break;
            }
        }

        collected
    }
}

pub struct RollbackEngine;

impl RollbackEngine {
    pub fn latest_checkpoint<'a>(
        checkpoints: &'a [RollbackCheckpoint],

        branch_id: &str,
    ) -> Option<&'a RollbackCheckpoint> {
        checkpoints
            .iter()
            .filter(|checkpoint| checkpoint.branch_id == branch_id)
            .max_by_key(|checkpoint| checkpoint.timestamp)
    }
}
