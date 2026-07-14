use anyhow::Result;

use async_trait::async_trait;

use crate::reflection::{
    ReflectionResult,
};

use crate::reflection::engine::{
    ReflectionEngine,
};

use crate::telemetry::CognitionTelemetry;

pub struct BasicReflectionEngine;

#[async_trait]
impl ReflectionEngine
    for BasicReflectionEngine
{

    async fn reflect(

        &self,

        telemetry:
            &CognitionTelemetry,

    ) -> Result<
        ReflectionResult
    > {

        let mut strengths =
            Vec::new();

        let mut weaknesses =
            Vec::new();

        let mut improvements =
            Vec::new();

        if telemetry.success {

            strengths.push(
                String::from(
                    "execution completed successfully"
                )
            );
        } else {

            weaknesses.push(
                String::from(
                    "execution failed"
                )
            );

            improvements.push(
                String::from(
                    "improve reasoning robustness"
                )
            );
        }

        if let Some(score) =
            telemetry.score
        {

            if score < 0.5 {

                weaknesses.push(
                    String::from(
                        "low evaluation score"
                    )
                );

                improvements.push(
                    String::from(
                        "improve task decomposition quality"
                    )
                );
            } else {

                strengths.push(
                    String::from(
                        "acceptable evaluation score"
                    )
                );
            }
        }

        Ok(
            ReflectionResult {

                summary:
                    String::from(
                        "reflection completed"
                    ),

                strengths,

                weaknesses,

                improvements,
            }
        )
    }

    fn name(
        &self
    ) -> &'static str {

        "basic-reflection-engine"
    }
}
