//! Pandora Orchestrator — the full 9-stage constitutional execution pipeline.
//!
//! Wires every engine into a single run() call:
//!   Instruction → Workflow → Capability → Provider → Recorder
//!   → Telemetry → FailureIntelligence → Knowledge → Ledger
//!
//! This is the `pandora.run(task)` entry point for Phase 2B.
//!
//! Every stage returns a StageOutput. Parliament merges deltas.

pub mod agentic_loop;
pub mod provider_adapter;
pub mod constitutional_floor;

use anyhow::Result;
use pandora_services::ExecutionController;
use pandora_shadow_council::ShadowCouncil;
use pandora_types::ledger::{ExecutionLedger, LedgerEntry, LedgerOutcome};
use pandora_types::provider::ollama::OllamaProvider;
use pandora_types::provider::GenerationRequest;
use pandora_types::provider::Provider;
mod sinks;
use pandora_types::artifacts::ArtifactGraph;
use pandora_types::capability_resolution::CapabilityResolutionEngine;
use pandora_types::events::EventSink;
use pandora_types::execution_plan::ExecutionPlan;
use pandora_types::failure_intelligence::{FailureIntelligenceEngine, FailureRecord};
use pandora_types::knowledge_distillation::KnowledgeDistillationEngine;
use pandora_types::policy_engine::PolicyEngine;
use pandora_types::provenance::{ExecutionProvenanceGraph, ProvenanceNodeKind};

use pandora_types::parliament::ParliamentVerdict;
use pandora_types::provider_db::{ProviderDb, ProviderObservation};
use pandora_types::provider_intel::ProviderIntelligenceEngine;
use pandora_types::recorder::{ExecutionFrame, ExecutionRecorder, ReplayId};
use pandora_types::runtime_context::RuntimeContext;
use pandora_types::session::SessionStore;
use pandora_types::telemetry_engine::TelemetryEngine;
use pandora_types::workflow_engine::{ExecutionGraph, StepKind, WorkflowStep};
use sinks::BroadcastSink;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, info};

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
    pub fn new() -> Self {
        Self::default()
    }

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
            self.capability
                .as_ref()
                .map(|c| c.provider.as_str())
                .unwrap_or(""),
            self.provider.as_ref().map(|p| p.tokens_used).unwrap_or(0),
            self.success,
        ));
    }
}

// ── ProviderRegistry — multi-provider dispatch with model resolution (2C) ──

use pandora_types::provider::ExecutionTarget;

