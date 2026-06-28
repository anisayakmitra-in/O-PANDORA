use serde::{Deserialize, Serialize};

/// Role of a message participant in a chat request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}

/// A single message in a chat request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    /// Create a user message.
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    /// Create a system message.
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    /// Create an assistant message.
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

/// A chat/completion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    /// Model identifier
    pub model: String,
    /// Conversation messages
    pub messages: Vec<Message>,
    /// Sampling temperature (0.0 = deterministic)
    pub temperature: Option<f32>,
    /// Maximum tokens to generate
    pub max_tokens: Option<usize>,
    /// Whether to stream the response
    #[serde(default)]
    pub stream: bool,
}

impl ChatRequest {
    /// Create a simple single-prompt request.
    pub fn simple(model: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            messages: vec![Message::user(prompt)],
            temperature: None,
            max_tokens: None,
            stream: false,
        }
    }
}
