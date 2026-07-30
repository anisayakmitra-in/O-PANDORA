//! Versioned wire contracts shared by Pandora clients and runtime servers.

use serde::{Deserialize, Serialize};

pub const API_VERSION: &str = "v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairRequest {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairResponse {
    pub api_version: String,
    pub token: String,
    pub expires_in_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeRequest {
    pub token: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub task: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub strategy: String,
    #[serde(default)]
    pub evaluator: String,
    #[serde(default)]
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResponse {
    pub api_version: String,
    pub session_id: String,
    pub status: String,
    pub output: String,
    pub duration_ms: u64,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RuntimeEvent {
    Started {
        session_id: String,
        task: String,
    },
    Output {
        session_id: String,
        chunk: String,
    },
    ToolCall {
        session_id: String,
        tool: String,
    },
    ApprovalRequired {
        session_id: String,
        capability: String,
    },
    Completed {
        session_id: String,
        success: bool,
    },
    Failed {
        session_id: String,
        error: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub api_version: String,
    pub sequence: u64,
    pub event: RuntimeEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub api_version: String,
    pub node_id: String,
    pub name: String,
    pub version: String,
    pub platform: String,
    pub architecture: String,
    pub capabilities: Vec<String>,
    pub auth_required: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub api_version: String,
    pub status: String,
    pub runtime: String,
    pub version: String,
}