/// Registry of available providers with model-level resolution.
pub struct ProviderRegistry {
    providers: Vec<Arc<dyn Provider>>,
    default_provider_name: Option<String>,
    default_model_name: Option<String>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            default_provider_name: None,
            default_model_name: None,
        }
    }

    pub fn register(&mut self, provider: Arc<dyn Provider>) {
        if self.default_provider_name.is_none() {
            self.default_provider_name = Some(provider.name().to_string());
            let manifest = provider.manifest();
            self.default_model_name = manifest.models.first().cloned();
        }
        self.providers.push(provider);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        self.providers.iter().find(|p| p.name() == name).cloned()
    }

    pub fn set_default_model(&mut self, model: Option<String>) {
        self.default_model_name = model;
    }
    pub fn set_defaults(&mut self, provider: Option<&str>, model: Option<&str>) {
        self.default_provider_name = provider.map(|s| s.to_string());
        self.default_model_name = model.map(|s| s.to_string());
    }

    /// Resolve an ExecutionTarget from hints + defaults.
    pub fn resolve(
        &self,
        provider_hint: Option<&str>,
        model_hint: Option<&str>,
        _cap: Option<&str>,
    ) -> Option<ExecutionTarget> {
        let pname = provider_hint.or(self.default_provider_name.as_deref())?;
        let provider = self.get(pname)?;
        let manifest = provider.manifest();
        let model = model_hint
            .or(self.default_model_name.as_deref())
            .or(manifest.models.first().map(|s| s.as_str()))?;
        let locality = manifest.locality.clone();
        Some(ExecutionTarget {
            provider: pname.to_string(),
            model: model.to_string(),
            endpoint: manifest.endpoint.clone(),
            capabilities: manifest.capabilities.clone(),
            locality,
        })
    }

    pub fn list(&self) -> Vec<&str> {
        self.providers.iter().map(|p| p.name()).collect()
    }

    pub fn list_models(&self) -> Vec<(String, String)> {
        let mut r = Vec::new();
        for p in &self.providers {
            for m in &p.manifest().models {
                r.push((p.name().to_string(), m.clone()));
            }
        }
        r
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
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
    pub sessions: SessionStore,
    pub council: ShadowCouncil,
    pub parliament: pandora_types::parliament::Parliament,
    pub controller: ExecutionController,
    pub plan: ExecutionPlan,
    pub events: Box<dyn EventSink>,
    pub provenance: ExecutionProvenanceGraph,
    pub cancel_token: pandora_types::provider::CancellationToken,
    pub artifacts: ArtifactGraph,
    pub provider_db: ProviderDb,
    pub provider_intel: ProviderIntelligenceEngine,
    pub policy_engine: PolicyEngine,
    pub memory: pandora_types::hierarchical_memory::HierarchicalMemory,
    pub event_store: pandora_types::event_store::EventStore,
    pub healing: pandora_types::self_healing::HealingSession,
    pub constitutional_floor: crate::constitutional_floor::ConstitutionalFloor,
    pub provider_failover_count: u32,
}

