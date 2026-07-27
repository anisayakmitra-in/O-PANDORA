//! Execution memory types — every execution stores history, checkpoints,
//! replay data, artifacts, diagnostics, and lineage in ANUBIS.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::execution::{ExecutionCheckpoint, ExecutionResult};

/// Execution record stored in ANUBIS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionRecord {
    pub session_id: String,
    pub request_id: String,
    pub result: ExecutionResult,
    pub checkpoints: Vec<ExecutionCheckpoint>,
    pub artifacts: Vec<ExecutionArtifact>,
    pub diagnostics: Vec<String>,
    pub capability_usage: BTreeMap<String, u64>,
    pub lineage: ExecutionLineage,
    pub timestamp_ms: u64,
}

/// An artifact produced during execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionArtifact {
    pub artifact_id: String,
    pub name: String,
    pub kind: MemoryArtifactKind,
    pub size_bytes: u64,
    pub created_at_ms: u64,
}

#[non_exhaustive]
/// Kinds of execution artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryArtifactKind {
    Output,
    Log,
    Diagnostic,
    Snapshot,
    Replay,
    Benchmark,
}

/// Execution lineage tracks parent-child relationships.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExecutionLineage {
    pub parent_session_id: Option<String>,
    pub child_session_ids: Vec<String>,
    pub depth: u32,
}
