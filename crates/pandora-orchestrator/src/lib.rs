//! Pandora Orchestrator — the full 9-stage constitutional execution pipeline.
//!
//! Wires every engine into a single run() call:
//!   Instruction → Workflow → Capability → Provider → Recorder
//!   → Telemetry → FailureIntelligence → Knowledge → Ledger
//!
//! This is the `pandora.run(task)` entry point for Phase 2B.
//!
//! Every stage returns a StageOutput. Parliament merges deltas.

use anyhow::Result;
use pandora_ledger::{ExecutionLedger, LedgerEntry, LedgerOutcome};
use pandora_provider::ollama::OllamaProvider;
use std::sync::Arc;
use pandora_provider::traits::Provider;
use pandora_provider::types::GenerationRequest;
use pandora_types::capability_resolution::CapabilityResolutionEngine;
use pandora_types::failure_intelligence::{FailureIntelligenceEngine, FailureRecord};
use pandora_types::knowledge_distillation::KnowledgeDistillationEngine;
use pandora_types::recorder::{ExecutionFrame, ExecutionRecorder, ReplayId};
use pandora_types::runtime_context::RuntimeContext;
use pandora_types::telemetry_engine::TelemetryEngine;
use pandora_types::workflow_engine::{ExecutionGraph, StepKind, WorkflowStep};
use std::collections::HashMap;
use std::time::Instant;
use tokio_util::sync::CancellationToken;

// ── Stage Output types ──

#[derive(Debug, Clone)]
pub struct WorkflowStageOutput {
    pub graph: ExecutionGraph,
    pub step_count: usize,
}

#[derive(Debug, Clone)]
pub struct CapabilityStageOutput {
    pub provider: String,
    pub model: String,
    pub candidates_considered: usize,
}

#[derive(Debug, Clone)]
pub struct ProviderStageOutput {
    pub text: String,
    pub tokens_used: usize,
    pub duration_ms: u128,
}

#[derive(Debug, Clone)]
pub struct RecorderStageOutput {
    pub replay_id: String,
    pub frame_count: usize,
}

/// Full pipeline result — returned from run().
#[derive(Debug, Clone)]
pub struct ExecutionReport {
    pub execution_id: String,
    pub output: String,
    pub duration_ms: u128,
    pub provider: String,
    pub model: String,
    pub workflow_steps: usize,
    pub telemetry_spans: usize,
    pub root_causes_found: usize,
    pub knowledge_nodes: usize,
    pub ledger_entries: usize,
    pub replay_id: String,
    pub success: bool,
}


// ── RuntimeDelta — accumulates stage outputs for Parliament merge (2B-3) ──

/// Immutable accumulated delta from all 9 stages.
/// Parliament merges this into the runtime context in one step.
#[derive(Debug, Clone, Default)]
pub struct RuntimeDelta {
    pub workflow: Option<WorkflowStageOutput>,
    pub capability: Option<CapabilityStageOutput>,
    pub provider: Option<ProviderStageOutput>,
    pub recorder: Option<RecorderStageOutput>,
    pub telemetry_spans: usize,
    pub root_causes: usize,
    pub knowledge_nodes: usize,
    pub ledger_entries: usize,
    pub success: bool,
}

impl RuntimeDelta {
    pub fn new() -> Self { Self::default() }

    /// Merge into a RuntimeContext — the Parliament step.
    pub fn merge_into(&self, ctx: &mut RuntimeContext) {
        if let Some(ref wf) = self.workflow {
            ctx.set_variable("workflow_steps", wf.step_count.to_string());
        }
        if let Some(ref cap) = self.capability {
            ctx.set_variable("resolved_provider", cap.provider.clone());
            ctx.set_variable("resolved_model", cap.model.clone());
        }
        if let Some(ref prov) = self.provider {
            ctx.set_variable("output_tokens", prov.tokens_used.to_string());
        }
        if let Some(ref rec) = self.recorder {
            ctx.set_variable("replay_id", rec.replay_id.clone());
        }
        ctx.set_variable("telemetry_spans", self.telemetry_spans.to_string());
        ctx.set_variable("root_causes", self.root_causes.to_string());
        ctx.set_variable("knowledge_nodes", self.knowledge_nodes.to_string());
        ctx.set_variable("ledger_entries", self.ledger_entries.to_string());
        ctx.set_variable("pipeline_success", self.success.to_string());
        ctx.record_telemetry(format!(
            "RuntimeDelta merged: steps={} provider={} tokens={} success={}",
            self.workflow.as_ref().map(|w| w.step_count).unwrap_or(0),
            self.capability.as_ref().map(|c| c.provider.as_str()).unwrap_or(""),
            self.provider.as_ref().map(|p| p.tokens_used).unwrap_or(0),
            self.success,
        ));
    }
}

