use anyhow::Result;

use async_trait::async_trait;

#[async_trait]
pub trait Pipeline<I, O>
where
    I: Send + Sync,
    O: Send + Sync,
{

    async fn execute(
        &self,
        input: I,
    ) -> Result<O>;

    fn name(
        &self
    ) -> &'static str;
}

pub mod sequential;
