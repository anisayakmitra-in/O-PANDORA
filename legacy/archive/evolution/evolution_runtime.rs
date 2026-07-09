//! GEPA/DSR Runtime Hook Types.
//!
//! Every constitutional object supporting GEPA/DSR
//! exposes evaluate/learn/optimize/propose/repair
//! hooks. No business logic. Only invocation.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Result of a GEPA evaluation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub subject_id: String,
    pub score: f64,
    pub patterns: Vec<String>,
    pub metrics: BTreeMap<String, f64>,
    pub timestamp_ms: u64,
}

/// A DSR improvement proposal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImprovementProposal {
    pub proposal_id: String,
    pub subject_id: String,
    pub kind: ProposalKind,
    pub description: String,
    pub confidence: f64,
    pub rollback_plan: Option<String>,
    pub timestamp_ms: u64,
}

/// Kinds of DSR improvement proposals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProposalKind {
    Repair,
    Optimization,
    Mutation,
    Rollback,
    Architectural,
}

/// Result of applying an improvement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImprovementResult {
    pub proposal_id: String,
    pub applied: bool,
    pub score_before: f64,
    pub score_after: f64,
    pub rollback_available: bool,
    pub timestamp_ms: u64,
}