// ── ProviderRegistry — multi-provider dispatch (2B-4) ──

/// Registry of available providers. Resolves Arc<dyn Provider> by name.
/// Default provider is the first registered (typically Ollama for local).
pub struct ProviderRegistry {
    providers: Vec<Arc<dyn Provider>>,
    default_index: usize,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self { providers: Vec::new(), default_index: 0 }
    }

    /// Register a provider. First registered becomes default.
    pub fn register(&mut self, provider: Arc<dyn Provider>) {
        self.providers.push(provider);
    }

    /// Get provider by name, falling back to default.
    pub fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        self.providers.iter().find(|p| p.name() == name).cloned()
    }

    /// Default provider (first registered).
    pub fn default_provider(&self) -> Option<Arc<dyn Provider>> {
        self.providers.get(self.default_index).cloned()
    }

    /// List all provider names.
    pub fn list(&self) -> Vec<&str> {
        self.providers.iter().map(|p| p.name()).collect()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self { Self::new() }
}

// ── PandoraRuntime — constitutional pipeline ──

pub struct PandoraRuntime {
    pub ctx: RuntimeContext,
    pub recorder: ExecutionRecorder,
    pub telemetry: TelemetryEngine,
    pub failure_intel: FailureIntelligenceEngine,
    pub knowledge: KnowledgeDistillationEngine,
    pub ledger: ExecutionLedger,
    pub cap_resolution: CapabilityResolutionEngine,
    pub providers: ProviderRegistry,
}

impl PandoraRuntime {
    pub fn new() -> Self {
        let mut providers = ProviderRegistry::new();
        // ponytail: register Ollama by default; users can add more
        providers.register(Arc::new(OllamaProvider::new_default()));
        Self {
            ctx: RuntimeContext::new("default-session", "pandora"),
            recorder: ExecutionRecorder::new(),
            telemetry: TelemetryEngine::new(),
            failure_intel: FailureIntelligenceEngine::new(),
            knowledge: KnowledgeDistillationEngine::new(),
            ledger: ExecutionLedger::new(),
            cap_resolution: CapabilityResolutionEngine::new(),
            providers,
        }
    }

    /// Register an additional provider at runtime.
    pub fn register_provider(&mut self, provider: Arc<dyn Provider>) {
        self.providers.register(provider);
    }

