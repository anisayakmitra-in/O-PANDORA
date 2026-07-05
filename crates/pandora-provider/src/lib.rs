#![allow(clippy::field_reassign_with_default)]
//! Provider contract crate.
//!
//! This crate defines the core traits and types for model providers.
//! Provider implementations (Ollama, OpenRouter, Anthropic, etc.) live in separate crates.

pub mod capability;
pub mod compat;
pub mod constitutional;
pub mod error;
pub mod llamacpp;
pub use llamacpp::LlamaCppProvider;
pub mod manifest;
pub mod registry;
pub mod target;
pub mod traits;
pub mod types;

pub mod adapter;
#[cfg(feature = "legacy-ollama")]
pub mod custom;
pub mod discovery;
pub mod loader;
#[cfg(feature = "legacy-ollama")]
pub mod openai;

#[cfg(feature = "legacy-ollama")]
pub mod ollama;

#[cfg(feature = "legacy-ollama")]
pub mod legacy;

// Re-export core types for convenience
pub use capability::{LanguageSupport, ModelCapabilities};
pub use error::ProviderError;
pub use manifest::ProviderManifest;
pub use registry::ProviderRegistry;
pub use target::{ExecutionPolicy, ExecutionTarget, Locality};
pub use traits::Provider;
pub use types::{GenerationRequest, GenerationResponse, TokenChunk};
