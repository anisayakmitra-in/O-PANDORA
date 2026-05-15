use async_trait::async_trait;

use anyhow::Result;

use crate::dataset::CognitionDatasetEntry;

use crate::evolution::MutationProposal;

#[async_trait]
pub trait MutationEngine {

    async fn generate_mutations(

        &self,

        dataset:
            &[CognitionDatasetEntry],

    ) -> Result<
        Vec<MutationProposal>
    >;

    fn name(
        &self
    ) -> &'static str;
}