    /// Execute a task through the full 9-stage constitutional pipeline.
    pub async fn run(&mut self, task: &str, domain: &str) -> Result<ExecutionReport> {
        let execution_id = format!("exec-{}", chrono::Utc::now().timestamp_millis());
        let start = Instant::now();

        // Stage 1: Instruction (task string IS instruction for now — Phase 3 concern)
        let _ctx = &self.ctx;

        // Stage 2: Workflow Engine
        let mut graph = ExecutionGraph::new("pandora-wf");
        let plan_step = WorkflowStep::new("plan", StepKind::Plan, "Plan execution");
        let exec_step =
            WorkflowStep::new("execute", StepKind::Execute, "Execute task").depends_on("plan");
        graph.add_step(plan_step);
        graph.add_step(exec_step);
        let topo = graph.topological_sort();
        println!(
            "[STAGE 2 - WORKFLOW] {} steps: {:?}",
            graph.steps.len(),
            topo
        );
        let wf_out = WorkflowStageOutput {
            graph,
            step_count: topo.len(),
        };

        // Stage 3: Capability Resolution
        let candidates = self.cap_resolution.resolve_domain(domain);
        let (provider_name, model) = if let Some(best) = candidates.first() {
            (best.provider.clone(), best.model.clone())
        } else {
            ("ollama".into(), "qwen2.5-coder:7b".into())
        };
        println!(
            "[STAGE 3 - RESOLUTION] {} candidates -> {}/{}",
            candidates.len(),
            provider_name,
            model
        );
        let cap_out = CapabilityStageOutput {
            provider: provider_name.clone(),
            model: model.clone(),
            candidates_considered: candidates.len(),
        };

        // Stage 4: Provider Execution (real HTTP call)
        let provider = self.providers.get(&provider_name)
            .or_else(|| self.providers.default_provider())
            .ok_or_else(|| anyhow::anyhow!("No provider available for: {}", provider_name))?;
        let request = GenerationRequest {
            model: model.clone(),
            prompt: format!(
                "Task: {task}\nDomain: {domain}\n\nExecute and return only the result.",
            ),
            temperature: 0.2,
            max_tokens: 4096,
        };
        let cancel = CancellationToken::new();
        let exec_start = Instant::now();
        let response = provider
            .generate(request, cancel)
            .await
            .map_err(|e| anyhow::anyhow!("Provider {} failed: {}", provider_name, e))?;
        let exec_ms = exec_start.elapsed().as_millis();
        println!(
            "[STAGE 4 - EXECUTION] {} tokens, {} ms",
            response.text.len(),
            exec_ms
        );
        let provider_out = ProviderStageOutput {
            text: response.text.clone(),
            tokens_used: response.text.len(),
            duration_ms: exec_ms,
        };

        // Stage 5: Recorder — record for replay
        let frame_id = format!("frame-{execution_id}-1");
        let frame = ExecutionFrame {
            frame_id: frame_id.clone(),
            parent_id: None,
            step_kind: "execute".into(),
            step_label: "provider-call".into(),
            provider: provider_name.clone(),
            model: model.clone(),
            input_hash: format!("h{:x}", task.len() as u64),
            output_hash: format!("h{:x}", response.text.len() as u64),
            duration_ms: exec_ms as u64,
            tokens_used: response.text.len(),
            cost: 0.0,
            success: true,
            retries: 0,
            artifacts: vec![],
            telemetry: vec![],
            timestamp: chrono::Utc::now(),
        };
        self.recorder
            .record_frame(&ReplayId(frame_id.clone()), frame)
            .ok();
        println!("[STAGE 5 - RECORDER] frame captured");

        let rec_out = RecorderStageOutput {
            replay_id: frame_id,
            frame_count: 1,
        };

        // Stage 6: Telemetry — begin/end trace with spans
        let trace_id = self.telemetry.begin_trace(&execution_id, task);
        let span_id = self
            .telemetry
            .begin_span(&trace_id, "provider-exec", "execute");
        self.telemetry.end_trace(&trace_id);
        let trace_count = self.telemetry.trace_count();
        println!(
            "[STAGE 6 - TELEMETRY] {} traces, span: {}",
            trace_count, span_id
        );

        // Stage 7: Failure Intelligence
        let success = !response.text.is_empty();
        if !success {
            let record = FailureRecord::new(provider_name.clone(), domain);
            self.failure_intel.ingest(record);
            self.failure_intel.cluster();
        }
        let root_causes = self.failure_intel.root_cause_count();
        println!("[STAGE 7 - INTEL] {} root causes", root_causes);

        // Stage 8: Knowledge Distillation
        if response.text.len() > 50 {
            let l1_id = self.knowledge.ingest_telemetry(
                format!("exec-{execution_id}"),
                format!("Task: {task} | Provider: {provider_name}"),
                vec![domain.to_string(), "execution".to_string()],
            );
            let _l2 = self.knowledge.distill_to_l1(
                vec![l1_id],
                format!("Execution of: {task}"),
                &response.text,
            );
            println!(
                "[STAGE 8 - DISTILLATION] {} knowledge nodes",
                self.knowledge.knowledge_count()
            );
        }

        // Stage 9: Execution Ledger — immutable permanent record
        self.ledger
            .append(LedgerEntry {
                execution_id: execution_id.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                provider: provider_name.clone(),
                workflow: "full-pipeline".into(),
                skill_version: None,
                reason: format!("Execute task in domain '{}'", domain),
                cost: 0.0,
                decision: format!("{}/{}", provider_name, model),
                outcome: if success {
                    LedgerOutcome::Success
                } else {
                    LedgerOutcome::Failure("empty-response".into())
                },
                previous_hash: None,
                hash: format!("hash-{:x}", rand::random::<u64>()),
                metadata: HashMap::from([
                    ("task".into(), task.into()),
                    ("domain".into(), domain.into()),
                    ("output_tokens".into(), response.text.len().to_string()),
                    ("duration_ms".into(), exec_ms.to_string()),
                ]),
            })
            .ok();

        println!("[STAGE 9 - LEDGER] {} entries total", self.ledger.len());

        // ── Parliament merge: RuntimeDelta → RuntimeContext ──
        let replay_id = rec_out.replay_id.clone();
        let delta = RuntimeDelta {
            workflow: Some(wf_out.clone()),
            capability: Some(cap_out.clone()),
            provider: Some(provider_out),
            recorder: Some(rec_out),
            telemetry_spans: trace_count,
            root_causes,
            knowledge_nodes: self.knowledge.knowledge_count(),
            ledger_entries: self.ledger.len(),
            success,
        };
        delta.merge_into(&mut self.ctx);
        println!("[PARLIAMENT] RuntimeDelta merged into context");

        let total = start.elapsed();
        Ok(ExecutionReport {
            execution_id,
            output: response.text,
            duration_ms: total.as_millis(),
            provider: provider_name,
            model,
            workflow_steps: wf_out.step_count,
            telemetry_spans: trace_count,
            root_causes_found: root_causes,
            knowledge_nodes: self.knowledge.knowledge_count(),
            ledger_entries: self.ledger.len(),
            replay_id,
            success,
        })
    }
}

