//! Provider types — LLM provider abstractions.
//!
//! Every provider backend (Ollama, OpenAI, llama.cpp) implements the
//! `Provider` trait. The orchestrator selects providers and dispatches
//! generation requests through this interface.

use serde::{Deserialize, Serialize};

// ── Core types ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    pub prompt: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: usize,
    pub system: Option<String>,
    pub top_p: f32,
}

impl Default for GenerationRequest {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            model: String::new(),
            temperature: 0.3,
            max_tokens: 4096,
            system: None,
            top_p: 0.9,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    pub text: String,
    pub tokens_used: usize,
    pub model: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderManifest {
    pub name: String,
    pub endpoint: String,
    pub models: Vec<String>,
    pub capabilities: Vec<String>,
    pub locality: String,
}

impl Default for ProviderManifest {
    fn default() -> Self {
        Self {
            name: "ollama".into(),
            endpoint: "http://localhost:11434".into(),
            models: vec![std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "".into())],
            capabilities: vec!["text".into()],
            locality: "local".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTarget {
    pub provider: String,
    pub model: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub locality: String,
}

/// The Provider trait — any LLM backend implements this.
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn generate(&self, request: GenerationRequest) -> Result<String, String>;
    fn manifest(&self) -> ProviderManifest {
        ProviderManifest::default()
    }

    /// Generate a response with tool definitions available to the LLM.
    /// The LLM can respond with text and/or tool_calls.
    /// Default implementation falls back to plain generate() (no tool use).
    fn generate_with_tools(
        &self,
        request: GenerationRequest,
        _tools: &[ToolDefinition],
        _messages: &[ChatMessage],
    ) -> Result<ChatCompletion, String> {
        // Default: no tool support, just call generate and wrap the result.
        let text = self.generate(request)?;
        Ok(ChatCompletion {
            text,
            tool_calls: vec![],
            finish_reason: "stop".into(),
            tokens_used: 0,
        })
    }

    /// Whether this provider supports function/tool calling.
    fn supports_tools(&self) -> bool {
        false
    }
}

// ── Ollama provider ──

pub mod ollama {
    use super::*;

    pub struct OllamaProvider {
        pub endpoint: String,
        pub model: String,
    }

    impl OllamaProvider {
        pub fn new(endpoint: &str, model: &str) -> Self {
            Self {
                endpoint: endpoint.to_string(),
                model: model.to_string(),
            }
        }
        pub fn new_default() -> Self {
            Self {
                endpoint: "http://localhost:11434".into(),
                model: std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "".into()),
            }
        }
    }

    impl Provider for OllamaProvider {
        fn name(&self) -> &str {
            "ollama"
        }
        fn manifest(&self) -> ProviderManifest {
            ProviderManifest {
                name: "ollama".into(),
                endpoint: self.endpoint.clone(),
                models: vec![self.model.clone()],
                capabilities: vec!["text".into()],
                locality: "local".into(),
            }
        }
        fn generate(&self, request: GenerationRequest) -> Result<String, String> {
            let url = format!("{}/api/generate", self.endpoint);
            let body = serde_json::json!({
                "model": self.model, "prompt": request.prompt,
                "options": { "temperature": request.temperature, "num_predict": request.max_tokens },
                "stream": false
            });
            let client = reqwest::blocking::Client::new();
            let resp = client
                .post(&url)
                .json(&body)
                .send()
                .map_err(|e| format!("req failed: {e}"))?;
            let json: serde_json::Value = resp.json().map_err(|e| format!("parse failed: {e}"))?;
            Ok(json["response"].as_str().unwrap_or("").to_string())
        }

        fn supports_tools(&self) -> bool {
            true
        }

        fn generate_with_tools(
            &self,
            request: GenerationRequest,
            tools: &[ToolDefinition],
            messages: &[ChatMessage],
        ) -> Result<ChatCompletion, String> {
            let url = format!("{}/api/chat", self.endpoint);

            // Convert messages to Ollama chat format
            let chat_messages: Vec<serde_json::Value> = messages
                .iter()
                .map(|m| {
                    let mut msg = serde_json::json!({
                        "role": m.role,
                        "content": m.content,
                    });
                    if !m.tool_calls.is_empty() {
                        let tool_calls: Vec<serde_json::Value> = m
                            .tool_calls
                            .iter()
                            .map(|tc| {
                                serde_json::json!({
                                    "function": {
                                        "name": tc.name,
                                        "arguments": tc.arguments,
                                    }
                                })
                            })
                            .collect();
                        msg["tool_calls"] = serde_json::Value::Array(tool_calls);
                    }
                    if let Some(ref id) = m.tool_call_id {
                        msg["tool_call_id"] = serde_json::Value::String(id.clone());
                    }
                    msg
                })
                .collect();

            // Convert tool definitions to Ollama function format
            let tools_json: Vec<serde_json::Value> = tools
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.parameters,
                        }
                    })
                })
                .collect();

            let body = serde_json::json!({
                "model": self.model,
                "messages": chat_messages,
                "tools": tools_json,
                "options": {
                    "temperature": request.temperature,
                    "num_predict": request.max_tokens,
                },
                "stream": false
            });

            let client = reqwest::blocking::Client::new();
            let resp = client
                .post(&url)
                .json(&body)
                .send()
                .map_err(|e| format!("tool req failed: {e}"))?;

            let json: serde_json::Value =
                resp.json().map_err(|e| format!("tool parse failed: {e}"))?;

            // Extract text content
            let text = json["message"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();

            // Extract tool calls from response
            let tool_calls: Vec<ToolCall> = json["message"]["tool_calls"]
                .as_array()
                .map(|calls| {
                    calls
                        .iter()
                        .enumerate()
                        .map(|(i, tc)| {
                            let name = tc["function"]["name"]
                                .as_str()
                                .unwrap_or("unknown")
                                .to_string();
                            let args = tc["function"]["arguments"]
                                .as_str()
                                .unwrap_or("{}")
                                .to_string();
                            ToolCall {
                                id: format!("call-{i}"),
                                name,
                                arguments: args,
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();

            let finish_reason = if !tool_calls.is_empty() {
                "tool_calls"
            } else if text.is_empty() {
                "length"
            } else {
                "stop"
            };

            Ok(ChatCompletion {
                text,
                tool_calls,
                finish_reason: finish_reason.into(),
                tokens_used: json["eval_count"].as_u64().unwrap_or(0) as usize,
            })
        }
    }
}

// ── Tool / Function calling types ──

/// A tool definition sent to the LLM so it knows what genes are available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// A tool call requested by the LLM during the agentic loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

/// A message in the agentic conversation loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String, // "system" | "user" | "assistant" | "tool"
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
    pub tool_call_id: Option<String>,
}

/// Response from the LLM in the agentic loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletion {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    pub finish_reason: String, // "stop" | "tool_calls" | "length"
    pub tokens_used: usize,
}

/// Simple cancellation token — replaces tokio_util::sync::CancellationToken.
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: Arc<Mutex<bool>>,
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(Mutex::new(false)),
        }
    }
    pub fn cancel(&self) {
        *self.cancelled.lock().expect("cancel lock") = true;
    }
    pub fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().expect("cancel lock")
    }
}
