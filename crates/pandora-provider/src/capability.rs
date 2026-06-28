use serde::{Deserialize, Serialize};

/// Capabilities a model provider may support.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelCapabilities {
    /// Whether the model supports multiple languages
    pub multilingual: bool,

    /// List of supported languages with confidence scores
    pub supported_languages: Vec<LanguageSupport>,

    /// Maximum context window in tokens
    pub context_window: usize,

    /// Whether streaming generation is supported
    pub supports_streaming: bool,

    /// Whether embedding generation is supported
    pub supports_embeddings: bool,

    /// Whether tool/function calling is supported
    pub supports_tools: bool,
}

/// Language support metadata for a model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSupport {
    /// ISO language code (e.g., "en", "zh")
    pub language_code: String,

    /// Human-readable language name
    pub language_name: String,

    /// Primary country/region
    pub country: String,

    /// Confidence score for language support (0.0-1.0)
    pub confidence: f32,
}
