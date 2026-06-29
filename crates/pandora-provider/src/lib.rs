//! Provider contract crate.
//!
//! This crate defines the core traits and types for model providers.
//! Provider implementations (Ollama, OpenRouter, Anthropic, etc.) live in separate crates.

pub mod capability;
pub mod compat;
pub mod constitutional;
pub mod error;
pub mod manifest;
pub mod registry;
pub mod traits;
pub mod types;

#[cfg(feature = "legacy-ollama")]
pub mod legacy;

// Re-export core types for convenience
pub use capability::{LanguageSupport, ModelCapabilities};
pub use error::ProviderError;
pub use manifest::ProviderManifest;
pub use registry::ProviderRegistry;
pub use traits::Provider;
pub use types::{GenerationRequest, GenerationResponse, TokenChunk};
