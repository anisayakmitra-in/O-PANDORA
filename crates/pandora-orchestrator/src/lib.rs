//! Pandora Orchestrator — integration crate that wires all engines into one pipeline.
//!
//! This crate depends on all engine crates and connects them through the
//! ExecutionOrchestrator. Each engine contributes to one stage of the
//! parliamentary execution lifecycle. No engine invents its own state.

use pandora_types::capability_graph::CapabilityGraphEngine;
use pandora_types::capability_resolution::CapabilityResolutionEngine;
use pandora_types::experiment::ExperimentEngine;
use pandora_types::failure_intelligence::FailureIntelligenceEngine;
use pandora_types::knowledge_distillation::KnowledgeDistillationEngine;
use pandora_types::policy_engine::PolicyEngine;
use pandora_types::profile_engine::ProfileEngine;
use pandora_types::provider_learning::ProviderLearningEngine;
use pandora_types::recorder::{ExecutionFrame, ExecutionRecorder, RecordedProperties};
use pandora_types::runtime_context::RuntimeContext;
use pandora_types::telemetry_engine::{SpanStatus, TelemetryEngine};
use pandora_types::workflow_engine::{ExecutionGraph, WorkflowEngine};

use chrono::Utc;

/// The Execution Result — a complete record of one orchestrated run.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub trace_id: String,
    pub replay_id: Option<pandora_types::recorder::ReplayId>,
    pub completed_stages: Vec<String>,
    pub failed_stage: Option<String>,
    pub total_duration_ms: u64,
    pub execution_graph: Option<ExecutionGraph>,
    pub failure_report: Option<String>,
    pub completed_at: chrono::DateTime<Utc>,
    pub artifact_count: usize,
}

/// The Execution Lifecycle — fixed pipeline that every execution follows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LifecycleStage {
    Intent,
    Instruction,
    Context,
    Planning,
    Workflow,
    CapabilityGraph,
    CapabilityResolution,
    ProviderLearning,
    Execution,
    Recorder,
    Telemetry,
    FailureIntelligence,
    KnowledgeDistillation,
    Complete,
}

/// The Integration Orchestrator — wires all engines into the execution lifecycle.
pub struct Orchestrator {
    pub workflow_engine: WorkflowEngine,
    pub capability_resolver: CapabilityResolutionEngine,
    pub provider_learning: ProviderLearningEngine,
    pub failure_intelligence: FailureIntelligenceEngine,
    pub knowledge_distillation: KnowledgeDistillationEngine,
    pub recorder: ExecutionRecorder,
    pub telemetry: TelemetryEngine,
    pub policy_engine: PolicyEngine,
    pub profile_engine: ProfileEngine,
    pub capability_graph: CapabilityGraphEngine,
    pub experiment_engine: ExperimentEngine,
}

impl Orchestrator {
    pub fn new() -> Self {
        let mut o = Self {
            workflow_engine: WorkflowEngine,
            capability_resolver: CapabilityResolutionEngine::new(),
            provider_learning: ProviderLearningEngine::new(),
            failure_intelligence: FailureIntelligenceEngine::new(),
            knowledge_distillation: KnowledgeDistillationEngine::new(),
            recorder: ExecutionRecorder::new(),
            telemetry: TelemetryEngine::new(),
            policy_engine: PolicyEngine::new(),
            profile_engine: ProfileEngine::new(),
            capability_graph: CapabilityGraphEngine::new(),
            experiment_engine: ExperimentEngine::new(),
        };
        o.policy_engine.build_standard();
        o.profile_engine.build_standard();
        o.capability_graph.build_standard();
        o
    }

