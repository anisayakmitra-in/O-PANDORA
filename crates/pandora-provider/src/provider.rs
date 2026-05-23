use async_trait::async_trait;

use tokio::sync::mpsc;

use tokio_util::sync::CancellationToken;

use crate::types::{
    GenerationRequest, GenerationResponse, ModelCapabilities, ProviderError, TokenChunk,
};

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &'static str;

    async fn generate(
        &self,

        request: GenerationRequest,

        cancel: CancellationToken,
    ) -> Result<GenerationResponse, ProviderError>;

    async fn stream_generate(
        &self,

        request: GenerationRequest,

        cancel: CancellationToken,

        tx: mpsc::Sender<TokenChunk>,
    ) -> Result<(), ProviderError>;

    async fn embed(
        &self,

        text: String,

        cancel: CancellationToken,
    ) -> Result<Vec<f32>, ProviderError>;

    fn capabilities(&self) -> ModelCapabilities;
}
