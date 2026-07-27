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
    fn generate(&self, request: GenerationRequest) -> Result<String, crate::PandoraError>;
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
    ) -> Result<ChatCompletion, crate::PandoraError> {
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

    /// Stream a response from the LLM, calling the callback for each chunk.
    /// Default implementation: call generate() and emit one chunk.
    fn generate_stream(
        &self,
        request: GenerationRequest,
        callback: &StreamCallback,
    ) -> Result<String, crate::PandoraError> {
        let text = self.generate(request)?;
        callback(StreamChunk {
            text: text.clone(),
            tool_calls: vec![],
            done: true,
        });
        Ok(text)
    }

    /// Whether this provider supports streaming responses.
    fn supports_streaming(&self) -> bool {
        false
    }
}

// ── Streaming types ──

/// A chunk of a streaming response from the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    pub done: bool,
}

/// Callback for streaming responses.
pub type StreamCallback = Box<dyn Fn(StreamChunk) + Send + Sync>;

// ── Ollama provider ──

pub mod ollama {
    use super::*;

    pub struct OllamaProvider {
        pub endpoint: String,
        pub model: String,
        client: reqwest::blocking::Client,
    }

    impl OllamaProvider {
        pub fn new(endpoint: &str, model: &str) -> Self {
            Self {
                endpoint: endpoint.to_string(),
                model: model.to_string(),
                client: reqwest::blocking::Client::new(),
            }
        }
        pub fn new_default() -> Self {
            Self {
                endpoint: "http://localhost:11434".into(),
                model: std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "".into()),
                client: reqwest::blocking::Client::new(),
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
        fn generate(&self, request: GenerationRequest) -> Result<String, crate::PandoraError> {
            let url = format!("{}/api/generate", self.endpoint);
            let body = serde_json::json!({
                "model": self.model, "prompt": request.prompt,
                "options": { "temperature": request.temperature, "num_predict": request.max_tokens },
                "stream": false
            });
            let resp = self
                .client
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

        fn supports_streaming(&self) -> bool {
            true
        }

        fn generate_stream(
            &self,
            request: GenerationRequest,
            callback: &StreamCallback,
        ) -> Result<String, crate::PandoraError> {
            let url = format!("{}/api/generate", self.endpoint);
            let body = serde_json::json!({
                "model": self.model, "prompt": request.prompt,
                "options": { "temperature": request.temperature, "num_predict": request.max_tokens },
                "stream": true
            });
            let resp = self
                .client
                .post(&url)
                .json(&body)
                .send()
                .map_err(|e| format!("stream req failed: {e}"))?;

            // Read line by line (Ollama streams NDJSON)
            let mut full_text = String::new();
            for line in resp.text().unwrap_or_default().lines() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                    if let Some(chunk) = json["response"].as_str() {
                        if !chunk.is_empty() {
                            full_text.push_str(chunk);
                            callback(StreamChunk {
                                text: chunk.to_string(),
                                tool_calls: vec![],
                                done: false,
                            });
                        }
                    }
                    if json["done"].as_bool() == Some(true) {
                        callback(StreamChunk {
                            text: String::new(),
                            tool_calls: vec![],
                            done: true,
                        });
                    }
                }
            }
            Ok(full_text)
        }