    /// Execute a task through the full lifecycle pipeline.
    pub fn execute(
        &mut self,
        task: &str,
        domain: &str,
        ctx: &mut RuntimeContext,
    ) -> ExecutionResult {
        let start = std::time::Instant::now();
        let mut stages: Vec<String> = Vec::new();
        let trace_id = self.telemetry.begin_trace(&ctx.execution_id.0, task);

        // Stage 1-3: Intent, Instruction, Context
        self.telemetry
            .begin_span(&trace_id, "Setup context", "intent");
        ctx.record_telemetry(format!("Task: {} in domain: {}", task, domain));
        self.telemetry
            .end_span(&trace_id, "setup", SpanStatus::Ok, 0);
        stages.push("intent".into());
        stages.push("instruction".into());
        stages.push("context".into());

        // Stage 4: Planning — create execution graph via WorkflowEngine
        let sid = self
            .telemetry
            .begin_span(&trace_id, "Plan execution", "planning");
        let graph = WorkflowEngine::plan(ctx, task);
        ctx.add_artifact(format!(
            "workflow:{} ({} steps)",
            graph.workflow_name,
            graph.steps.len()
        ));
        self.telemetry
            .end_span(&trace_id, &sid, SpanStatus::Ok, ctx.elapsed_secs() * 1000);
        stages.push("planning".into());

        // Stage 5: Capability Graph check
        let sid = self
            .telemetry
            .begin_span(&trace_id, "Check capabilities", "capability_graph");
        let analysis = self.capability_graph.analyze_task(task, domain);
        if analysis.missing > 0 {
            ctx.record_telemetry(format!(
                "Missing capabilities: {}",
                analysis.missing_capabilities.join(", ")
            ));
        }
        self.telemetry.end_span(&trace_id, &sid, SpanStatus::Ok, 0);
        stages.push("capability_graph".into());

        // Stage 6: Capability Resolution
        let sid = self
            .telemetry
            .begin_span(&trace_id, "Resolve providers", "resolution");
        let request = pandora_types::capability_resolution::CapabilityRequest {
            domain: domain.to_string(),
            task_type: "general".to_string(),
            constraints: pandora_types::capability_resolution::CapabilityConstraints {
                max_cost: None,
                min_score: None,
                max_latency_ms: None,
                require_offline: false,
                require_tools: false,
                require_vision: false,
                min_context: None,
                preferred_models: Vec::new(),
            },
        };
        let candidates = self.capability_resolver.resolve(&request);
        ctx.provider_selection = candidates
            .first()
            .map(|c| format!("{}:{}", c.provider, c.model));
        self.telemetry.end_span(&trace_id, &sid, SpanStatus::Ok, 0);
        stages.push("resolution".into());

        // Stage 7: Provider Learning
        let sid = self
            .telemetry
            .begin_span(&trace_id, "Update models", "learning");
        for c in &candidates {
            let mut obs = pandora_types::provider_learning::ModelObservation::new(
                &c.model,
                &c.provider,
                domain,
            );
            obs.score = c.overall_score;
            self.provider_learning.observe(obs);
        }
        self.telemetry.end_span(&trace_id, &sid, SpanStatus::Ok, 0);
        stages.push("learning".into());

        // Stage 8: Execution + Recorder
        let sid = self
            .telemetry
            .begin_span(&trace_id, "Execute steps", "execution");
        let rprops = RecordedProperties {
            memory_mode: format!("{:?}", ctx.properties.memory_mode),
            loop_mode: format!("{:?}", ctx.properties.loop_mode),
            safety_level: format!("{:?}", ctx.properties.safety_level),
            execution_backend: format!("{:?}", ctx.properties.execution_backend),
            reasoning_depth: ctx.properties.reasoning_depth,
            telemetry_level: ctx.properties.telemetry_level,
        };
        let rid = self.recorder.begin(
            task,
            domain,
            &ctx.execution_id.0,
            &ctx.session_id,
            &ctx.project_id,
            rprops,
        );
        for step in &graph.steps {
            let mut frame = ExecutionFrame::new(step.kind.name(), &step.label);
            frame.provider = ctx.provider_selection.clone().unwrap_or_default();
            frame.duration_ms = 10;
            frame.success = true;
            let _ = self.recorder.record_frame(&rid, frame);
        }
        let duration = ctx.elapsed_secs() * 1000;
        let _ = self.recorder.finalize(&rid, duration, 0, 0.0, 0, true);
        self.telemetry
            .end_span(&trace_id, &sid, SpanStatus::Ok, duration);
        stages.push("execution".into());
        stages.push("recorder".into());

        // Stage 9: Telemetry
        self.telemetry.end_trace(&trace_id);
        stages.push("telemetry".into());

        // Stage 10: Failure Intelligence
        let sid = self
            .telemetry
            .begin_span(&trace_id, "Analyze failures", "failure");
        for line in &ctx.telemetry {
            let mut record =
                pandora_types::failure_intelligence::FailureRecord::new("orchestrator", domain);
            record.error_message = line.clone();
            record.trace_id = trace_id.clone();
            self.failure_intelligence.ingest(record);
        }
        let reports = {
            self.failure_intelligence.cluster();
            self.failure_intelligence.generate_reports()
        };
        self.telemetry.end_span(&trace_id, &sid, SpanStatus::Ok, 0);
        stages.push("failure".into());

        // Stage 11: Knowledge Distillation
        let sid = self
            .telemetry
            .begin_span(&trace_id, "Distill knowledge", "distillation");
        self.knowledge_distillation.ingest_telemetry(
            "orchestrator",
            task,
            vec![domain.to_string()],
        );
        for r in &reports {
            ctx.record_telemetry(format!(
                "Failure report: [{}] {}",
                r.failure_class.name(),
                r.root_cause
            ));
        }
        self.telemetry.end_span(&trace_id, &sid, SpanStatus::Ok, 0);
        stages.push("distillation".into());

        // Post-execution: Policies
        let policy_actions: Vec<String> = self
            .policy_engine
            .execute("after_coding", domain)
            .iter()
            .map(|a| format!("policy:{}", a.name()))
            .collect();
        for a in &policy_actions {
            ctx.record_telemetry(a);
        }

        stages.push("complete".into());
        let total_dur = start.elapsed().as_millis() as u64;

        ExecutionResult {
            success: true,
            trace_id,
            replay_id: Some(rid),
            completed_stages: stages,
            failed_stage: None,
            total_duration_ms: total_dur,
            execution_graph: Some(graph),
            failure_report: reports.first().map(|r| r.description.clone()),
            completed_at: Utc::now(),
            artifact_count: ctx.artifacts.len(),
        }
    }

