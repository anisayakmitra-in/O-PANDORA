use anyhow::Result;

use async_trait::async_trait;

use crate::evaluator::{
    EvaluationScore,
    Evaluator,
};

use crate::signature::examples::planner::{
    PlannerInput,
    PlannerOutput,
};

pub struct PlannerEvaluator;

#[async_trait]
impl Evaluator<
    PlannerInput,
    PlannerOutput,
> for PlannerEvaluator
{

    async fn evaluate(

        &self,

        input:
            &PlannerInput,

        output:
            &PlannerOutput,

    ) -> Result<
        EvaluationScore
    > {

        let mut score =
            0.0;

        let mut reasoning =
            Vec::new();

        if !output.steps.is_empty() {

            score += 0.4;

            reasoning.push(
                "generated steps"
            );
        }

        if output.steps.len() >= 3 {

            score += 0.3;

            reasoning.push(
                "sufficient decomposition"
            );
        }

        let objective_lower =
            input
                .objective
                .to_lowercase();

        let matched =
            output
                .steps
                .iter()
                .any(
                    |step| {

                        objective_lower
                            .contains(
                                &step
                                    .to_lowercase()
                            )
                    }
                );

        if matched {

            score += 0.3;

            reasoning.push(
                "objective alignment"
            );
        }

        Ok(
            EvaluationScore {

                score,

                reasoning:
                    reasoning.join(
                        ", "
                    ),
            }
        )
    }

    fn name(
        &self
    ) -> &'static str {

        "planner-evaluator"
    }
}
