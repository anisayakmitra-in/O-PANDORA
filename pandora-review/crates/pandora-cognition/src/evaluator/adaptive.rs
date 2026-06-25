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
pub struct EvaluatorMetric {

    pub metric_name:
        String,

    pub weight:
        f32,

    pub description:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct AdaptiveEvaluator {

    pub evaluator_id:
        String,

    pub metrics:
        Vec<EvaluatorMetric>,

    pub evolution_generation:
        u32,
}

pub struct AdaptiveEvaluatorEngine;

impl AdaptiveEvaluatorEngine {

    pub fn evolve(

        evaluator:
            &AdaptiveEvaluator,

    ) -> AdaptiveEvaluator {

        let mut metrics =
            evaluator.metrics.clone();

        for metric in &mut metrics {

            if metric.weight < 1.0 {

                metric.weight += 0.05;
            }
        }

        AdaptiveEvaluator {

            evaluator_id:
                format!(
                    "{}-evolved",
                    evaluator.evaluator_id
                ),

            metrics,

            evolution_generation:
                evaluator
                    .evolution_generation
                    + 1,
        }
    }

    pub fn detect_metric_drift(

        evaluator:
            &AdaptiveEvaluator,

    ) -> bool {

        evaluator
            .metrics
            .iter()
            .any(
                |metric| {

                    metric.weight
                        >
                        2.0
                }
            )
    }
}
