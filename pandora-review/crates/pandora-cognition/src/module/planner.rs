use async_trait::async_trait;

use anyhow::Result;

use pandora_provider::provider::Provider;

use pandora_provider::types::{
    GenerationRequest,
};

use crate::module::Module;

use crate::signature::examples::planner::{
    PlannerInput,
    PlannerOutput,
    PlannerSignature,
};

pub struct PlannerModule<P>
where
    P: Provider,
{
    provider:
        P,
}

impl<P> PlannerModule<P>
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
impl<P> Module<PlannerSignature>
    for PlannerModule<P>

where
    P:
        Provider
        + Send
        + Sync,
{

    async fn forward(

        &self,

        input:
            PlannerInput,

    ) -> Result<
        PlannerOutput
    > {

        let prompt =
            format!(

                "{}\n\nObjective:\n{}",

                PlannerSignature
                    ::instruction(),

                input.objective
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
                            0.3,

                        max_tokens:
                            256,
                    }
                )
                .await?;

        let steps =
            response
                .text
                .lines()
                .map(
                    |s| {
                        s.trim()
                            .to_string()
                    }
                )
                .filter(
                    |s| {
                        !s.is_empty()
                    }
                )
                .collect();

        Ok(
            PlannerOutput {
                steps
            }
        )
    }

    fn name(
        &self
    ) -> &'static str {

        "planner-module"
    }
}
