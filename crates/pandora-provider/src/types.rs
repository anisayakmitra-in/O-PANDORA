use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct GenerationRequest {

    pub prompt:
        String,

    pub model:
        String,

    pub temperature:
        f32,

    pub max_tokens:
        usize,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct GenerationResponse {

    pub text:
        String,

    pub tokens_used:
        usize,

    pub finish_reason:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct TokenChunk {

    pub text:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct LanguageSupport {

    pub language_code:
        String,

    pub language_name:
        String,

    pub country:
        String,

    pub confidence:
        f32,
}

pub struct ModelCapabilities {

    pub multilingual:
        bool,

    pub supported_languages:
        Vec<LanguageSupport>,

    pub context_window:
        usize,

    pub supports_streaming:
        bool,

    pub supports_embeddings:
        bool,

    pub supports_tools:
        bool,
}

#[derive(
    Debug,
    thiserror::Error,
)]
pub enum ProviderError {

    #[error(
        "provider unavailable: {0}"
    )]
    Unavailable(
        String
    ),

    #[error(
        "generation failed: {0}"
    )]
    GenerationFailed(
        String
    ),

    #[error(
        "request cancelled"
    )]
    Cancelled,

    #[error(
        "embedding failed: {0}"
    )]
    EmbeddingFailed(
        String
    ),
}
