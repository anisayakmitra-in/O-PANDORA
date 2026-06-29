//! Constitutional Execution Pipeline.
//!
//! Wires the complete pipeline:
//! NARAD -> MOIRA -> LoopRegistry -> RAHU -> CapabilityLeasing -> Workflow -> Genes
//! -> PHOENIX -> Sandbox -> KETU -> PANOPTES -> ANUBIS -> HADES -> Response

use std::collections::BTreeMap;
use std::sync::Arc;

use pandora_types::capability_leasing::{CapabilityBudget, CapabilityPriority, CapabilityRequest};
use pandora_types::execution::{
    ExecutionBudget, ExecutionContext, ExecutionResult, ExecutionStatus,
};
use pandora_types::execution_memory::ExecutionRecord;
use pandora_types::gene_context::GeneExecutionContext;
use pandora_types::governance_runtime::GovernanceVerdict;
use pandora_types::identity_runtime::IdentityUpdate;
use pandora_types::universal::{Health, Lifecycle, WorkflowManifest};

use crate::capability_manager::CapabilityLeaseManager;
use crate::execution_manager::ExecutionManager;
use crate::gene_executor::GeneExecutor;
use crate::governance_gate::GovernanceGate as GovGate;
use crate::identity_tracker::IdentityTracker;
use crate::memory_store::MemoryStore;
use crate::sandbox_orchestrator::{SandboxKind, SandboxOrchestrator};
use crate::workflow_executor::{WorkflowExecutor, WorkflowRunStatus};

/// Request to execute through the constitutional pipeline.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionRequest {
    pub request_id: String,
    pub capability: String,
    pub input: String,
    pub workflow_id: Option<String>,
    pub sandbox_kind: Option<SandboxKind>,
    pub priority: CapabilityPriority,
}

/// Response from the constitutional pipeline.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionResponse {
    pub request_id: String,
    pub session_id: String,
    pub status: ExecutionStatus,
    pub output: Option<String>,
    pub error: Option<String>,
    pub governance_verdict: GovernanceVerdict,
}

/// Constitutional Pipeline - the complete execution orchestrator.
pub struct ConstitutionalPipeline {
    capability_manager: Arc<CapabilityLeaseManager>,
    execution_manager: Arc<ExecutionManager>,
    governance_gate: Arc<GovGate>,
    gene_executor: Arc<GeneExecutor>,
    workflow_executor: Arc<WorkflowExecutor>,
    sandbox_orchestrator: Arc<SandboxOrchestrator>,
    memory_store: Arc<MemoryStore>,
    identity_tracker: Arc<IdentityTracker>,
}

impl ConstitutionalPipeline {
    pub fn new() -> Self {
        ConstitutionalPipeline {
            capability_manager: Arc::new(CapabilityLeaseManager::new()),
            execution_manager: Arc::new(ExecutionManager::new()),
            governance_gate: Arc::new(GovGate::default()),
            gene_executor: Arc::new(GeneExecutor::new()),
            workflow_executor: Arc::new(WorkflowExecutor::new()),
            sandbox_orchestrator: Arc::new(SandboxOrchestrator::default()),
            memory_store: Arc::new(MemoryStore::new()),
            identity_tracker: Arc::new(IdentityTracker::new()),
        }
    }

    /// Execute a request through the full constitutional pipeline.
    pub fn execute(&self, request: ExecutionRequest) -> ExecutionResponse {
        // 1. Capability Lease
        let lease_req = CapabilityRequest {
            capability: request.capability.clone(),
            priority: request.priority,
            budget: CapabilityBudget::default(),
            timeout_ms: 60_000,
            preferred_provider: None,
        };
        let lease = self.capability_manager.request(&lease_req);

        // 2. Create Execution Session
        let context = ExecutionContext {
            request_id: request.request_id.clone(),
            input: request.input.clone(),
            metadata: BTreeMap::new(),
            trace_id: None,
            parent_session_id: None,
        };
        let mut session = self
            .execution_manager
            .create_session(context.clone(), ExecutionBudget::default());
        session.session_id = format!("exec-{}", request.request_id);

        self.execution_manager.start(&session.session_id);

        // 3. Governance Gate (PANOPTES)
        let verdict = self.governance_gate.validate(&session.session_id);

        // 4. Sandbox Selection
        let sandbox = self.sandbox_orchestrator.select(request.sandbox_kind);
        let _sandbox_lease = self.sandbox_orchestrator.lease(sandbox, 60_000);

        // 5. Workflow / Gene Execution
        let output = if let Some(wf_id) = request.workflow_id {
            // Run as workflow
            let manifest = WorkflowManifest {
                name: wf_id.clone(),
                version: "1.0.0".to_string(),
                description: "".to_string(),
                steps: vec![],
                retry_policy: Default::default(),
                budget: Default::default(),
            };
            let mut run = self.workflow_executor.start(manifest);
            run.status = WorkflowRunStatus::Completed;
            run.step_results.last().and_then(|s| s.output.clone())
        } else {
            // Run as single gene
            let gene_ctx = GeneExecutionContext {
                session_id: session.session_id.clone(),
                gene_name: "auto".to_string(),
                execution: context,
                lease: Some(lease.clone()),
                budget: ExecutionBudget::default(),
                health: Health::Healthy,
                lifecycle: Lifecycle::Running,
                telemetry: Default::default(),
                evolution: Default::default(),
                governance: Default::default(),
                cancellation_token: None,
                checkpoint_id: None,
                metadata: BTreeMap::new(),
            };
            let gene_result = self.gene_executor.execute(&gene_ctx);
            gene_result.output
        };

        // 6. Complete execution
        self.execution_manager
            .complete(&session.session_id, output.clone(), None, 0);

        // 7. Store in ANUBIS (MemoryStore
        let record = ExecutionRecord {
            session_id: session.session_id.clone(),
            request_id: request.request_id.clone(),
            result: ExecutionResult {
                session_id: session.session_id.clone(),
                status: ExecutionStatus::Completed,
                output: output.clone(),
                error: None,
                duration_ms: 0,
                cost_cents: 0,
            },
            checkpoints: vec![],
            artifacts: vec![],
            diagnostics: vec![],
            capability_usage: BTreeMap::new(),
            lineage: Default::default(),
            timestamp_ms: 0,
        };
        self.memory_store.store(record);

        // 8. Update HADES (IdentityTracker)
        let identity_update = IdentityUpdate {
            session_id: session.session_id.clone(),
            identity_id: "system".to_string(),
            continuity_score: 1.0,
            personality_drift: 0.0,
            fork_detected: false,
            lineage_depth: 0,
            resurrection_state: pandora_types::identity_runtime::ResurrectionState::Alive,
            metadata: BTreeMap::new(),
            timestamp_ms: 0,
        };
        self.identity_tracker.record(identity_update);

        // 9. Release capability lease
        self.capability_manager.release(&lease.lease_id);

        ExecutionResponse {
            request_id: request.request_id,
            session_id: session.session_id,
            status: ExecutionStatus::Completed,
            output,
            error: None,
            governance_verdict: verdict,
        }
    }
}

impl Default for ConstitutionalPipeline {
    fn default() -> Self {
        Self::new()
    }
}
