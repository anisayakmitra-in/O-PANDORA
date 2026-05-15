use anyhow::Result;

use async_trait::async_trait;

use pandora_provider::provider::Provider;

use pandora_provider::types::{
    GenerationRequest,
};

use crate::reflection::{
    ReflectionResult,
};

use crate::reflection::engine::{
    ReflectionEngine,
};

use crate::telemetry::CognitionTelemetry;

pub struct LlmReflectionEngine<P>
where
    P: Provider,
{
    provider:
        P,
}

impl<P> LlmReflectionEngine<P>
where
    P: Provider,
{

    pub fn new(
        provider: P
    ) -> Self {

        Self {
            provider
        }
    }
}

#[async_trait]
impl<P> ReflectionEngine
    for LlmReflectionEngine<P>

where

    P:
        Provider
        + Send
        + Sync,
{

    async fn reflect(

        &self,

        telemetry:
            &CognitionTelemetry,

    ) -> Result<
        ReflectionResult
    > {

        let prompt =
            format!(

r#"
Analyze this cognition execution.

Module:
{}

Success:
{}

Score:
{:?}

Notes:
{:?}

Provide:
1. strengths
2. weaknesses
3. improvements
"#,

                telemetry.module_name,

                telemetry.success,

                telemetry.score,

                telemetry.notes
            );

        let response =
            self.provider
                .generate(
                    GenerationRequest {

                        prompt,

                        model:
                            String::from(
                                "qwen2.5-coder:7b"
                            ),

                        temperature:
                            0.4,

                        max_tokens:
                            512,
                    }
                )
                .await?;

        Ok(
            ReflectionResult {

                summary:
                    response.text,

                strengths:
                    vec![
                        String::from(
                            "llm-generated reflection"
                        )
                    ],

                weaknesses:
                    vec![
                        String::from(
                            "requires structured parsing"
                        )
                    ],

                improvements:
                    vec![
                        String::from(
                            "improve cognition strategies"
                        )
                    ],
            }
        )
    }

    fn name(
        &self
    ) -> &'static str {

        "llm-reflection-engine"
    }
}
