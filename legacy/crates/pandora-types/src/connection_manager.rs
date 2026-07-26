//! Connection Manager — unified provider abstraction.
//!
//! Every LLM backend is a Connection. Local (Ollama, llama.cpp, LM Studio, vLLM),
//! Cloud (OpenAI, Anthropic, Gemini, Groq, Together), or Enterprise (Azure, self-hosted).
//! All OpenAI-compatible APIs use the same `openai-compatible` type.
//!
//! Connections are stored in `~/.pandora/connections.toml`. API keys in
//! `~/.pandora/credentials.enc` (or OS keychain — future).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[non_exhaustive]
/// Where the provider runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ConnectionCategory {
    #[default]
    Local,
    Cloud,
    Enterprise,
}

impl ConnectionCategory {
    pub fn label(&self) -> &str {
        match self {
            Self::Local => "local",
            Self::Cloud => "cloud",
            Self::Enterprise => "enterprise",
        }
    }
}

#[non_exhaustive]
/// The protocol the provider speaks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ConnectionKind {
    Ollama,
    LlamaCpp,
    #[default]
    OpenAICompatible,
    OpenAI,
    Anthropic,
    Gemini,
    OpenRouter,
    Groq,
    Together,
    DeepSeek,
    Mistral,
    Custom,
}

impl ConnectionKind {
    pub fn label(&self) -> &str {
        match self {
            Self::Ollama => "ollama",
            Self::LlamaCpp => "llama.cpp",
            Self::OpenAICompatible => "openai-compatible",
            Self::OpenAI => "openai",
            Self::Anthropic => "anthropic",
            Self::Gemini => "gemini",
            Self::OpenRouter => "openrouter",
            Self::Groq => "groq",
            Self::Together => "together",
            Self::DeepSeek => "deepseek",
            Self::Mistral => "mistral",
            Self::Custom => "custom",
        }
    }
    pub fn is_openai_compatible(&self) -> bool {
        matches!(
            self,
            Self::OpenAICompatible
                | Self::OpenAI
                | Self::OpenRouter
                | Self::Groq
                | Self::Together
                | Self::DeepSeek
                | Self::Mistral
                | Self::Custom
        )
    }
    pub fn default_endpoint(&self) -> &str {
        match self {
            Self::Ollama => "http://127.0.0.1:11434",
            Self::LlamaCpp => "http://127.0.0.1:8080",
            Self::OpenAICompatible | Self::Custom => "http://127.0.0.1:8000/v1",
            Self::OpenAI => "https://api.openai.com/v1",
            Self::Anthropic => "https://api.anthropic.com/v1",
            Self::Gemini => "https://generativelanguage.googleapis.com/v1",
            Self::OpenRouter => "https://openrouter.ai/api/v1",
            Self::Groq => "https://api.groq.com/openai/v1",
            Self::Together => "https://api.together.xyz/v1",
            Self::DeepSeek => "https://api.deepseek.com/v1",
            Self::Mistral => "https://api.mistral.ai/v1",
        }
    }
}

/// A named connection — the primary unit of provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub meta: crate::resource::ResourceMeta,
    pub name: String,
    pub category: ConnectionCategory,
    pub kind: ConnectionKind,
    pub endpoint: String,
    pub default_model: String,
    pub api_key: Option<String>,
    pub priority: u32,
    pub tags: Vec<String>,
    pub fallback_connections: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub timeout_secs: u32,
    pub max_retries: u32,
    pub headers: HashMap<String, String>,
    pub health_status: String,
    pub latency_ms: u64,
    pub models: Vec<String>,
    pub capabilities: Vec<String>,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            meta: crate::resource::ResourceMeta::default(),
            name: "local-ollama".into(),
            category: ConnectionCategory::Local,
            kind: ConnectionKind::Ollama,
            endpoint: ConnectionKind::Ollama.default_endpoint().into(),
            default_model: String::new(),
            api_key: None,
            priority: 100,
            tags: vec!["local".into()],
            fallback_connections: vec![],
            metadata: std::collections::HashMap::new(),
            timeout_secs: 30,
            max_retries: 3,
            headers: HashMap::new(),
            health_status: "unknown".into(),
            latency_ms: 0,
            models: Vec::new(),
            capabilities: vec!["text".into()],
        }
    }
}