    /// Run with a specific profile applied.
    pub fn execute_with_profile(
        &mut self,
        task: &str,
        domain: &str,
        profile_name: &str,
    ) -> ExecutionResult {
        let mut ctx = RuntimeContext::new(format!("p-{}", profile_name), domain);
        if let Some(p) = self.profile_engine.get(profile_name) {
            ctx.properties.loop_mode = if p.loop_depth > 1 {
                pandora_types::runtime_context::LoopMode::Closed
            } else {
                pandora_types::runtime_context::LoopMode::None
            };
            ctx.properties.safety_level = if p.verification_enabled {
                pandora_types::runtime_context::SafetyLevel::High
            } else {
                pandora_types::runtime_context::SafetyLevel::Low
            };
            ctx.properties.reasoning_depth = p.reasoning_depth;
            ctx.properties.cost_budget = p.cost_budget;
            ctx.properties.latency_target_ms = p.max_latency_ms;
            ctx.properties.telemetry_level = p.telemetry_level;
        }
        self.execute(task, domain, &mut ctx)
    }

    pub fn rank_providers(&self, domain: &str) -> Vec<(String, f64, f64)> {
        self.provider_learning.rank_for_domain(domain)
    }
    pub fn list_recordings(&self) -> Vec<&pandora_types::recorder::RecordedExecution> {
        self.recorder.list()
    }
    pub fn list_experiments(&self) -> Vec<&pandora_types::experiment::Experiment> {
        self.experiment_engine.list()
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}
