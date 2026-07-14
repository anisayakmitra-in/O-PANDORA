use anyhow::Result;

use async_trait::async_trait;

use pandora_provider::provider::Provider;

use pandora_provider::types::{
    GenerationRequest,
};

use crate::dataset::CognitionDatasetEntry;

use crate::evolution::{
    MutationProposal,
};

use crate::evolution::engine::{
    MutationEngine,
};

pub struct LlmMutationEngine<P>
where
    P: Provider,
{
    provider:
        P,
}

impl<P> LlmMutationEngine<P>
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
impl<P> MutationEngine
    for LlmMutationEngine<P>

where

    P:
        Provider
        + Send
        + Sync,
{

    async fn generate_mutations(

        &self,

        dataset:
            &[CognitionDatasetEntry],

    ) -> Result<
        Vec<MutationProposal>
    > {

        let mut proposals =
            Vec::new();

        for entry in dataset {

            let prompt =
                format!(

r#"
Analyze this cognition execution.

OBJECTIVE:
{}

OUTPUT:
{}

SUCCESS:
{}

SCORE:
{:?}

Generate:
1. cognition weaknesses
2. improved reasoning strategy
3. better decomposition approach
4. mutation proposal
"#,

                    entry.objective,

                    entry.output,

                    entry.telemetry.success,

                    entry.telemetry.score
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
                                0.5,

                            max_tokens:
                                512,
                        }
                    )
                    .await?;

            proposals.push(
                MutationProposal {

                    target:
                        String::from(
                            "planner-module"
                        ),

                    current_behavior:
                        String::from(
                            "basic reasoning"
                        ),

                    proposed_behavior:
                        response.text,

                    reasoning:
                        String::from(
                            "llm-generated mutation strategy"
                        ),

                    confidence:
                        0.82,
                }
            );
        }

        Ok(
            proposals
        )
    }

    fn name(
        &self
    ) -> &'static str {

        "llm-mutation-engine"
    }
}