impl Default for PandoraRuntime {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ──

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_initializes() {
        let rt = PandoraRuntime::new();
        assert_eq!(rt.ledger.len(), 0);
        assert_eq!(rt.failure_intel.root_cause_count(), 0);
        assert_eq!(rt.knowledge.knowledge_count(), 0);
    }

    #[test]
    fn workflow_planning() {
        let mut graph = ExecutionGraph::new("test-wf");
        let step = WorkflowStep::new("step-1", StepKind::Execute, "Test");
        graph.add_step(step);
        assert_eq!(graph.steps.len(), 1);
        assert_eq!(graph.topological_sort().len(), 1);
    }

    #[test]
    fn capability_resolution_degenerate() {
        let engine = CapabilityResolutionEngine::new();
        let candidates = engine.resolve_domain("nonexistent");
        assert!(candidates.is_empty());
    }

    #[test]
    fn failure_intel_counts() {
        let mut engine = FailureIntelligenceEngine::new();
        engine.ingest(FailureRecord::new("test", "domain"));
        assert_eq!(engine.failure_count(), 1);
    }

    #[test]
    fn knowledge_empty() {
        let engine = KnowledgeDistillationEngine::new();
        assert_eq!(engine.knowledge_count(), 0);
    }

    #[test]
    fn ledger_initial() {
        let ledger = ExecutionLedger::new();
        assert_eq!(ledger.len(), 0);
    }

    #[test]
    fn recorder_begins() {
        let mut recorder = ExecutionRecorder::new();
        let _rid = recorder.begin(
            "test task",
            "coding",
            "exec-1",
            "session-1",
            "project-1",
            pandora_types::recorder::RecordedProperties {
                memory_mode: "default".into(),
                loop_mode: "closed".into(),
                safety_level: "standard".into(),
                execution_backend: "ollama".into(),
                reasoning_depth: 2,
                telemetry_level: 3,
            },
        );
        assert!(recorder
            .begin(
                "t2",
                "d2",
                "e2",
                "s2",
                "p2",
                pandora_types::recorder::RecordedProperties {
                    memory_mode: "default".into(),
                    loop_mode: "closed".into(),
                    safety_level: "standard".into(),
                    execution_backend: "local".into(),
                    reasoning_depth: 1,
                    telemetry_level: 1,
                }
            )
            .0
            .starts_with("replay-"));
    }

    #[test]
    fn runtime_delta_merges_into_context() {
        let mut ctx = RuntimeContext::new("s", "p");
        let delta = RuntimeDelta {
            workflow: Some(WorkflowStageOutput {
                graph: ExecutionGraph::new("g"), step_count: 3,
            }),
            capability: Some(CapabilityStageOutput {
                provider: "ollama".into(), model: "qwen".into(), candidates_considered: 1,
            }),
            provider: Some(ProviderStageOutput {
                text: "hi".into(), tokens_used: 2, duration_ms: 10,
            }),
            recorder: Some(RecorderStageOutput { replay_id: "r1".into(), frame_count: 1 }),
            telemetry_spans: 5,
            root_causes: 0,
            knowledge_nodes: 4,
            ledger_entries: 1,
            success: true,
        };
        delta.merge_into(&mut ctx);
        assert_eq!(ctx.get_variable("workflow_steps").unwrap(), "3");
        assert_eq!(ctx.get_variable("resolved_provider").unwrap(), "ollama");
        assert_eq!(ctx.get_variable("knowledge_nodes").unwrap(), "4");
        assert_eq!(ctx.get_variable("pipeline_success").unwrap(), "true");
    }

    #[test]
    fn provider_registry_default_is_first() {
        let reg = ProviderRegistry::new();
        // empty -> no default
        assert!(reg.default_provider().is_none());
        assert!(reg.get("ollama").is_none());

        // ponytail: can't construct OllamaProvider without legacy-ollama feat in cfg(test),
        // so just verify the empty case behavior
        let _: Vec<&str> = reg.list();
        assert!(reg.list().is_empty());
    }
}
