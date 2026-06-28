use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::capability::ModelCapabilities;
use crate::error::ProviderError;
use crate::manifest::ProviderManifest;
use crate::types::{GenerationRequest, GenerationResponse, TokenChunk};

/// Core provider trait for model inference.
///
/// Implementors expose a single async interface for generation,
/// streaming generation, and embedding, plus a manifest of capabilities.
#[async_trait]
pub trait Provider: Send + Sync {
    /// Unique provider identifier (e.g., "ollama", "openrouter", "anthropic").
    fn name(&self) -> &'static str;

    /// Provider manifest with metadata and capabilities.
    fn manifest(&self) -> ProviderManifest;

    /// Generate a completion (non-streaming).
    async fn generate(
        &self,
        request: GenerationRequest,
        cancel: CancellationToken,
    ) -> Result<GenerationResponse, ProviderError>;

    /// Generate a completion with streaming tokens.
    async fn stream_generate(
        &self,
        request: GenerationRequest,
        cancel: CancellationToken,
        tx: mpsc::Sender<TokenChunk>,
    ) -> Result<(), ProviderError>;

    /// Generate embeddings.
    async fn embed(
        &self,
        text: String,
        cancel: CancellationToken,
    ) -> Result<Vec<f32>, ProviderError>;

    /// Get model capabilities.
    fn capabilities(&self) -> ModelCapabilities {
        self.manifest().capabilities
    }
}
