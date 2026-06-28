use serde::{Deserialize, Serialize};

/// Declarative capabilities a tool requires or advertises.
///
/// The set of capabilities is intentionally open so that future
/// systems (Capability Leasing, Source Harnesses, Meta Harnesses,
/// Extension Harnesses, KUBER Palace, WASM, MCP, etc.) can extend
/// it without breaking the contract crate.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ToolCapability {
    /// Capability identifier (e.g., `filesystem.read`, `network.http`).
    pub id: String,

    /// Whether the tool *requires* this capability (true) or merely
    /// *advertises* it (false). Tools that require a capability should
    /// fail at registration time if the host environment cannot grant
    /// it.
    pub required: bool,
}

impl ToolCapability {
    /// Declare a required capability.
    pub fn required(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            required: true,
        }
    }

    /// Declare an advertised (optional) capability.
    pub fn optional(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            required: false,
        }
    }
}

/// A set of capability declarations, deduplicated by `id`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ToolCapabilitySet {
    capabilities: Vec<ToolCapability>,
}

impl ToolCapabilitySet {
    /// Empty capability set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a capability. If a capability with the same `id` already
    /// exists and is marked required, it stays required.
    pub fn with(mut self, cap: ToolCapability) -> Self {
        if let Some(existing) = self.capabilities.iter_mut().find(|c| c.id == cap.id) {
            if cap.required {
                existing.required = true;
            }
        } else {
            self.capabilities.push(cap);
        }
        self
    }

    /// All declared capabilities.
    pub fn all(&self) -> &[ToolCapability] {
        &self.capabilities
    }

    /// Whether the tool requires the given capability.
    pub fn requires(&self, id: &str) -> bool {
        self.capabilities.iter().any(|c| c.required && c.id == id)
    }
}
