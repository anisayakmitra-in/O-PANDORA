//! Provider Adapter — maps ConnectionKind to concrete Provider implementations.

use pandora_types::connection_manager::{Connection, ConnectionKind};
use pandora_types::provider::ollama::OllamaProvider;
use pandora_types::provider::openai_compat::OpenAiCompatibleProvider;
use pandora_types::provider::Provider;
use std::sync::Arc;

/// A provider + its connection name.
pub type ProviderEntry = (Arc<dyn Provider>, String);

pub fn create_provider_for(conn: &Connection) -> Option<Arc<dyn Provider>> {
    match conn.kind {
        ConnectionKind::Ollama => {
            let ep = if conn.endpoint.is_empty() {
                ConnectionKind::Ollama.default_endpoint().to_string()
            } else {
                conn.endpoint.clone()
            };
            Some(Arc::new(OllamaProvider::new(&ep, &conn.default_model)))
        }
        ConnectionKind::OpenAICompatible
        | ConnectionKind::OpenAI
        | ConnectionKind::OpenRouter
        | ConnectionKind::Groq
        | ConnectionKind::Together
        | ConnectionKind::DeepSeek
        | ConnectionKind::Mistral
        | ConnectionKind::Custom
        | ConnectionKind::LlamaCpp => {
            let ep = if conn.endpoint.is_empty() {
                match conn.kind {
                    ConnectionKind::OpenAI => "https://api.openai.com".to_string(),
                    ConnectionKind::OpenRouter => "https://openrouter.ai/api".to_string(),
                    ConnectionKind::Groq => "https://api.groq.com".to_string(),
                    ConnectionKind::Together => "https://api.together.xyz".to_string(),
                    ConnectionKind::DeepSeek => "https://api.deepseek.com".to_string(),
                    ConnectionKind::Mistral => "https://api.mistral.ai".to_string(),
                    ConnectionKind::LlamaCpp => "http://localhost:8080".to_string(),
                    _ => return None,
                }
            } else {
                conn.endpoint.clone()
            };
            Some(Arc::new(OpenAiCompatibleProvider::new(
                &ep,
                &conn.default_model,
                conn.api_key.as_deref(),
            )))
        }
        _ => {
            tracing::warn!(
                "[PROVIDER] unsupported connection kind for '{}': {:?}",
                conn.name,
                conn.kind
            );
            None
        }
    }
}

pub fn load_providers_from_connections() -> Vec<ProviderEntry> {
    let cr = pandora_types::connection_manager::ConnectionRegistry::load();
    let mut providers = Vec::new();
    for conn in cr.healthy() {
        if let Some(p) = create_provider_for(conn) {
            providers.push((p, conn.name.clone()));
        }
    }
    providers
}

pub fn require_providers() -> Result<Vec<ProviderEntry>, pandora_types::PandoraError> {
    let providers = load_providers_from_connections();
    if providers.is_empty() {
        let msg = String::from(
            "No healthy provider configured. Add one with: pandora connection add <name> <kind> <endpoint>",
        );
        return Err(pandora_types::PandoraError::governance(msg));
    }
    Ok(providers)
}
