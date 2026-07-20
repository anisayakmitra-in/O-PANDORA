//! Runtime Resource — the universal interface for all Pandora runtime objects.
//!
//! Every runtime resource (harness, gene, connection, plan, artifact, skill)
//! implements this trait. The runtime treats them uniformly: inspect, monitor,
//! govern, and schedule them through the same interface.
//!
//! Design: modeled after Kubernetes' ObjectMeta + conditions pattern.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Every runtime resource has this metadata block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMeta {
    pub id: String,
    pub namespace: String,
    pub version: String,
    pub kind: ResourceKind,
    pub owner: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
}

impl Default for ResourceMeta {
    fn default() -> Self {
        Self {
            id: String::new(),
            namespace: String::new(),
            version: String::new(),
            kind: ResourceKind::Unknown,
            owner: None,
            created_at: SystemTime::UNIX_EPOCH,
            updated_at: SystemTime::UNIX_EPOCH,
            labels: HashMap::new(),
            annotations: HashMap::new(),
        }
    }
}

/// What kind of resource this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ResourceKind {
    #[default]
    Unknown,
    Connection,
    Harness,
    Gene,
    Skill,
    Plan,
    Artifact,
    Session,
    Policy,
    Provider,
    Worker,
    Package,
}

/// The health of a runtime resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceHealth {
    pub status: HealthStatus,
    pub message: Option<String>,
    pub last_check: SystemTime,
    pub conditions: Vec<HealthCondition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Unknown,
    Healthy,
    Degraded,
    Unhealthy,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCondition {
    pub r#type: String,
    pub status: HealthStatus,
    pub reason: String,
    pub message: String,
}

/// Lineage tracking — where did this resource come from?
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceLineage {
    pub parent_id: Option<String>,
    pub source_package: Option<String>,
    pub signature: Option<String>,
    pub hash: Option<String>,
}

/// Capabilities a resource provides.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapabilitySet {
    pub provides: Vec<String>,
    pub requires: Vec<String>,
    pub features: HashMap<String, String>,
}

/// Every runtime resource must implement this.
pub trait RuntimeResource: Send + Sync {
    fn meta(&self) -> &ResourceMeta;
    fn health(&self) -> &ResourceHealth;
    fn lineage(&self) -> &ResourceLineage;
    fn capabilities(&self) -> &CapabilitySet;

    /// Check if this resource is healthy.
    fn is_healthy(&self) -> bool {
        matches!(self.health().status, HealthStatus::Healthy)
    }

    /// Resources this one depends on.
    fn dependencies(&self) -> Vec<String> {
        vec![]
    }

    /// A human-readable summary for the observatory.
    fn summary(&self) -> String {
        format!(
            "{} v{} ({:?})",
            self.meta().id,
            self.meta().version,
            self.meta().kind
        )
    }
}

/// Implements RuntimeResource for types that embed resource metadata.
#[macro_export]
macro_rules! impl_resource {
    ($ty:ty) => {
        impl $crate::resource::RuntimeResource for $ty {
            fn meta(&self) -> &$crate::resource::ResourceMeta {
                &self.meta
            }
            fn health(&self) -> &$crate::resource::ResourceHealth {
                &self.health
            }
            fn lineage(&self) -> &$crate::resource::ResourceLineage {
                &self.lineage
            }
            fn capabilities(&self) -> &$crate::resource::CapabilitySet {
                &self.capabilities
            }
        }
    };
}
