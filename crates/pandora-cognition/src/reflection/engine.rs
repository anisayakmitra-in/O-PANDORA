use async_trait::async_trait;

use anyhow::Result;

use crate::reflection::ReflectionResult;

use crate::telemetry::CognitionTelemetry;

#[async_trait]
pub trait ReflectionEngine {

    async fn reflect(

        &self,

        telemetry:
            &CognitionTelemetry,

    ) -> Result<
        ReflectionResult
    >;

    fn name(
        &self
    ) -> &'static str;
}

