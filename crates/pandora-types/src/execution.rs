//! Phoenix Execution Runtime Types.
//!
//! Phoenix is the sole execution runtime. Every
//! request executes through Phoenix. This module
//! defines the types for the execution lifecycle.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::universal::{Health, Lifecycle};

// ============================================================
// Core Execution Types
// ============================================================

/// An execution session. Phoenix creates one per request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionSession {
    pub session_id: String,
    pub status: ExecutionStatus,
    pub health: Health,
    pub lifecycle: Lifecycle,
    pub created_at_ms: u64,
    pub budget: ExecutionBudget,
    pub context: ExecutionContext,
}

/// Status of an execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
}

/// Budget for an execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionBudget {
    pub max_duration_ms: u64,
    pub max_cost_cents: u64,
    pub max_memory_mb: u64,
    pub max_cpu_ms: u64,
}

impl Default for ExecutionBudget {
    fn default() -> Self {
        ExecutionBudget {
            max_duration_ms: 300_000,
            max_cost_cents: 500,
            max_memory_mb: 4096,
            max_cpu_ms: 120_000,
        }
    }
}

/// Context for an execution. Carries all state
/// through the constitutional pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub request_id: String,
    pub input: String,
    pub metadata: BTreeMap<String, String>,
    pub trace_id: Option<String>,
    pub parent_session_id: Option<String>,
}

// ============================================================
// Execution Graph
// ============================================================

/// A node in the execution graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionNode {
    pub node_id: String,
    pub name: String,
    pub status: ExecutionStatus,
    pub capability: Option<String>,
    pub dependencies: Vec<String>,
    pub started_at_ms: Option<u64>,
    pub completed_at_ms: Option<u64>,
}

/// The execution graph for a session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionGraph {
    pub nodes: Vec<ExecutionNode>,
    pub edges: Vec<(String, String)>,
}

// ============================================================
// Checkpoint & Recovery
// ============================================================

/// A checkpoint of execution state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionCheckpoint {
    pub checkpoint_id: String,
    pub session_id: String,
    pub node_id: String,
    pub state: BTreeMap<String, String>,
    pub timestamp_ms: u64,
}

/// Result of an execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub session_id: String,
    pub status: ExecutionStatus,
    pub output: Option<String>,
    pub error: Option<String>,
    pub duration_ms: u64,
    pub cost_cents: u64,
}

// ============================================================
// Telemetry & Statistics
// ============================================================

/// Telemetry for an execution session.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExecutionTelemetry {
    pub metrics: BTreeMap<String, u64>,
    pub events: Vec<String>,
}

/// Statistics for executions.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExecutionStatistics {
    pub total_executions: u64,
    pub active_executions: u64,
    pub completed_executions: u64,
    pub failed_executions: u64,
    pub avg_duration_ms: u64,
    pub total_cost_cents: u64,
}
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn session_serde() {
        let s = ExecutionSession {
            session_id: "s1".to_string(),
            status: ExecutionStatus::Running,
            health: Health::Healthy,
            lifecycle: Lifecycle::Running,
            created_at_ms: 0,
            budget: ExecutionBudget::default(),
            context: ExecutionContext {
                request_id: "r1".to_string(),
                input: "hello".to_string(),
                metadata: BTreeMap::new(),
                trace_id: None,
                parent_session_id: None,
            },
        };
        let json = serde_json::to_string(&s).unwrap();
        let _: ExecutionSession = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn budget_default() {
        let b = ExecutionBudget::default();
        assert_eq!(b.max_duration_ms, 300_000);
    }

    #[test]
    fn graph_serde() {
        let g = ExecutionGraph {
            nodes: vec![ExecutionNode {
                node_id: "n1".to_string(),
                name: "start".to_string(),
                status: ExecutionStatus::Completed,
                capability: None,
                dependencies: vec![],
                started_at_ms: Some(0),
                completed_at_ms: Some(100),
            }],
            edges: vec![],
        };
        let json = serde_json::to_string(&g).unwrap();
        let _: ExecutionGraph = serde_json::from_str(&json).unwrap();
    }
}
