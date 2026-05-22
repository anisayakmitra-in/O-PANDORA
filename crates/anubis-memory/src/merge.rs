use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum MergeStrategy {

    Consensus,

    Weighted,

    GovernancePriority,

    EvolutionaryBest,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct SynthesizedBranch {

    pub synthesized_id:
        String,

    pub source_branches:
        Vec<String>,

    pub strategy:
        MergeStrategy,

    pub summary:
        String,
}

pub struct MergeEngine;

impl MergeEngine {

    pub fn synthesize(

        branch_ids:
            Vec<String>,

        strategy:
            MergeStrategy,

        summary:
            String,

    ) -> SynthesizedBranch {

        SynthesizedBranch {

            synthesized_id:
                format!(
                    "merge-{}",
                    uuid::Uuid
                        ::new_v4()
                ),

            source_branches:
                branch_ids,

            strategy,

            summary,
        }
    }
}

impl MergeEngine {

    pub fn consensus_score(

        branch_count:
            usize,

        agreement_ratio:
            f32,

    ) -> f32 {

        branch_count as f32
            *
            agreement_ratio
    }
}
