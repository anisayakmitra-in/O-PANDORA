use serde::{Deserialize, Serialize};

/// Request for text generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    /// The prompt to generate from.
    pub prompt: String,

    /// Model identifier to use.
    pub model: String,

    /// Sampling temperature (0.0 = deterministic).
    pub temperature: f32,

    /// Maximum tokens to generate.
    pub max_tokens: usize,
}

/// Response from text generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    /// Generated text.
    pub text: String,

    /// Number of tokens used.
    pub tokens_used: usize,

    /// Reason generation stopped (e.g., "stop", "length", "error").
    pub finish_reason: String,
}

/// A single token chunk for streaming responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenChunk {
    /// Token text.
    pub text: String,
}
