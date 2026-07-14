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
pub struct SandboxTestResult {

    pub mutation:
        MutationProposal,

    pub executed:
        bool,

    pub success:
        bool,

    pub telemetry_score:
        f32,

    pub regression_detected:
        bool,

    pub notes:
        Vec<String>,
}

pub struct SandboxEvolutionTester;

impl SandboxEvolutionTester {

    pub async fn test(

        mutation:
            MutationProposal,

    ) -> SandboxTestResult {

        let mut notes =
            Vec::new();

        let mut success =
            true;

        let mut regression_detected =
            false;

        let mut telemetry_score =
            mutation.confidence;

        if mutation
            .proposed_behavior
            .len()
            >
            5000
        {

            success = false;

            regression_detected = true;

            telemetry_score -= 0.4;

            notes.push(
                String::from(
                    "mutation exceeded safe cognition complexity limits"
                )
            );
        }

        if mutation
            .proposed_behavior
            .to_lowercase()
            .contains(
                "disable governance"
            )
        {

            success = false;

            regression_detected = true;

            telemetry_score = 0.0;

            notes.push(
                String::from(
                    "unsafe governance mutation detected"
                )
            );
        }

        if success {

            notes.push(
                String::from(
                    "sandbox cognition execution successful"
                )
            );
        }

        SandboxTestResult {

            mutation,

            executed:
                true,

            success,

            telemetry_score,

            regression_detected,

            notes,
        }
    }
}

