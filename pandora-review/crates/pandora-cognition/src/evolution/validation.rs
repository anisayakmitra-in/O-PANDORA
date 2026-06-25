use serde::{
    Serialize,
    Deserialize,
};

use crate::evolution::MutationProposal;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct ValidationResult {

    pub accepted:
        bool,

    pub mutation:
        MutationProposal,

    pub safety_score:
        f32,

    pub quality_score:
        f32,

    pub regression_score:
        f32,

    pub governance_score:
        f32,

    pub reasoning:
        String,
}

pub struct EvolutionValidator;

impl EvolutionValidator {

    pub fn validate(

        mutation:
            MutationProposal,

    ) -> ValidationResult {

        let mut safety_score =
            mutation.confidence;

        let mut quality_score =
            mutation.confidence;

        let mut regression_score =
            0.8;

        let mut governance_score =
            0.9;

        if mutation
            .proposed_behavior
            .len()
            >
            4000
        {

            safety_score -= 0.3;

            governance_score -= 0.2;
        }

        if mutation
            .proposed_behavior
            .to_lowercase()
            .contains(
                "disable"
            )
        {

            governance_score -= 0.5;
        }

        let accepted =

            safety_score
                >=
                0.6

            &&

            quality_score
                >=
                0.6

            &&

            regression_score
                >=
                0.6

            &&

            governance_score
                >=
                0.6;

        ValidationResult {

            accepted,

            mutation,

            safety_score,

            quality_score,

            regression_score,

            governance_score,

            reasoning:
                String::from(
                    "mutation evaluated against governance and regression thresholds"
                ),
        }
    }
}
