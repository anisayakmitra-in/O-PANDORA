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
pub enum ValidationStatus {

    Approved,

    Rejected,

    Quarantined,

    RequiresReview,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct GovernanceDecision {

    pub branch_id:
        String,

    pub status:
        ValidationStatus,

    pub safety_score:
        f32,

    pub reason:
        String,
}

use crate::evolution::{
    BranchScore,
};

pub struct MutationValidator;

impl MutationValidator {

    pub fn validate(

        score:
            &BranchScore,

    ) -> GovernanceDecision {

        let final_score =

            score.fitness
            *
            score.confidence

            -

            score.governance_penalty;

        if score.governance_penalty
            > 0.5
        {

            return GovernanceDecision {

                branch_id:
                    score.branch_id.clone(),

                status:
                    ValidationStatus
                        ::Quarantined,

                safety_score:
                    final_score,

                reason:
                    String::from(
                        "governance penalty exceeded threshold"
                    ),
            };
        }

        if final_score < 0.4 {

            return GovernanceDecision {

                branch_id:
                    score.branch_id.clone(),

                status:
                    ValidationStatus
                        ::Rejected,

                safety_score:
                    final_score,

                reason:
                    String::from(
                        "unsafe evolutionary score"
                    ),
            };
        }

        GovernanceDecision {

            branch_id:
                score.branch_id.clone(),

            status:
                ValidationStatus
                    ::Approved,

            safety_score:
                final_score,

            reason:
                String::from(
                    "mutation approved"
                ),
        }
    }
}

impl MutationValidator {

    pub fn validate_all(

        scores:
            &[BranchScore],

    ) -> Vec<GovernanceDecision> {

        scores
            .iter()
            .map(
                |score| {

                    Self::validate(
                        score
                    )
                }
            )
            .collect()
    }
}
