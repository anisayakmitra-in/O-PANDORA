//! Sandbox Orchestrator.
//!
//! Phoenix never executes directly. Everything
//! executes through a SandboxBackend. This orchestrator
//! selects, leases, and manages sandbox backends.

use serde::{Deserialize, Serialize};

/// Supported sandbox backend kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SandboxKind {
    #[default]
    Native,
    Docker,
    Firecracker,
    Wasmtime,
    Browser,
    Remote,
    Ssh,
    Vm,
    Kubernetes,
    Gpu,
    Tee,
    Custom,
}

/// A sandbox lease granted by the orchestrator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SandboxLease {
    pub lease_id: String,
    pub kind: SandboxKind,
    pub granted_at_ms: u64,
    pub expires_at_ms: u64,
}

/// Orchestrates sandbox backend selection and leasing.
pub struct SandboxOrchestrator {
    default_kind: SandboxKind,
}

impl SandboxOrchestrator {
    pub fn new(default_kind: SandboxKind) -> Self {
        SandboxOrchestrator { default_kind }
    }

    /// Select a sandbox for the given request.
    pub fn select(&self, _requested: Option<SandboxKind>) -> SandboxKind {
        _requested.unwrap_or(self.default_kind)
    }

    /// Lease a sandbox.
    pub fn lease(&self, kind: SandboxKind, duration_ms: u64) -> SandboxLease {
        SandboxLease {
            lease_id: format!("sb-{:?}-{}", kind, duration_ms),
            kind,
            granted_at_ms: 0,
            expires_at_ms: duration_ms,
        }
    }
}

impl Default for SandboxOrchestrator {
    fn default() -> Self {
        Self::new(SandboxKind::Native)
    }
}
