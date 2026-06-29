//! Constitutional Execution Pipeline.
//!
//! Wires all runtime components into a single
//! execution flow:
//!
//! Request -> NARAD -> MOIRA -> LoopRegistry ->
//! RAHU -> CapabilityLeasing -> Workflow ->
//! Genes -> PHOENIX -> Sandbox -> KETU ->
//! PANOPTES -> ANUBIS -> HADES -> Response

use pandora_types::capability_leasing::{CapabilityBudget, CapabilityPriority, CapabilityRequest};
use pandora_types::execution::{ExecutionBudget, ExecutionContext};
use pandora_types::execution_memory::{ExecutionLineage, ExecutionRecord};
use pandora_types::gene_context::GeneExecutionContext;
use pandora_types::identity_runtime::{IdentityUpdate, ResurrectionState};
use pandora_types::universal::{Health, Lifecycle, Telemetry};

use crate::capability_manager::CapabilityLeaseManager;
use crate::execution_manager::ExecutionManager;
use crate::gene_executor::GeneExecutor;
use crate::governance_gate::GovernanceGate;
use crate::identity_tracker::IdentityTracker;
use crate::memory_store::MemoryStore;
use crate::sandbox_orchestrator::SandboxOrchestrator;
use crate::workflow_executor::WorkflowExecutor;

/// The constitutional pipeline orchestrator.
/// Wires all components together.
pub struct ConstitutionalPipeline {
    pub capabilities: CapabilityLeaseManager,
    pub execution: ExecutionManager,
    pub workflows: WorkflowExecutor,
    pub sandbox: SandboxOrchestrator,
    pub governance: GovernanceGate,
    pub memory: MemoryStore,
    pub identity: IdentityTracker,
    pub genes: GeneExecutor,
}

impl ConstitutionalPipeline {
    pub fn new() -> Self {
        ConstitutionalPipeline {
            capabilities: CapabilityLeaseManager::new(),
            execution: ExecutionManager::new(),
            workflows: WorkflowExecutor::new(),
            sandbox: SandboxOrchestrator::default(),
            governance: GovernanceGate::default(),
            memory: MemoryStore::new(),
            identity: IdentityTracker::new(),
            genes: GeneExecutor::new(),
        }
    }

    /// Execute a request through the constitutional pipeline.
    pub fn execute(&self, request_id: &str, input: &str) -> PipelineResult {
        // 1. Create execution session
        let session = self.execution.create_session(
            ExecutionContext {
                request_id: request_id.to_string(),
                input: input.to_string(),
                metadata: Default::default(),
                trace_id: None,
                parent_session_id: None,
            },
            ExecutionBudget::default(),
        );

        // 2. Start execution
        self.execution.start(&session.session_id);

        // 3. Lease capability
        let lease = self.capabilities.request(&CapabilityRequest {
            capability: "execution".to_string(),
            priority: CapabilityPriority::Normal,
            budget: CapabilityBudget::default(),
            timeout_ms: 60_000,
            preferred_provider: None,
        });

        // 4. Select sandbox
        let sandbox_kind = self.sandbox.select(None);
        let _sb_lease = self.sandbox.lease(sandbox_kind, 60_000);

        // 5. Governance gate
        let verdict = self.governance.validate(&session.session_id);
        let approved = self.governance.is_approved(&verdict);

        if !approved {
            self.execution.complete(
                &session.session_id,
                None,
                Some("governance rejected".to_string()),
                0,
            );
            return PipelineResult {
                session_id: session.session_id.clone(),
                success: false,
                output: None,
                error: Some("governance rejected".to_string()),
                governance_approved: false,
            };
        }

        // 6. Execute gene
        let gene_ctx = GeneExecutionContext {
            session_id: session.session_id.clone(),
            gene_name: "default".to_string(),
            execution: ExecutionContext {
                request_id: request_id.to_string(),
                input: input.to_string(),
                metadata: Default::default(),
                trace_id: None,
                parent_session_id: None,
            },
            lease: Some(lease.clone()),
            budget: ExecutionBudget::default(),
            health: Health::Healthy,
            lifecycle: Lifecycle::Running,
            telemetry: Telemetry::default(),
            evolution: Default::default(),
            governance: Default::default(),
            cancellation_token: None,
            checkpoint_id: None,
            metadata: Default::default(),
        };
        let gene_result = self.genes.execute(&gene_ctx);

        // 7. Complete execution
        self.execution.complete(
            &session.session_id,
            gene_result.output.clone(),
            gene_result.error.clone(),
            gene_result.duration_ms,
        );

        // 8. Store in ANUBIS
        self.memory.store(ExecutionRecord {
            session_id: session.session_id.clone(),
            request_id: request_id.to_string(),
            result: self.execution.get_result(&session.session_id).unwrap(),
            checkpoints: vec![],
            artifacts: vec![],
            diagnostics: vec![],
            capability_usage: Default::default(),
            lineage: ExecutionLineage {
                parent_session_id: None,
                child_session_ids: vec![],
                depth: 0,
            },
            timestamp_ms: 0,
        });

        // 9. Update HADES
        self.identity.record(IdentityUpdate {
            session_id: session.session_id.clone(),
            identity_id: "default".to_string(),
            continuity_score: 1.0,
            personality_drift: 0.0,
            fork_detected: false,
            lineage_depth: 0,
            resurrection_state: ResurrectionState::Alive,
            metadata: Default::default(),
            timestamp_ms: 0,
        });

        // 10. Release leases
        self.capabilities.release(&lease.lease_id);

        PipelineResult {
            session_id: session.session_id.clone(),
            success: gene_result.status == pandora_types::gene_context::GeneStatus::Success,
            output: gene_result.output,
            error: gene_result.error,
            governance_approved: true,
        }
    }
}

impl Default for ConstitutionalPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a pipeline execution.
#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub session_id: String,
    pub success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
    pub governance_approved: bool,
}
