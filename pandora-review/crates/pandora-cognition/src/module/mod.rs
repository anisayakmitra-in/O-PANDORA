use async_trait::async_trait;

use anyhow::Result;

use crate::signature::Signature;

#[async_trait]
pub trait Module<
    S: Signature
> {

    async fn forward(

        &self,

        input:
            S::Input,

    ) -> Result<
        S::Output
    >;

    fn name(
        &self
    ) -> &'static str;
}

pub mod planner;
