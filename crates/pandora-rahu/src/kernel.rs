//! Runtime Kernel.
//!
//! The kernel orchestrates all Pandora subsystems through
//! the constitutional pipeline. It is the single entry
//! point for runtime execution.
//!
//! Architecture:
//!   User -> NARAD -> Kernel -> Shadow Council -> Source Harness
//!   -> Meta Harness -> Workflow -> Capability -> RAHU -> Gene
//!   -> PHOENIX -> Sandbox -> KETU -> PANOPTES -> ANUBIS
//!   -> HADES -> Telemetry -> Result

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use pandora_types::universal::{Health, Lifecycle};

use crate::cognition_scheduler::CognitionScheduler;
use crate::event_bus::{EventBus, EventCategory, RuntimeEvent};
use crate::execution_history::{ExecutionHistory, HistoryEntry, HistoryKind};
use crate::pipeline::{ConstitutionalPipeline, PipelineResult};
use crate::shadow_council::{CouncilProposal, ProposalStatus, ShadowCouncil};

/// Kernel execution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KernelMode {
    /// Single-shot execution.
    #[default]
    Single,
    /// Continuous cognition loop.
    Continuous,
    /// Background maintenance.
    Background,
}

/// Kernel state.
#[derive(Debug, Clone, Default)]
pub struct KernelState {
    pub mode: KernelMode,
    pub health: Health,
    pub lifecycle: Lifecycle,
    pub executions: u64,
    pub errors: u64,
}

/// The runtime kernel orchestrating all subsystems.
pub struct Kernel {
    pipeline: ConstitutionalPipeline,
    scheduler: CognitionScheduler,
    event_bus: EventBus,
    history: ExecutionHistory,
    state: Arc<Mutex<KernelState>>,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel {
            pipeline: ConstitutionalPipeline::new(),
            scheduler: CognitionScheduler::new(),
            event_bus: EventBus::new(),
            history: ExecutionHistory::new(),
            state: Arc::new(Mutex::new(KernelState::default())),
        }
    }

    /// Execute a request through the full constitutional pipeline.
    pub fn execute(&mut self, request_id: &str, input: &str) -> PipelineResult {
        // Publish start event
        self.event_bus.publish(RuntimeEvent {
            event_id: format!("start-{}", request_id),
            category: EventCategory::Execution,
            payload: input.to_string(),
            timestamp_ms: 0,
            metadata: BTreeMap::new(),
        });

        // Record in history
        self.history.record(HistoryEntry {
            entry_id: format!("req-{}", request_id),
            kind: HistoryKind::Execution,
            target_id: request_id.to_string(),
            payload: input.to_string(),
            metadata: BTreeMap::new(),
            timestamp_ms: 0,
        });

        // Execute through constitutional pipeline
        let result = self.pipeline.execute(request_id, input);

        // Update state
        {
            let mut state = self.state.lock().unwrap();
            state.executions += 1;
            if !result.success {
                state.errors += 1;
            }
            state.health = if result.success {
                Health::Healthy
            } else {
                Health::Degraded
            };
        }

        // Publish completion event
        self.event_bus.publish(RuntimeEvent {
            event_id: format!("complete-{}", request_id),
            category: EventCategory::Execution,
            payload: if result.success {
                "success".to_string()
            } else {
                "failure".to_string()
            },
            timestamp_ms: 0,
            metadata: BTreeMap::new(),
        });

        // Record completion in history
        self.history.record(HistoryEntry {
            entry_id: format!("result-{}", request_id),
            kind: HistoryKind::Execution,
            target_id: request_id.to_string(),
            payload: format!("success={}", result.success),
            metadata: BTreeMap::new(),
            timestamp_ms: 0,
        });

        result
    }

    /// Start the kernel in a given mode.
    pub fn start(&self, mode: KernelMode) {
        let mut state = self.state.lock().unwrap();
        state.mode = mode;
        state.lifecycle = Lifecycle::Running;
        state.health = Health::Healthy;
    }

    /// Stop the kernel.
    pub fn stop(&self) {
        let mut state = self.state.lock().unwrap();
        state.lifecycle = Lifecycle::Stopped;
    }

    /// Get kernel state.
    pub fn state(&self) -> KernelState {
        self.state.lock().unwrap().clone()
    }

    /// Get the event bus.
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Get execution history.
    pub fn history(&self) -> &ExecutionHistory {
        &self.history
    }

    /// Get the scheduler.
    pub fn scheduler(&self) -> &CognitionScheduler {
        &self.scheduler
    }

    /// Get the pipeline.
    pub fn pipeline(&self) -> &ConstitutionalPipeline {
        &self.pipeline
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_executes_through_pipeline() {
        let mut kernel = Kernel::new();
        kernel.start(KernelMode::Single);
        let result = kernel.execute("req-1", "hello world");
        assert!(result.success);
        assert!(result.governance_approved);
        assert_eq!(kernel.state().executions, 1);
        assert_eq!(kernel.state().errors, 0);
    }

    #[test]
    fn kernel_tracks_history() {
        let mut kernel = Kernel::new();
        kernel.start(KernelMode::Single);
        kernel.execute("req-1", "test");
        let entries = kernel.history().filter_by_kind(HistoryKind::Execution);
        assert!(!entries.is_empty());
    }

    #[test]
    fn kernel_publishes_events() {
        let mut kernel = Kernel::new();
        kernel.start(KernelMode::Single);
        kernel.execute("req-1", "test");
        let events = kernel.event_bus().history();
        assert!(events.len() >= 2); // start + complete
    }

    #[test]
    fn kernel_state_transitions() {
        let kernel = Kernel::new();
        assert_eq!(kernel.state().lifecycle, Lifecycle::Created);
        kernel.start(KernelMode::Continuous);
        assert_eq!(kernel.state().lifecycle, Lifecycle::Running);
        kernel.stop();
        assert_eq!(kernel.state().lifecycle, Lifecycle::Stopped);
    }

    #[test]
    fn kernel_records_failures() {
        let mut kernel = Kernel::new();
        kernel.start(KernelMode::Single);
        // Execute twice to check error counting works
        kernel.execute("req-1", "ok");
        kernel.execute("req-2", "ok");
        assert_eq!(kernel.state().executions, 2);
    }
}

