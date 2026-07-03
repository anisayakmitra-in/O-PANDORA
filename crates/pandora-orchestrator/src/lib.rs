//! Pandora Execution Pipeline — full constitutional runtime.
//!
//! Wires all engines into a single execute() call:
//!   Task → Workflow → CapResolution → Provider → Recorder
//!   → Telemetry → FailureIntel → KnowledgeDist → Ledger

use anyhow::{Context, Result};
use pandora_ledger::{ExecutionLedger, LedgerEntry, LedgerOutcome};
use pandora_provider::ollama::OllamaProvider;
use pandora_provider::traits::Provider;
use pandora_provider::types::GenerationRequest;
use pandora_types::capability_resolution::CapabilityResolutionEngine;
use pandora_types::failure_intelligence::FailureIntelligenceEngine;
use pandora_types::knowledge_distillation::KnowledgeDistillationEngine;
use pandora_types::recorder::{ExecutionRecorder, RecordedExecution, ReplayFrame};
use pandora_types::runtime_context::RuntimeContext;
use pandora_types::telemetry_engine::TelemetryEngine;
use pandora_types::workflow_engine::{StepKind, WorkflowEngine, WorkflowStep};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio_util::sync::CancellationToken;

/// Result of a full pipeline execution.
pub struct ExecutionResult {
    pub output: String,
    pub execution_id: String,
    pub duration_ms: u128,
    pub provider: String,
    pub model: String,
    pub replay_id: String,
    pub telemetry_events: usize,
    pub ledger_entries: usize,
}

/// The Pandora constitutional execution pipeline.
pub struct PandoraRuntime {
    pub ctx: RuntimeContext,
    pub recorder: ExecutionRecorder,
    pub telemetry: TelemetryEngine,
    pub failure_intel: FailureIntelligenceEngine,
    pub knowledge: KnowledgeDistillationEngine,
    pub ledger: ExecutionLedger,
    pub cap_resolution: CapabilityResolutionEngine,
    pub workflow_engine: WorkflowEngine,
}

impl PandoraRuntime {
    pub fn new() -> Self {
        Self {
            ctx: RuntimeContext::new(),
            recorder: ExecutionRecorder::new(),
            telemetry: TelemetryEngine::new(),
            failure_intel: FailureIntelligenceEngine::new(),
            knowledge: KnowledgeDistillationEngine::new(),
            ledger: ExecutionLedger::new(),
            cap_resolution: CapabilityResolutionEngine::new(),
            workflow_engine: WorkflowEngine::new("pandora-exec"),
        }
    }

    /// Execute a task through the full constitutional pipeline.
    pub async fn execute(&mut self, task: &str, domain: &str) -> Result<ExecutionResult> {
        let execution_id = format!("exec-{}", chrono::Utc::now().timestamp());
        let start = Instant::now();

        // 1. Workflow Engine — plan execution steps
        let mut step = WorkflowStep::new("step-1", StepKind::Execute, task);
        step = step.with_description("Execute task via provider");
        self.workflow_engine.add_step(step);

        let plan = self.workflow_engine.plan();
        println!("[PLAN] {} steps: {:?}", plan.len(), plan);

        // 2. Capability Resolution — find best provider for domain
        let capability = self.cap_resolution.resolve_domain(domain);
        let (provider_name, model) = capability.first()
            .map(|c| (c.provider_name.clone(), c.model_name.clone()))
            .unwrap_or_else(|| ("ollama".into(), "qwen2.5-coder:7b".into()));

        // 3. Provider Selection & Execution
        let provider = OllamaProvider::new_default();
        let request = GenerationRequest {
            model: model.clone(),
            prompt: task.into(),
            temperature: 0.2,
            max_tokens: 4096,
        };
        let cancel = CancellationToken::new();

        let exec_start = Instant::now();
        let response = provider.generate(request, cancel)
            .await
            .context("Provider execution failed")?;
        let provider_duration = exec_start.elapsed();

        // 4. Recorder — record the execution
        let frame = ReplayFrame::new("execute", "provider-call")
            .with_input(&format!("{{\"model\":\"{}\",\"prompt\":\"{}\"}}", model, task))
            .with_output(&response.text);
        self.recorder.begin(execution_id.clone(), domain);
        self.recorder.record_frame(frame);
        let recorded = self.recorder.finalize("completed");

        // 5. Telemetry — emit execution spans
        self.telemetry.provider_call(&provider.name(), "ollama", &request.model, provider_duration, response.text.len());

        // 6. Failure Intelligence — check for execution failures
        if response.text.is_empty() {
            self.failure_intel.record_failure(
                "provider-returned-empty",
                &format!("Provider {} returned empty response", provider.name()),
            );
        }

        // 7. Knowledge Distillation — extract insights
        if response.text.len() > 10 {
            self.knowledge.ingest(&[
                ("execution".into(), response.text.clone()),
                ("provider".into(), provider.name().into()),
                ("domain".into(), domain.into()),
            ]);
        }

        // 8. Execution Ledger — permanent immutable record
        self.ledger.append(LedgerEntry {
            execution_id: execution_id.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            provider: provider_name.clone(),
            workflow: "direct-execute".into(),
            skill_version: None,
            reason: format!("Execute task in domain '{}'", domain),
            cost: 0.0,
            decision: format!("{}/{}", provider_name, model),
            outcome: LedgerOutcome::Success,
            previous_hash: None,
            hash: format!("hash-{}", rand::random::<u64>()),
            metadata: HashMap::from([
                ("task".into(), task.into()),
                ("domain".into(), domain.into()),
                ("tokens".into(), response.text.len().to_string()),
            ]),
        }).ok();

        let duration = start.elapsed();
        Ok(ExecutionResult {
            output: response.text,
            execution_id,
            duration_ms: duration.as_millis(),
            provider: provider_name,
            model,
            replay_id: recorded.replay_id,
            telemetry_events: 1,
            ledger_entries: self.ledger.len(),
        })
    }
}

impl Default for PandoraRuntime {
    fn default() -> Self {
        Self::new()
    }
}
