//! Pandora Permission — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RuntimePermission {
    MemoryRead,

    MemoryWrite,

    ToolExecution,

    ShellAccess,

    NetworkAccess,

    MutationAccess,

    EvolutionAccess,

    SandboxControl,

    GovernanceOverride,

    TelemetryAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenePermissionProfile {
    pub gene_id: String,

    pub granted: Vec<RuntimePermission>,
}

pub struct PermissionValidator;

impl PermissionValidator {
    pub fn has_permission(profile: &GenePermissionProfile, permission: RuntimePermission) -> bool {
        profile.granted.contains(&permission)
    }
}