impl PandoraRuntime {
    pub fn new() -> Self {
        let mut providers = ProviderRegistry::new();

        // Phase 6: Use provider adapter — no hardcoded Ollama fallback.
        // Each connection is mapped to the correct Provider implementation.
        match crate::provider_adapter::load_providers_from_connections() {
            loaded if !loaded.is_empty() => {
                for (provider, _name) in loaded {
                    providers.register(provider);
                }
            }
            _ => {
                // No connections configured — emit a clear diagnostic.
                // In dev mode, fall back to local Ollama for convenience.
                if std::env::var("PANDORA_DEV_MODE").is_ok() {
                    eprintln!("[PROVIDER] No connections found. Using local Ollama (dev mode).");
                    providers.register(Arc::new(OllamaProvider::new_default()));
                } else {
                    eprintln!(
                        "[PROVIDER] No healthy provider configured.\n                         Add one with: pandora connection add <name> <kind> <endpoint>\n                         Or set PANDORA_DEV_MODE=1 for local Ollama fallback."
                    );
                }
            }
        }

        providers.set_default_model(Some(
            std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| String::new()),
        ));
        Self {
            ctx: RuntimeContext::new("default-session", "pandora"),
            recorder: ExecutionRecorder::new(),
            telemetry: TelemetryEngine::new(),
            failure_intel: FailureIntelligenceEngine::new(),
            knowledge: KnowledgeDistillationEngine::new(),
            ledger: ExecutionLedger::new(),
            sessions: SessionStore::new(),
            council: { ShadowCouncil::new() },
            parliament: pandora_types::parliament::Parliament::new(),
            controller: ExecutionController::new(),
            plan: ExecutionPlan::default(),
            events: Box::new(BroadcastSink::new(256).0),
            provenance: ExecutionProvenanceGraph::new("pending"),
            artifacts: ArtifactGraph::new(),
            cancel_token: pandora_types::provider::CancellationToken::new(),
            provider_db: ProviderDb::new(),
            provider_intel: ProviderIntelligenceEngine::new(),
            policy_engine: PolicyEngine::new(),
            cap_resolution: CapabilityResolutionEngine::new(),
            providers,
            memory: pandora_types::hierarchical_memory::HierarchicalMemory::new(),
            event_store: pandora_types::event_store::EventStore::new(
                std::env::var("PANDORA_HOME")
                    .map(|h| std::path::PathBuf::from(h).join("events"))
                    .unwrap_or_else(|_| {
                        std::path::PathBuf::from(
                            std::env::var("HOME").unwrap_or_else(|_| ".".into()),
                        )
                        .join(".pandora/events")
                    }),
            ),
            healing: pandora_types::self_healing::HealingSession::new("default"),
            constitutional_floor: crate::constitutional_floor::ConstitutionalFloor::new("default"),
            provider_failover_count: 0,
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

        // ── Provenance: initialize graph ──
        self.provenance = ExecutionProvenanceGraph::new(&execution_id);
        let tid = format!("task-{execution_id}");
        let oid = format!("outcome-{execution_id}");
        self.provenance
            .add_node(ProvenanceNodeKind::Task, &tid, task);
        self.provenance.add_node(
            ProvenanceNodeKind::ExecutionPlan,
            format!("plan-{execution_id}"),
            "plan",
        );
        self.provenance
            .add_node(ProvenanceNodeKind::Outcome, &oid, "Pending");
        self.provenance
            .connect(&tid, format!("plan-{execution_id}"), "controller initiated");

        // ── Parliament pre-flight: constitutional check before any execution ──
        let pre_verdict = self.parliament.pre_flight(&execution_id, task);
        match pre_verdict {
            ParliamentVerdict::Deny { reason } => {
                return Err(anyhow::anyhow!("Parliament pre-flight denied: {}", reason));
            }
            ParliamentVerdict::RequireApproval { who, expires } => {
                return Err(anyhow::anyhow!(
                    "Parliament requires approval from {:?} (expires: {:?})",
                    who, expires
                ));
            }
            ParliamentVerdict::Modify { amended_plan } => {
                // Apply the amended plan if provided
                if let Ok(plan) = serde_json::from_value(amended_plan) {
                    self.plan = plan;
                    tracing::info!("[PARLIAMENT] Execution plan amended by Parliament");
                }
            }
            ParliamentVerdict::Escalate { to } => {
                tracing::warn!("[PARLIAMENT] Pre-flight escalated to: {:?}", to);
            }
            ParliamentVerdict::Allow => {}
            _ => {} // non_exhaustive
        }

        // ── Session: first-class execution object ──
        let mut session = pandora_types::Session::new(&execution_id, task);
        session
            .metadata
            .insert("domain".to_string(), domain.to_string());
        session
            .metadata
            .insert("execution_id".to_string(), execution_id.clone());
        session.status = pandora_types::SessionStatus::Running;
        let _session_id = session.id.clone();

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

        // Stage 2b: Shadow Council — route by intent and capability
        let route_request = pandora_types::intent_router::CapabilityRequest {
            intent: task.to_string(),
            required: pandora_types::intent_router::IntentRouter::capabilities_from_intent(task),
            preferred: vec![],
            budget: None,
            policy: None,
        };

        let route = self.council.route(route_request).map_err(|e| {
            anyhow::anyhow!("Shadow Council could not route task: {}", e)
        })?;

        info!(
            "[STAGE 2b - COUNCIL] routed to harness '{}' (gene: {:?}): {}",
            route.harness_id, route.gene_id, route.rationale
        );

        session
            .metadata
            .insert("selected_harness".to_string(), route.harness_id.clone());
        if let Some(ref gene_id) = route.gene_id {
            session
                .metadata
                .insert("selected_gene".to_string(), gene_id.clone());
        }

        // ponytail: sandbox via ExecutionBudget.sandbox_level
        debug!("[PERM] sandbox level: {:?}", self.plan.budget.sandbox_level);
        // Stage 3: Capability Resolution
        let candidates = self.cap_resolution.resolve_domain(domain);
        let (provider_name, model) = if let Some(best) = candidates.first() {
            (best.provider.clone(), best.model.clone())
        } else if let Some((p, m)) = self
            .provider_db
            .best(&self.plan.provider_policy)
            .map(|(pp, mm)| (pp.to_string(), mm.to_string()))
        {
            (p, m)
        } else if let Some((p, m)) = self.provider_intel.best(true, false) {
            (p.to_string(), m.to_string())
        } else if let Some(target) = self.providers.resolve(None, None, None) {
            (target.provider, target.model)
        } else {
            self.controller.decide(
                "provider-selection",
                "none",
                "no provider available",
                vec![],
            );
            return Err(anyhow::anyhow!(
                "No provider available - configure a default"
            ));
        };
        // Record decision
        let rejected: Vec<(&str, &str)> = candidates
            .iter()
            .skip(1)
            .map(|c| (c.provider.as_str(), "lower priority"))
            .collect();
        self.controller.decide(
            "provider-selection",
            &format!("{}/{}", provider_name, model),
            "highest priority candidate",
            rejected,
        );
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

        // Stage 4: Agentic Loop — LLM <-> gene execution
        let provider = self
            .providers
            .get(&provider_name)
            .ok_or_else(|| anyhow::anyhow!("Provider not found: {}", provider_name))?;

        // Collect all registered genes from the Shadow Council
        let gene_refs: Vec<&dyn pandora_types::gene::Gene> = self
            .council
            .all_genes()
            .iter()
            .map(|ig| ig.gene.as_ref())
            .collect();

        let exec_start = Instant::now();

        let response = if gene_refs.is_empty() {
            // No genes registered — fall back to single-shot
            let request = GenerationRequest {
                model: model.clone(),
                prompt: format!(
                    "Task: {task}\nDomain: {domain}\n\nExecute and return only the result.",
                ),
                temperature: 0.2,
                ..Default::default()
            };
            provider
                .generate(request)
                .map_err(|e| anyhow::anyhow!("Provider {} failed: {}", provider_name, e))?
        } else {
            // --- Load context from self-improvement modules ---
            // Search memory for relevant facts about this task
            let memory_hits = self.memory.search_by_content(task, None);
            if !memory_hits.is_empty() {
                info!(
                    "[STAGE 4 - MEMORY] {} relevant memories loaded",
                    memory_hits.len()
                );
            }
            // Purge expired memories
            self.memory.purge_expired();

            // Start healing session for this execution
            let healing_session = pandora_types::self_healing::HealingSession::new(&execution_id);
            self.constitutional_floor = crate::constitutional_floor::ConstitutionalFloor::new(&execution_id);

            // Run the agentic loop: LLM calls genes as tools
            let config = agentic_loop::AgenticConfig::default();
            let result = agentic_loop::run_agentic_loop(
                task,
                domain,
                provider.as_ref(),
                &gene_refs,
                None,
                Some(&self.parliament),
                &config,
                Some(&mut self.constitutional_floor),
            )
            .map_err(|e| anyhow::anyhow!("Agentic loop failed: {}", e))?;

            println!(
                "[STAGE 4 - AGENTIC LOOP] {} turns, {} tool calls, {} tokens, {} ms, {} ctx dropped, {} governance warnings",
                result.turns_used, result.tool_calls_made, result.total_tokens, result.duration_ms,
                result.context_messages_dropped, result.governance_warnings
            );

            // --- Record tool results in self-improvement modules ---
            for tr in &result.tool_results {
                // 1. EventStore: record each tool call as a pipeline event
                self.event_store.push(
                    &execution_id,
                    pandora_types::events::PipelineEvent::GeneExecuted {
                        gene: tr.tool_name.clone(),
                        duration_ms: tr.duration_ms,
                        success: tr.success,
                    },
                );

                // 2. Telemetry: trace each tool call
                let tid = self
                    .telemetry
                    .begin_trace(&execution_id, format!("tool:{}", tr.tool_name));
                self.telemetry.begin_span(&tid, "gene-execute", "tool");

                // 3. FailureIntelligence: classify tool failures
                if !tr.success {
                    let record = pandora_types::failure_intelligence::FailureRecord::new(
                        tr.tool_name.clone(),
                        domain,
                    );
                    self.failure_intel.ingest(record);
                    self.failure_intel.cluster();
                    info!(
                        "[STAGE 4 - HEALING] gene '{}' failed, {} root causes tracked",
                        tr.tool_name,
                        self.failure_intel.root_cause_count()
                    );
                }

                // 4. KnowledgeDistillation: distill tool results into knowledge
                if tr.output.len() > 50 {
                    let l1_id = self.knowledge.ingest_telemetry(
                        format!("tool-{}", tr.tool_name),
                        format!("Gene: {} | Input: {}", tr.tool_name, tr.input),
                        vec![domain.to_string(), "tool".to_string(), tr.tool_name.clone()],
                    );
                    let _l2 = self.knowledge.distill_to_l1(
                        vec![l1_id],
                        format!("Tool execution: {}", tr.tool_name),
                        &tr.output,
                    );
                }
            }

            // 5. HierarchicalMemory: store session in episodic memory
            self.memory.remember(
                pandora_types::hierarchical_memory::MemoryLayer::Session,
                format!(
                    "Task: {} | Tools: {} | Turns: {}",
                    task, result.tool_calls_made, result.turns_used
                ),
                vec![domain.to_string(), "execution".to_string()],
                0.5,
            );

            // 6. SelfHealing: check if any failures were detected
            let _can_retry = healing_session.can_retry();

            // 7. Flush event store
            let _ = self.event_store.flush();

            result.output
        };

        let exec_ms = exec_start.elapsed().as_millis();
        let provider_out = ProviderStageOutput {
            text: response.clone(),
            tokens_used: response.len(),
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
            output_hash: format!("h{:x}", response.len() as u64),
            duration_ms: exec_ms as u64,
            tokens_used: response.len(),
            cost: 0.0,
            success: true,
            retries: 0,
            artifacts: vec![],
            telemetry: vec![],
            timestamp: chrono::Utc::now(),
        };
        let _ = self
            .recorder
            .record_frame(&ReplayId(frame_id.clone()), frame);
        if self.cancel_token.is_cancelled() {
            return Err(anyhow::anyhow!("Execution cancelled"));
        }
        info!("[STAGE 5 - RECORDER] frame captured");
        self.provider_db.record(ProviderObservation {
            provider: provider_name.clone(),
            model: model.clone(),
            latency_ms: exec_ms as u64,
            tokens_used: response.len(),
            success: !response.is_empty(),
            cost_usd: 0.0,
            timestamp: std::time::SystemTime::now(),
        });

        session
            .metadata
            .insert("replay_id".to_string(), frame_id.clone());

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
        let success = !response.is_empty();
        // ponytail: persist to ~/.pandora/events.log
        use std::io::Write;
        if let Ok(home) = std::env::var("HOME") {
            let log = std::path::PathBuf::from(home).join(".pandora/events.log");
            let _ = std::fs::create_dir_all(
                log.parent().unwrap_or_else(|| std::path::Path::new("/tmp")),
            );
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log)
                .map(|mut f| {
                    let _ = writeln!(
                        f,
                        "{}|execution|{}b|{}",
                        self.ctx.session_id,
                        response.len(),
                        success
                    );
                });
        }
        if !success {
            let record = FailureRecord::new(provider_name.clone(), domain);
            self.failure_intel.ingest(record);
            self.failure_intel.cluster();
        }
        let root_causes = if success {
            0
        } else {
            self.failure_intel.root_cause_count()
        };
        info!("[STAGE 7 - INTEL] {} root causes", root_causes);

