use anyhow::Result;

use async_trait::async_trait;

use crate::pipeline::Pipeline;

pub struct SequentialPipeline<F>
{
    executor: F,

    pipeline_name:
        &'static str,
}

impl<F> SequentialPipeline<F> {

    pub fn new(

        pipeline_name:
            &'static str,

        executor:
            F,

    ) -> Self {

        Self {

            executor,

            pipeline_name,
        }
    }
}

#[async_trait]
impl<
    I,
    O,
    F,
    Fut,
> Pipeline<I, O>
    for SequentialPipeline<F>

where

    I:
        Send
        + Sync
        + 'static,

    O:
        Send
        + Sync
        + 'static,

    F:
        Fn(I) -> Fut
        + Send
        + Sync,

    Fut:
        std::future::Future<
            Output = Result<O>
        >
        + Send,
{

    async fn execute(

        &self,

        input:
            I,

    ) -> Result<O> {

        (self.executor)(input)
            .await
    }

    fn name(
        &self
    ) -> &'static str {

        self.pipeline_name
    }
}
