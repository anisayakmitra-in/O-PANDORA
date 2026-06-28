use serde::{Deserialize, Serialize};

/// Raw, opaque input passed to a tool.
///
/// The tool is responsible for interpreting the value (typically via
/// `serde_json::from_value` against its declared schema). Keeping the
/// input as a `serde_json::Value` allows tools to accept arbitrary
/// structured payloads without the contract crate needing to know
/// about their schemas.
pub type ToolInput = serde_json::Value;

/// Raw, opaque output produced by a tool.
pub type ToolOutput = serde_json::Value;

/// Free-form metadata associated with a tool, a tool call, or a
/// tool result.
///
/// Concrete keys are tool-defined. Reserved keys (none today) may be
/// added in the future.
pub type ToolMetadata = std::collections::BTreeMap<String, serde_json::Value>;

/// Identifier for a registered tool.
pub type ToolId = String;

/// Versioned identifier for a tool, `name@version`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolVersion {
    /// Tool name.
    pub name: String,

    /// Semantic version string.
    pub version: String,
}

impl ToolVersion {
    /// Build a `name@version` string.
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }

    /// Render as `name@version`.
    pub fn as_string(&self) -> String {
        format!("{}@{}", self.name, self.version)
    }
}
