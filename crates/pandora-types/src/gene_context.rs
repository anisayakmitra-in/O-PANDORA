//! Gene Execution Context.
//!
//! Every Gene receives this context when executing.
//! It carries everything the Gene needs: capability
//! lease, budget, telemetry, health, cancellation,
//! checkpoint, replay, evolution config, governance.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::capability_leasing::CapabilityLease;
use crate::execution::{ExecutionBudget, ExecutionContext};
use crate::universal::{EvolutionConfig, GovernanceMetadata, Health, Lifecycle, Telemetry};

/// Context passed to every Gene during execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneExecutionContext {
    pub session_id: String,
    pub gene_name: String,
    pub execution: ExecutionContext,
    pub lease: Option<CapabilityLease>,
    pub budget: ExecutionBudget,
    pub health: Health,
    pub lifecycle: Lifecycle,
    pub telemetry: Telemetry,
    pub evolution: EvolutionConfig,
    pub governance: GovernanceMetadata,
    pub cancellation_token: Option<String>,
    pub checkpoint_id: Option<String>,
    pub metadata: BTreeMap<String, String>,
}

/// Result of a Gene execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneExecutionResult {
    pub gene_name: String,
    pub session_id: String,
    pub status: GeneStatus,
    pub output: Option<String>,
    pub error: Option<String>,
    pub duration_ms: u64,
    pub telemetry: Telemetry,
}

/// Status of a Gene execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneStatus {
    Success,
    Failure,
    Cancelled,
    TimedOut,
    Skipped,
}
