use async_trait::async_trait;

use anyhow::Result;

use crate::dataset::CognitionDatasetEntry;

#[derive(
    Debug,
    Clone,
)]
pub struct OptimizationResult {

    pub optimizer:
        String,

    pub mutations:
        Vec<String>,

    pub reasoning:
        String,
}

#[async_trait]
pub trait Optimizer {

    async fn optimize(

        &self,

        dataset:
            &[CognitionDatasetEntry],

    ) -> Result<
        OptimizationResult
    >;

    fn name(
        &self
    ) -> &'static str;
}

pub mod gepa;
