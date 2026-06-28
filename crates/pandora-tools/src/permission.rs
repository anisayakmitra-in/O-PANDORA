use serde::{Deserialize, Serialize};

/// A single permission declaration attached to a tool.
///
/// Permissions are open-ended (id + opaque params) so the contract
/// can stay decoupled from the actual governance or leasing system
/// that will evaluate them.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolPermission {
    /// Permission identifier (e.g., `fs.read.path`, `net.egress.host`).
    pub id: String,

    /// Free-form parameters describing the scope of the permission
    /// (e.g. `{"path_prefix": "/data/"}`).
    #[serde(default)]
    pub params: serde_json::Value,
}

impl ToolPermission {
    /// Create a permission with no extra parameters.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            params: serde_json::Value::Null,
        }
    }

    /// Attach parameters to the permission.
    pub fn with_params(mut self, params: serde_json::Value) -> Self {
        self.params = params;
        self
    }
}

/// A collection of permission declarations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct ToolPermissionSet {
    permissions: Vec<ToolPermission>,
}

impl ToolPermissionSet {
    /// Empty permission set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a permission declaration.
    pub fn with(mut self, perm: ToolPermission) -> Self {
        self.permissions.push(perm);
        self
    }

    /// All declared permissions.
    pub fn all(&self) -> &[ToolPermission] {
        &self.permissions
    }

    /// Whether a permission with the given id is declared.
    pub fn declares(&self, id: &str) -> bool {
        self.permissions.iter().any(|p| p.id == id)
    }
}
