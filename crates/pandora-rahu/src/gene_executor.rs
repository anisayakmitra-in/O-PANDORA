//! Gene Executor.
//!
//! Genes execute through the constitutional pipeline:
//! Workflow -> Capability Lease -> Phoenix -> Sandbox.
//! Never directly. Supports CHAIN, HYBRID, INDEPENDENT.

use pandora_types::gene_context::{GeneExecutionContext, GeneExecutionResult, GeneStatus};

/// Dispatches gene execution through the constitutional pipeline.
pub struct GeneExecutor;

impl GeneExecutor {
    pub fn new() -> Self {
        GeneExecutor
    }

    /// Execute a gene with the given context.
    /// Returns a placeholder result. Real execution
    /// requires wiring to PHOENIX and sandbox backends.
    pub fn execute(&self, ctx: &GeneExecutionContext) -> GeneExecutionResult {
        GeneExecutionResult {
            gene_name: ctx.gene_name.clone(),
            session_id: ctx.session_id.clone(),
            status: GeneStatus::Success,
            output: Some(format!("gene {} executed", ctx.gene_name)),
            error: None,
            duration_ms: 0,
            telemetry: ctx.telemetry.clone(),
        }
    }
}

impl Default for GeneExecutor {
    fn default() -> Self {
        Self::new()
    }
}