impl Connection {
    pub fn new(name: &str, kind: ConnectionKind, endpoint: &str) -> Self {
        let category = match kind {
            ConnectionKind::Ollama
            | ConnectionKind::LlamaCpp
            | ConnectionKind::OpenAICompatible => ConnectionCategory::Local,
            ConnectionKind::Custom => ConnectionCategory::Local,
            _ => ConnectionCategory::Cloud,
        };
        Self {
            meta: crate::resource::ResourceMeta::default(),
            name: name.into(),
            category,
            kind,
            endpoint: endpoint.into(),
            ..Default::default()
        }
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.default_model = model.into();
        self
    }
    pub fn with_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.into());
        self
    }
    pub fn with_tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn with_api_key(mut self, key: &str) -> Self {
        self.api_key = Some(key.to_string());
        self
    }
    pub fn with_category(mut self, cat: ConnectionCategory) -> Self {
        self.category = cat;
        self
    }
    pub fn supports_chat(&self) -> bool {
        self.capabilities.contains(&"chat".into()) || !self.models.is_empty()
    }
    pub fn with_fallback(mut self, id: &str) -> Self {
        self.fallback_connections.push(id.into());
        self
    }
    pub fn with_metadata(mut self, k: &str, v: &str) -> Self {
        self.metadata.insert(k.into(), v.into());
        self
    }

    pub fn is_healthy(&self) -> bool {
        self.health_status == "online"
    }

    /// Test the connection — send a minimal request to the endpoint.
    pub fn test(&mut self) -> Result<(), String> {
        use std::time::Instant;
        let start = Instant::now();
        let url = if self.kind == ConnectionKind::Ollama {
            format!("{}/api/tags", self.endpoint.trim_end_matches('/'))
        } else {
            format!("{}/models", self.endpoint.trim_end_matches('/'))
        };
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .map_err(|e| format!("client: {e}"))?;
        let mut req = client.get(&url);
        if let Some(key) = &self.api_key {
            req = req.header("Authorization", format!("Bearer {key}"));
        }
        for (k, v) in &self.headers {
            req = req.header(k.as_str(), v.as_str());
        }
        let resp = req.send().map_err(|e| format!("unreachable: {e}"))?;
        self.latency_ms = start.elapsed().as_millis() as u64;
        self.health_status = if resp.status().is_success() {
            "online".into()
        } else {
            format!("{}", resp.status())
        };
        // Parse models from response
        if let Ok(json) = resp.json::<serde_json::Value>() {
            if let Some(models) = json.get("models").or_else(|| json.get("data")) {
                if let Some(arr) = models.as_array() {
                    self.models = arr
                        .iter()
                        .filter_map(|m| m.get("id").and_then(|v| v.as_str()).map(String::from))
                        .collect();
                }
            }
        }
        Ok(())
    }
}

/// The connection registry — loads/stores connections from disk.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionRegistry {
    pub connections: Vec<Connection>,
    pub default_connection: Option<String>,
}

impl ConnectionRegistry {
    pub fn load() -> Self {
        let dir = std::env::var("HOME")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("/tmp"))
            .join(".pandora");
        let path = dir.join("connections.toml");
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let dir = std::env::var("HOME")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("/tmp"))
            .join(".pandora");
        std::fs::create_dir_all(&dir).map_err(|e| format!("mkdir: {e}"))?;
        let path = dir.join("connections.toml");
        let toml = toml::to_string_pretty(self).map_err(|e| format!("serialize: {e}"))?;
        std::fs::write(&path, toml).map_err(|e| format!("write: {e}"))
    }

    pub fn add(&mut self, conn: Connection) -> Result<(), String> {
        if self.connections.iter().any(|c| c.name == conn.name) {
            return Err(format!(
                "Connection '{}' already exists. Remove it first.",
                conn.name
            ));
        }
        self.connections.push(conn);
        self.save()
    }

    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        let len = self.connections.len();
        self.connections.retain(|c| c.name != name);
        if self.connections.len() == len {
            return Err(format!("Connection '{}' not found", name));
        }
        self.save()
    }

    pub fn find(&self, name: &str) -> Option<&Connection> {
        self.connections.iter().find(|c| c.name == name)
    }
    pub fn find_mut(&mut self, name: &str) -> Option<&mut Connection> {
        self.connections.iter_mut().find(|c| c.name == name)
    }

    pub fn list(&self) -> Vec<&Connection> {
        self.connections.iter().collect()
    }
    pub fn healthy(&self) -> Vec<&Connection> {
        self.connections.iter().filter(|c| c.is_healthy()).collect()
    }

    pub fn by_category(&self, cat: ConnectionCategory) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|c| c.category == cat)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ollama_is_local() {
        let c = Connection::new("ollama", ConnectionKind::Ollama, "http://localhost:11434");
        assert_eq!(c.category, ConnectionCategory::Local);
    }

    #[test]
    fn openai_compatible_unifies_many() {
        assert!(ConnectionKind::OpenAICompatible.is_openai_compatible());
        assert!(ConnectionKind::OpenAI.is_openai_compatible());
        assert!(ConnectionKind::Groq.is_openai_compatible());
        assert!(ConnectionKind::OpenRouter.is_openai_compatible());
        assert!(!ConnectionKind::Ollama.is_openai_compatible());
    }

    #[test]
    fn registry_add_remove() {
        let mut reg = ConnectionRegistry::default();
        reg.add(Connection::new(
            "test",
            ConnectionKind::OpenAICompatible,
            "http://localhost:8000/v1",
        ))
        .unwrap();
        assert!(reg.find("test").is_some());
        reg.remove("test").unwrap();
        assert!(reg.find("test").is_none());
    }
}