        fn generate_with_tools(
            &self,
            request: GenerationRequest,
            tools: &[ToolDefinition],
            messages: &[ChatMessage],
        ) -> Result<ChatCompletion, crate::PandoraError> {
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

            let resp = self
                .client
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
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

/// OpenAI-compatible provider — speaks the /v1/chat/completions API.
///
/// Works with any service that exposes OpenAI's chat completions format:
/// OpenAI, OpenRouter, Groq, Together, DeepSeek, Mistral, llama.cpp server,
/// LM Studio, vLLM, Ollama (via /v1/), and custom endpoints.
pub mod openai_compat {
    use super::*;

    pub struct OpenAiCompatibleProvider {
        pub endpoint: String,
        pub model: String,
        pub api_key: Option<String>,
        client: reqwest::blocking::Client,
    }

    impl OpenAiCompatibleProvider {
        pub fn new(endpoint: &str, model: &str, api_key: Option<&str>) -> Self {
            let mut headers = reqwest::header::HeaderMap::new();
            if let Some(key) = api_key {
                if !key.is_empty() {
                    let auth = reqwest::header::HeaderValue::from_str(
                        &format!("Bearer {key}"),
                    )
                    .unwrap_or_else(|_| reqwest::header::HeaderValue::from_static(""));
                    headers.insert(reqwest::header::AUTHORIZATION, auth);
                }
            }

            let client = reqwest::blocking::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap_or_else(|_| reqwest::blocking::Client::new());

            Self {
                endpoint: endpoint.trim_end_matches('/').to_string(),
                model: model.to_string(),
                api_key: api_key.map(|s| s.to_string()),
                client,
            }
        }

        fn chat_url(&self) -> String {
            // Most providers support /v1/chat/completions.
            // If the endpoint already has /v1, use it as-is.
            if self.endpoint.ends_with("/v1") {
                format!("{}/chat/completions", self.endpoint)
            } else if self.endpoint.contains("/v1") {
                format!("{}/chat/completions", self.endpoint)
            } else {
                format!("{}/v1/chat/completions", self.endpoint)
            }
        }
    }

    impl Provider for OpenAiCompatibleProvider {
        fn name(&self) -> &str {
            "openai-compatible"
        }

        fn manifest(&self) -> ProviderManifest {
            ProviderManifest {
                name: "openai-compatible".into(),
                endpoint: self.endpoint.clone(),
                models: vec![self.model.clone()],
                capabilities: vec!["text".into(), "tools".into()],
                locality: "cloud".into(),
            }
        }

        fn generate(&self, request: GenerationRequest) -> Result<String, crate::PandoraError> {
            let url = self.chat_url();
            let model = if request.model.is_empty() {
                &self.model
            } else {
                &request.model
            };

            let body = serde_json::json!({
                "model": model,
                "messages": [
                    { "role": "user", "content": request.prompt }
                ],
                "temperature": request.temperature,
                "max_tokens": request.max_tokens,
                "stream": false,
            });

            let resp = self
                .client
                .post(&url)
                .json(&body)
                .send()
                .map_err(|e| crate::PandoraError::provider(format!("HTTP error: {e}")))?;

            let json: serde_json::Value = resp
                .json()
                .map_err(|e| crate::PandoraError::provider(format!("JSON parse error: {e}")))?;

            // Extract content from OpenAI format
            let content = json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("");
            Ok(content.to_string())
        }

        fn supports_tools(&self) -> bool {
            true
        }

        fn generate_with_tools(
            &self,
            request: GenerationRequest,
            tools: &[ToolDefinition],
            messages: &[ChatMessage],
        ) -> Result<ChatCompletion, crate::PandoraError> {
            let url = self.chat_url();
            let model = if request.model.is_empty() {
                &self.model
            } else {
                &request.model
            };

            // Build API messages from chat history
            let mut api_messages: Vec<serde_json::Value> = Vec::new();
            for msg in messages {
                let role = &msg.role;
                let content = &msg.content;

                let mut api_msg = serde_json::json!({
                    "role": role,
                    "content": content,
                });

                if !msg.tool_calls.is_empty() {
                    api_msg = serde_json::json!({
                        "role": "assistant",
                        "content": content,
                        "tool_calls": msg.tool_calls.iter().map(|tc| {
                            serde_json::json!({
                                "id": tc.id,
                                "type": "function",
                                "function": {
                                    "name": tc.name,
                                    "arguments": tc.arguments,
                                },
                            })
                        }).collect::<Vec<_>>(),
                    });
                }

                if let Some(ref tci) = msg.tool_call_id {
                    api_msg = serde_json::json!({
                        "role": "tool",
                        "tool_call_id": tci,
                        "content": content,
                    });
                }

                api_messages.push(api_msg);
            }

            // Build tool definitions
            let api_tools: Vec<serde_json::Value> = tools
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.parameters,
                        },
                    })
                })
                .collect();

            let mut body = serde_json::json!({
                "model": model,
                "messages": api_messages,
                "tools": api_tools,
                "temperature": request.temperature,
                "max_tokens": request.max_tokens,
            });

            // Remove tools if empty (some APIs reject empty tools array)
            if tools.is_empty() {
                body.as_object_mut().map(|m| m.remove("tools"));
            }

            let resp = self
                .client
                .post(&url)
                .json(&body)
                .send()
                .map_err(|e| crate::PandoraError::provider(format!("HTTP error: {e}")))?;

            let status = resp.status();
            let json: serde_json::Value = resp
                .json()
                .map_err(|e| crate::PandoraError::provider(format!("JSON parse error ({status}): {e}")))?;

            // Check for API errors
            if let Some(err) = json["error"]["message"].as_str() {
                return Err(crate::PandoraError::provider(format!("API error: {err}")));
            }

            let choice = &json["choices"][0];
            let finish_reason = choice["finish_reason"]
                .as_str()
                .unwrap_or("stop")
                .to_string();
            let content = choice["message"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();

            // Extract tool calls if present
            let mut tool_calls = Vec::new();
            if let Some(tcs) = choice["message"]["tool_calls"].as_array() {
                for tc in tcs {
                    tool_calls.push(ToolCall {
                        id: tc["id"].as_str().unwrap_or("").to_string(),
                        name: tc["function"]["name"].as_str().unwrap_or("").to_string(),
                        arguments: tc["function"]["arguments"].as_str().unwrap_or("{}").to_string(),
                    });
                }
            }

            let tokens_used = json["usage"]["total_tokens"].as_u64().unwrap_or(0) as usize;

            Ok(ChatCompletion {
                text: content,
                tool_calls,
                finish_reason,
                tokens_used,
            })
        }
    }
}
