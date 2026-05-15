use anyhow::Result;

use async_trait::async_trait;

use crate::dataset::CognitionDatasetEntry;

use crate::optimizer::{
    OptimizationResult,
    Optimizer,
};

pub struct GepaOptimizer;

#[async_trait]
impl Optimizer
    for GepaOptimizer
{

    async fn optimize(

        &self,

        dataset:
            &[CognitionDatasetEntry],

    ) -> Result<
        OptimizationResult
    > {

        let mut mutations =
            Vec::new();

        let failures =
            dataset
                .iter()
                .filter(
                    |e| {
                        !e.telemetry.success
                    }
                )
                .count();

        let low_scores =
            dataset
                .iter()
                .filter(
                    |e| {

                        e.telemetry
                            .score
                            .unwrap_or(0.0)
                            < 0.5
                    }
                )
                .count();

        if failures > 0 {

            mutations.push(
                String::from(
                    "improve reasoning robustness"
                )
            );
        }

        if low_scores > 0 {

            mutations.push(
                String::from(
                    "improve task decomposition quality"
                )
            );
        }

        if mutations.is_empty() {

            mutations.push(
                String::from(
                    "maintain current cognition strategy"
                )
            );
        }

        Ok(
            OptimizationResult {

                optimizer:
                    String::from(
                        "gepa-optimizer"
                    ),

                mutations,

                reasoning:
                    format!(
                        "analyzed {} cognition traces",
                        dataset.len()
                    ),
            }
        )
    }

    fn name(
        &self
    ) -> &'static str {

        "gepa-optimizer"
    }
}