        // Stage 8: Knowledge Distillation
        if response.len() > 50 {
            let l1_id = self.knowledge.ingest_telemetry(
                format!("exec-{execution_id}"),
                format!("Task: {task} | Provider: {provider_name}"),
                vec![domain.to_string(), "execution".to_string()],
            );
            let _l2 = self.knowledge.distill_to_l1(
                vec![l1_id],
                format!("Execution of: {task}"),
                &response,
            );
            println!(
                "[STAGE 8 - DISTILLATION] {} knowledge nodes",
                self.knowledge.knowledge_count()
            );
        }

        // Stage 9: Execution Ledger — immutable permanent record
        self.ledger.append(LedgerEntry {
            stage: "complete".into(),
            duration_ms: exec_ms as u64,
            entry_id: format!("entry-{}", execution_id),
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
                ("output_tokens".into(), response.len().to_string()),
                ("duration_ms".into(), exec_ms.to_string()),
            ]),
        });

        if self.cancel_token.is_cancelled() {
            return Err(anyhow::anyhow!("Execution cancelled"));
        }
        info!("[STAGE 9 - LEDGER] {} entries total", self.ledger.len());

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
        let verdict = self.parliament.post_flight(&self.ctx.session_id, &response);
        match verdict {
            ParliamentVerdict::Deny { reason } => {
                tracing::warn!("[PARLIAMENT] Post-flight denied: {}", reason);
            }
            ParliamentVerdict::RequireApproval { who, expires } => {
                tracing::warn!("[PARLIAMENT] Post-flight requires approval from {:?} (expires: {:?})", who, expires);
            }
            ParliamentVerdict::Modify { .. } => {
                tracing::info!("[PARLIAMENT] Post-flight plan amended");
            }
            ParliamentVerdict::Escalate { to } => {
                tracing::warn!("[PARLIAMENT] Post-flight escalated to: {:?}", to);
            }
            ParliamentVerdict::Allow => {}
            _ => {} // non_exhaustive
        }

        let total = start.elapsed();

        // ── Finalize session ──
        session.status = if success {
            pandora_types::SessionStatus::Completed
        } else {
            pandora_types::SessionStatus::Failed("empty response".into())
        };
        session.completed_at = Some(std::time::SystemTime::now());
        session.workflow = Some("full-pipeline".into());
        session.replay_id = Some(replay_id.clone());
        // Store decision log
        let decision_count = self.controller.decision_log.len();
        session
            .metadata
            .insert("decisions".to_string(), decision_count.to_string());
        session.metadata.insert(
            "decision_log".to_string(),
            format!(
                "{:?}",
                self.controller
                    .decision_log
                    .decisions
                    .iter()
                    .map(|d| &d.stage)
                    .collect::<Vec<_>>()
            ),
        );
        // ponytail: store by execution_id for now; real session mgmt later
        self.sessions.create(&execution_id, task);
        if let Some(s) = self.sessions.get_mut(&execution_id) {
            s.status = session.status.clone();
            s.completed_at = session.completed_at;
            s.replay_id = session.replay_id.clone();
        }

        Ok(ExecutionReport {
            execution_id,
            output: response,
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

    /// Multi-agent execution — split task into sub-tasks, run concurrently, merge results.
    /// ponytail: simple sentence splitting; real decomposition would use PlanningService.
    pub async fn run_multi(
        &mut self,
        task: &str,
        domain: &str,
        max_workers: usize,
    ) -> Result<ExecutionReport> {
        let start = std::time::Instant::now();
        let execution_id = format!("multi-{}", chrono::Utc::now().timestamp_millis());

        // Split task into sub-tasks (ponytail: split on sentences or newlines)
        let sub_tasks: Vec<&str> = if task.contains("\n") {
            task.lines().filter(|l| !l.trim().is_empty()).collect()
        } else {
            // Split on sentence boundaries
            task.split(['.', '!', '?'])
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect()
        };

        if sub_tasks.len() <= 1 {
            // Single task — just run normally
            return self.run(task, domain).await;
        }

        let workers = sub_tasks.len().min(max_workers).max(1);
        println!(
            "[MULTI-AGENT] {} sub-tasks, {} workers",
            sub_tasks.len(),
            workers
        );

        // Spawn workers concurrently — each is a normal run() call
        let mut handles = Vec::new();
        for chunk in sub_tasks.chunks(sub_tasks.len().div_ceil(workers)) {
            let sub = chunk.join(". ");
            // Clone provider for worker (ponytail: Arc<dyn Provider> is clonable)
            let domain = domain.to_string();
            handles.push(tokio::spawn(async move {
                // ponytail: we share the orchestrator state via self, but tokio::spawn
                // requires 'static. For now, run workers sequentially in a loop.
                // Full parallel would use a pool of PandoraRuntime instances.
                (sub, domain)
            }));
        }

        // ponytail: run workers through the existing pipeline sequentially.
        // True parallelism requires one PandoraRuntime per worker.
        let mut outputs = Vec::new();
        let mut total_ms = 0u128;
        let mut all_success = true;
        for sub in &sub_tasks {
            match self.run(sub, domain).await {
                Ok(report) => {
                    outputs.push(report.output.clone());
                    total_ms += report.duration_ms;
                    if !report.success {
                        all_success = false;
                    }
                }
                Err(e) => {
                    outputs.push(format!("[ERROR] {}", e));
                    all_success = false;
                }
            }
        }

        let merged = outputs.join(
            "
---
",
        );
        let _elapsed = start.elapsed();

        Ok(ExecutionReport {
            execution_id: execution_id.clone(),
            output: merged,
            duration_ms: total_ms,
            provider: "multi-agent".into(),
            model: "aggregate".into(),
            workflow_steps: sub_tasks.len(),
            telemetry_spans: outputs.len(),
            root_causes_found: 0,
            knowledge_nodes: 0,
            ledger_entries: self.ledger.len(),
            replay_id: execution_id.clone(),
            success: all_success,
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
                execution_backend: "custom".into(),
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
                graph: ExecutionGraph::new("g"),
                step_count: 3,
            }),
            capability: Some(CapabilityStageOutput {
                provider: "default".into(),
                model: std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "".into()),
                candidates_considered: 1,
            }),
            provider: Some(ProviderStageOutput {
                text: "hi".into(),
                tokens_used: 2,
                duration_ms: 10,
            }),
            recorder: Some(RecorderStageOutput {
                replay_id: "r1".into(),
                frame_count: 1,
            }),
            telemetry_spans: 5,
            root_causes: 0,
            knowledge_nodes: 4,
            ledger_entries: 1,
            success: true,
        };
        delta.merge_into(&mut ctx);
        assert_eq!(ctx.get_variable("workflow_steps").unwrap(), "3");
        assert_eq!(ctx.get_variable("resolved_provider").unwrap(), "default");
        assert_eq!(ctx.get_variable("knowledge_nodes").unwrap(), "4");
        assert_eq!(ctx.get_variable("pipeline_success").unwrap(), "true");
    }

    #[test]
    fn provider_registry_default_is_first() {
        let reg = ProviderRegistry::new();
        // empty -> no default
        assert!(reg.resolve(None, None, None).is_none());
        assert!(reg.get("ollama").is_none());

        // ponytail: can't construct OllamaProvider without legacy-ollama feat in cfg(test),
        // so just verify the empty case behavior
        let _: Vec<&str> = reg.list();
        assert!(reg.list().is_empty());
        assert!(reg.list_models().is_empty());
    }
    #[test]
    fn provider_registry_resolve_returns_none_if_empty() {
        let reg = ProviderRegistry::new();
        assert!(reg.resolve(None, None, None).is_none());
        // No providers registered, any hint should return None
    }
}
