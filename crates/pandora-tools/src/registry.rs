use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::error::{Result, ToolError};
use crate::manifest::ToolManifest;
use crate::traits::Tool;

/// Thread-safe, async-aware registry of [`Tool`] implementations.
///
/// The registry is intentionally a thin container. Capability
/// resolution, permission checking, and source harness routing are
/// the responsibility of higher layers (governance, harnesses,
/// KUBER Palace), not the contract crate.
pub struct ToolRegistry {
    tools: Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a tool. The key is the tool's `manifest().id`.
    /// Returns [`ToolError::ExecutionFailed`] if a tool with the same
    /// id is already registered.
    pub async fn register(&self, tool: Arc<dyn Tool>) -> Result<()> {
        let id = tool.manifest().id;
        let mut tools = self.tools.write().await;
        if tools.contains_key(&id) {
            return Err(ToolError::ExecutionFailed(format!(
                "tool '{id}' is already registered"
            )));
        }
        tools.insert(id, tool);
        Ok(())
    }

    /// Unregister a tool by id. Returns true if a tool was removed.
    pub async fn unregister(&self, id: &str) -> bool {
        let mut tools = self.tools.write().await;
        tools.remove(id).is_some()
    }

    /// Check whether a tool is registered.
    pub async fn contains(&self, id: &str) -> bool {
        let tools = self.tools.read().await;
        tools.contains_key(id)
    }

    /// Look up a tool by id.
    pub async fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        let tools = self.tools.read().await;
        tools.get(id).cloned()
    }

    /// Get a tool by id, or return [`ToolError::NotFound`].
    pub async fn require(&self, id: &str) -> Result<Arc<dyn Tool>> {
        self.get(id)
            .await
            .ok_or_else(|| ToolError::NotFound(id.to_string()))
    }

    /// Number of registered tools.
    pub async fn len(&self) -> usize {
        let tools = self.tools.read().await;
        tools.len()
    }

    /// Whether the registry is empty.
    pub async fn is_empty(&self) -> bool {
        let tools = self.tools.read().await;
        tools.is_empty()
    }

    /// List all registered tool ids.
    pub async fn list_ids(&self) -> Vec<String> {
        let tools = self.tools.read().await;
        tools.keys().cloned().collect()
    }

    /// List manifests for every registered tool.
    pub async fn list_manifests(&self) -> Vec<ToolManifest> {
        let tools = self.tools.read().await;
        let mut manifests: Vec<ToolManifest> = tools.values().map(|t| t.manifest()).collect();
        manifests.sort_by(|a, b| a.id.cmp(&b.id));
        manifests
    }
}