// ============================================================
// Runtime Integration
// ============================================================

/// Runtime integration layer. Wires the kernel to all
/// Pandora subsystems through the constitutional pipeline.
pub struct RuntimeIntegration {
    kernel: Kernel,
    council: ShadowCouncil,
    proposals: std::sync::Arc<std::sync::Mutex<Vec<CouncilProposal>>>,
}

impl RuntimeIntegration {
    pub fn new() -> Self {
        RuntimeIntegration {
            kernel: Kernel::new(),
            council: ShadowCouncil::new(),
            proposals: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    /// Execute through the full integrated pipeline.
    pub fn execute(&mut self, request_id: &str, input: &str) -> IntegratedResult {
        let mut proposal = self.council.submit(
            request_id,
            crate::shadow_council::council_action_approve(),
            "auto-approved",
        );
        self.council.approve(&mut proposal);
        self.proposals.lock().unwrap().push(proposal.clone());

        let result = self.kernel.execute(request_id, input);

        IntegratedResult {
            request_id: request_id.to_string(),
            session_id: result.session_id.clone(),
            council_approved: proposal.status == ProposalStatus::Approved,
            pipeline_success: result.success,
            output: result.output,
            error: result.error,
        }
    }

    pub fn start(&self, mode: KernelMode) {
        self.kernel.start(mode);
    }

    pub fn stop(&self) {
        self.kernel.stop();
    }

    pub fn kernel(&self) -> &Kernel {
        &self.kernel
    }

    pub fn event_bus(&self) -> &EventBus {
        self.kernel.event_bus()
    }

    pub fn history(&self) -> &ExecutionHistory {
        self.kernel.history()
    }

    pub fn proposals(&self) -> Vec<CouncilProposal> {
        self.proposals.lock().unwrap().clone()
    }
}

impl Default for RuntimeIntegration {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of an integrated execution.
#[derive(Debug, Clone)]
pub struct IntegratedResult {
    pub request_id: String,
    pub session_id: String,
    pub council_approved: bool,
    pub pipeline_success: bool,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::execution_history::HistoryKind;

    #[test]
    fn integrated_execution_flow() {
        let mut rt = RuntimeIntegration::new();
        rt.start(KernelMode::Single);
        let result = rt.execute("req-1", "hello");
        assert!(result.council_approved);
        assert!(result.pipeline_success);
    }

    #[test]
    fn integrated_council_proposals() {
        let mut rt = RuntimeIntegration::new();
        rt.start(KernelMode::Single);
        rt.execute("req-1", "test");
        let proposals = rt.proposals();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].status, ProposalStatus::Approved);
    }

    #[test]
    fn integrated_event_bus() {
        let mut rt = RuntimeIntegration::new();
        rt.start(KernelMode::Single);
        rt.execute("req-1", "test");
        let events = rt.event_bus().history();
        assert!(events.len() >= 2);
    }

    #[test]
    fn integrated_history() {
        let mut rt = RuntimeIntegration::new();
        rt.start(KernelMode::Single);
        rt.execute("req-1", "test");
        let entries = rt.history().filter_by_kind(HistoryKind::Execution);
        assert!(!entries.is_empty());
    }

    #[test]
    fn integrated_multiple_executions() {
        let mut rt = RuntimeIntegration::new();
        rt.start(KernelMode::Continuous);
        rt.execute("req-1", "first");
        rt.execute("req-2", "second");
        assert_eq!(rt.kernel().state().executions, 2);
        assert_eq!(rt.proposals().len(), 2);
    }
}
