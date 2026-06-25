use async_trait::async_trait;

use anyhow::Result;

#[derive(
    Debug,
    Clone,
)]
pub struct EvaluationScore {

    pub score:
        f32,

    pub reasoning:
        String,
}

#[async_trait]
pub trait Evaluator<I, O>
where
    I: Send + Sync,
    O: Send + Sync,
{

    async fn evaluate(

        &self,

        input:
            &I,

        output:
            &O,

    ) -> Result<
        EvaluationScore
    >;

    fn name(
        &self
    ) -> &'static str;
}

pub mod planner;

pub mod adaptive;
