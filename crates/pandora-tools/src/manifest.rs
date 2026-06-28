use serde::{Deserialize, Serialize};

use crate::capability::ToolCapabilitySet;
use crate::permission::ToolPermissionSet;
use crate::types::ToolVersion;

/// Metadata describing a tool to the rest of the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolManifest {
    /// Tool identifier (e.g., `fs.read`).
    pub id: String,

    /// Human-readable tool name.
    pub name: String,

    /// Tool version.
    pub version: String,

    /// Short description of what the tool does.
    #[serde(default)]
    pub description: String,

    /// JSON Schema describing the accepted input.
    #[serde(default)]
    pub input_schema: serde_json::Value,

    /// JSON Schema describing the produced output.
    #[serde(default)]
    pub output_schema: serde_json::Value,

    /// Tool execution mode.
    #[serde(default)]
    pub mode: ToolMode,

    /// Capabilities required or advertised by the tool.
    #[serde(default)]
    pub capabilities: ToolCapabilitySet,

    /// Permissions the tool requires at runtime.
    #[serde(default)]
    pub permissions: ToolPermissionSet,
}

/// Where a tool can be executed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ToolMode {
    /// Runs in-process inside the host (default). The simplest mode;
    /// trust is delegated to the host's process sandbox.
    #[default]
    Native,

    /// Runs out-of-process or in a sandbox. The host does not
    /// directly load the tool's code.
    Sandboxed,

    /// Runs as a remote service (e.g. MCP over the network).
    Remote,
}

impl ToolManifest {
    /// Create a minimal manifest for the given `id`, `name`, `version`.
    pub fn new(id: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            description: String::new(),
            input_schema: serde_json::Value::Null,
            output_schema: serde_json::Value::Null,
            mode: ToolMode::default(),
            capabilities: ToolCapabilitySet::new(),
            permissions: ToolPermissionSet::new(),
        }
    }

    /// Set the human-readable description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set the JSON Schema for the tool's input.
    pub fn with_input_schema(mut self, schema: serde_json::Value) -> Self {
        self.input_schema = schema;
        self
    }

    /// Set the JSON Schema for the tool's output.
    pub fn with_output_schema(mut self, schema: serde_json::Value) -> Self {
        self.output_schema = schema;
        self
    }

    /// Set the tool execution mode.
    pub fn with_mode(mut self, mode: ToolMode) -> Self {
        self.mode = mode;
        self
    }

    /// Set the tool's capability declarations.
    pub fn with_capabilities(mut self, capabilities: ToolCapabilitySet) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Set the tool's permission declarations.
    pub fn with_permissions(mut self, permissions: ToolPermissionSet) -> Self {
        self.permissions = permissions;
        self
    }

    /// Render as `id@version`.
    pub fn versioned_id(&self) -> ToolVersion {
        ToolVersion::new(self.id.clone(), self.version.clone())
    }
}
