//! ExecutionTarget — resolved provider + model for execution.
//! Parliament defines policies; Capability Resolution produces an ExecutionTarget.

use crate::capability::ModelCapabilities;
use serde::{Deserialize, Serialize};

/// A fully resolved execution target. The orchestrator just executes this.
/// No hardcoded provider names or models — Parliament policies + config determine this.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTarget {
    pub provider: String,
    pub model: String,
    pub endpoint: Option<String>,
    pub capabilities: ModelCapabilities,
    pub locality: Locality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Locality {
    Local,
    Remote,
    Any,
}

impl Default for ExecutionTarget {
    fn default() -> Self {
        Self {
            provider: String::new(),
            model: String::new(),
            endpoint: None,
            capabilities: ModelCapabilities::default(),
            locality: Locality::Any,
        }
    }
}

/// Execution policy — how the Execution Service selects providers/models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPolicy {
    pub prefer_local: bool,
    pub prefer_cheapest: bool,
    pub prefer_fastest: bool,
    pub prefer_best_reasoning: bool,
    pub prefer_best_coding: bool,
    pub offline_only: bool,
    pub default_provider: Option<String>,
    pub default_model: Option<String>,
}

impl Default for ExecutionPolicy {
    fn default() -> Self {
        Self {
            prefer_local: true,
            prefer_cheapest: false,
            prefer_fastest: false,
            prefer_best_reasoning: false,
            prefer_best_coding: false,
            offline_only: false,
            default_provider: None,
            default_model: None,
        }
    }
}
