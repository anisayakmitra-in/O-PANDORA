//! Default service implementations — one file, all services.
//! Contracts in `pandora_types::services`, implementations here.

use pandora_types::services::*;
use std::collections::HashMap;
use std::process::Command;
use std::sync::Mutex;

// ── Memory Service ──

#[derive(Debug)]
pub struct DefaultMemoryService {
    store: Mutex<HashMap<String, HashMap<String, Vec<u8>>>>,
    provider: String,
    version: String,
}

impl DefaultMemoryService {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
            provider: "pandora".into(),
            version: "0.2.0".into(),
        }
    }
}
impl Default for DefaultMemoryService {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for DefaultMemoryService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Memory
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}

impl MemoryService for DefaultMemoryService {
    fn store(
        &self,
        namespace: &str,
        key: &str,
        value: &[u8],
    ) -> Result<(), pandora_types::PandoraError> {
        self.store
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .entry(namespace.into())
            .or_default()
            .insert(key.into(), value.to_vec());
        Ok(())
    }
    fn retrieve(
        &self,
        namespace: &str,
        key: &str,
    ) -> Result<Option<Vec<u8>>, pandora_types::PandoraError> {
        Ok(self
            .store
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .get(namespace)
            .and_then(|ns| ns.get(key).cloned()))
    }
    fn forget(&self, namespace: &str, key: &str) -> Result<(), pandora_types::PandoraError> {
        if let Some(ns) = self
            .store
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .get_mut(namespace)
        {
            ns.remove(key);
        }
        Ok(())
    }
    fn search(
        &self,
        namespace: &str,
        query: &str,
    ) -> Result<Vec<String>, pandora_types::PandoraError> {
        let q = query.to_lowercase();
        Ok(self
            .store
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .get(namespace)
            .map(|ns| {
                ns.keys()
                    .filter(|k| k.to_lowercase().contains(&q))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default())
    }
    fn archive(&self, _ns: &str, _k: &str) -> Result<(), pandora_types::PandoraError> {
        Ok(())
    }
    fn summarize(&self, _ns: &str) -> Result<String, pandora_types::PandoraError> {
        Ok("in-memory".into())
    }
}

// ── Execution Service ──

#[derive(Debug)]
struct ExecState {
    spawned: HashMap<String, String>,
    checkpoints: HashMap<String, Vec<(String, String)>>,
}

#[derive(Debug)]
pub struct DefaultExecutionService {
    state: Mutex<ExecState>,
    provider: String,
    version: String,
}
impl DefaultExecutionService {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(ExecState {
                spawned: HashMap::new(),
                checkpoints: HashMap::new(),
            }),
            provider: "pandora".into(),
            version: "0.2.0".into(),
        }
    }
}
impl Default for DefaultExecutionService {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for DefaultExecutionService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Execution
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}

impl ExecutionService for DefaultExecutionService {
    fn spawn(&self, task: &str) -> Result<String, pandora_types::PandoraError> {
        let id = format!("exec-{}", task.len());
        self.state
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .spawned
            .insert(id.clone(), task.into());
        Ok(id)
    }
    fn execute(&self, _id: &str, cmd: &str) -> Result<String, pandora_types::PandoraError> {
        let out = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| pandora_types::PandoraError::Internal(format!("execution failed: {e}")))?;
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string().into())
        }
    }
    fn checkpoint(&self, id: &str) -> Result<(), pandora_types::PandoraError> {
        let task = self
            .state
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .spawned
            .get(id)
            .ok_or(pandora_types::PandoraError::NotFound(format!(
                "unknown: {id}"
            )))?
            .clone();
        self.state
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .checkpoints
            .entry(id.into())
            .or_default()
            .push((format!("cp-{id}"), task));
        Ok(())
    }
    fn restore(&self, id: &str, _cp: &str) -> Result<(), pandora_types::PandoraError> {
        self.state
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .checkpoints
            .get(id)
            .ok_or(pandora_types::PandoraError::NotFound(format!(
                "no cps for {id}"
            )))?;
        Ok(())
    }
    fn teardown(&self, id: &str) -> Result<(), pandora_types::PandoraError> {
        let mut s = self
            .state
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
        s.spawned.remove(id);
        s.checkpoints.remove(id);
        Ok(())
    }
}

// ── Planning Engine ──

#[derive(Debug, Clone)]
pub struct PlanStep {
    pub id: String,
    pub description: String,
    pub depends_on: Vec<String>,
}

#[derive(Debug)]
pub struct PlanningEngine {
    plans: Mutex<HashMap<String, Vec<PlanStep>>>,
}
impl Default for PlanningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl PlanningEngine {
    pub fn new() -> Self {
        Self {
            plans: Mutex::new(HashMap::new()),
        }
    }
    pub fn decompose(&self, goal: &str) -> Vec<PlanStep> {
        let words: Vec<&str> = goal.split_whitespace().collect();
        if words.len() <= 3 {
            vec![PlanStep {
                id: "step-1".into(),
                description: goal.into(),
                depends_on: vec![],
            }]
        } else {
            let mid = words.len() / 2;
            vec![
                PlanStep {
                    id: "step-1".into(),
                    description: words[..mid].join(" "),
                    depends_on: vec![],
                },
                PlanStep {
                    id: "step-2".into(),
                    description: words[mid..].join(" "),
                    depends_on: vec!["step-1".into()],
                },
            ]
        }
    }
}

// ── Planning Service ──

#[derive(Debug)]
pub struct DefaultPlanningService {
    engine: PlanningEngine,
    provider: String,
    version: String,
}
impl DefaultPlanningService {
    pub fn new() -> Self {
        Self {
            engine: PlanningEngine::new(),
            provider: "pandora".into(),
            version: "0.2.0".into(),
        }
    }
}
impl Default for DefaultPlanningService {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for DefaultPlanningService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Planning
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}

impl PlanningService for DefaultPlanningService {
    fn plan(&self, goal: &str) -> Result<String, pandora_types::PandoraError> {
        let steps = self.engine.decompose(goal);
        let plan_id = format!("plan-{:x}", goal.len());
        self.engine
            .plans
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .insert(plan_id.clone(), steps);
        Ok(plan_id)
    }
    fn dag(&self, plan_id: &str) -> Result<Vec<String>, pandora_types::PandoraError> {
        let guard = self
            .engine
            .plans
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
        let steps = guard.get(plan_id).ok_or_else(|| {
            pandora_types::PandoraError::Internal(format!("Plan not found: {plan_id}"))
        })?;
        Ok(steps
            .iter()
            .map(|s| format!("{} -> [{}]", s.id, s.depends_on.join(",")))
            .collect())
    }
    fn retry_plan(
        &self,
        plan_id: &str,
        failed_step: &str,
    ) -> Result<String, pandora_types::PandoraError> {
        let guard = self
            .engine
            .plans
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
        let steps = guard.get(plan_id).ok_or_else(|| {
            pandora_types::PandoraError::Internal(format!("Plan not found: {plan_id}"))
        })?;
        let retry_steps: Vec<PlanStep> = steps
            .iter()
            .skip_while(|s| s.id != failed_step)
            .cloned()
            .collect();
        Ok(format!("Retry plan {plan_id}: {} steps", retry_steps.len()))
    }
    fn topology(&self, plan_id: &str) -> Result<String, pandora_types::PandoraError> {
        Ok(format!("Plan {plan_id}: {}", self.dag(plan_id)?.join("  ")))
    }
}

// ── ExecutionController ──

use pandora_types::decision::{Decision, DecisionLog};
use pandora_types::provenance::{ExecutionProvenanceGraph, ProvenanceNodeKind};

#[derive(Debug)]
pub struct ExecutionController {
    pub decision_log: DecisionLog,
    pub graph: ExecutionProvenanceGraph,
    max_retries: u32,
}

impl ExecutionController {
    pub fn new() -> Self {
        Self {
            decision_log: DecisionLog::new(),
            graph: ExecutionProvenanceGraph::new("default"),
            max_retries: 3,
        }
    }
    pub fn needs_approval(&self, action: &str) -> bool {
        action.contains("shell") || action.contains("write_file") || action.contains("provider")
    }
    pub fn decide_next(&self, output: &str, attempt: u32) -> &str {
        if self.should_retry(attempt, output) {
            return "retry";
        }
        match self.evaluate(output) {
            Evaluation::Accept(_) => "complete",
            Evaluation::Retry(_) => "retry",
            Evaluation::SwitchProvider(_) => "failover",
            Evaluation::Escalate(_) => "escalate",
        }
    }
    pub fn select_provider<'a>(
        &self,
        policy: &str,
        candidates: &[(&'a str, u64)],
    ) -> Option<&'a str> {
        if candidates.is_empty() {
            return None;
        }
        match policy {
            "local_only" => candidates
                .iter()
                .find(|(n, _)| n.contains("ollama") || n.contains("llamacpp"))
                .map(|(n, _)| *n),
            "fastest" => candidates
                .iter()
                .min_by_key(|(_, lat)| lat)
                .map(|(n, _)| *n),
            "cheapest" => candidates.first().map(|(n, _)| *n),
            "privacy" => candidates
                .iter()
                .find(|(n, _)| n.contains("ollama"))
                .map(|(n, _)| *n),
            _ => candidates.first().map(|(n, _)| *n),
        }
    }
    pub fn start_trace(&mut self, execution_id: &str, task: &str) {
        self.graph = ExecutionProvenanceGraph::new(execution_id);
        let tid = format!("task-{execution_id}");
        let oid = format!("outcome-{execution_id}");
        self.graph.add_node(ProvenanceNodeKind::Task, tid, task);
        self.graph
            .add_node(ProvenanceNodeKind::Outcome, oid, "Pending");
    }

    pub fn finish_trace(&mut self, execution_id: &str, success: bool) {
        let status = if success { "Completed" } else { "Failed" };
        let onid = format!("outcome-{execution_id}");
        if let Some(n) = self.graph.nodes.get_mut(&onid) {
            n.label = status.to_string();
        }
    }

    pub fn set_max_retries(&mut self, n: u32) {
        self.max_retries = n;
    }
    pub fn decide(&mut self, stage: &str, chosen: &str, reason: &str, rejected: Vec<(&str, &str)>) {
        let mut d = Decision::new(stage, chosen, reason);
        for (name, r) in rejected {
            d = d.reject(name, r);
        }
        self.decision_log.record(d);
    }
    pub fn should_retry(&self, attempt: u32, output: &str) -> bool {
        attempt < self.max_retries && (output.is_empty() || output.contains("[ERROR]"))
    }
    pub fn select_fallback<'a>(&self, primary: &str, available: &'a [&str]) -> Option<&'a str> {
        available.iter().find(|&&p| p != primary).copied()
    }
    pub fn evaluate(&self, output: &str) -> Evaluation {
        if output.is_empty() {
            return Evaluation::Retry("empty output".into());
        }
        if output.len() < 5 {
            return Evaluation::Retry("output too short".into());
        }
        Evaluation::Accept(1.0)
    }
}
impl Default for ExecutionController {
    fn default() -> Self {
        Self::new()
    }
}

#[non_exhaustive]
/// Result of evaluating execution output.
#[derive(Debug, Clone, PartialEq)]
pub enum Evaluation {
    Accept(f32),
    Retry(String),
    SwitchProvider(String),
    Escalate(String),
}

// ── Governance Service ──

#[derive(Debug)]
pub struct DefaultGovernanceService {
    allowed_actions: Mutex<Vec<String>>,
    audit_log: Mutex<Vec<(String, String)>>,
    provider: String,
    version: String,
}
impl DefaultGovernanceService {
    pub fn new() -> Self {
        Self {
            allowed_actions: Mutex::new(vec![
                "read".into(),
                "write".into(),
                "execute".into(),
                "list".into(),
                "search".into(),
                "plan".into(),
            ]),
            audit_log: Mutex::new(Vec::new()),
            provider: "pandora".into(),
            version: "0.2.0".into(),
        }
    }
}
impl Default for DefaultGovernanceService {
    fn default() -> Self {
        Self::new()
    }
}
impl Service for DefaultGovernanceService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Governance
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}
impl GovernanceService for DefaultGovernanceService {
    fn evaluate(&self, action: &str, _ctx: &str) -> Result<bool, pandora_types::PandoraError> {
        Ok(self
            .allowed_actions
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .iter()
            .any(|a| action.contains(a.as_str())))
    }
    fn audit(&self, action: &str, decision: &str) -> Result<(), pandora_types::PandoraError> {
        self.audit_log
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .push((action.into(), decision.into()));
        Ok(())
    }
    fn score(&self, target: &str) -> Result<f64, pandora_types::PandoraError> {
        Ok(1.0 - (target.len() as f64 / 1000.0).min(0.9))
    }
    fn verify(&self, artifact: &str) -> Result<bool, pandora_types::PandoraError> {
        Ok(!artifact.is_empty()
            && artifact
                .chars()
                .all(|c| c.is_alphanumeric() || c.is_whitespace() || ".!?-_/@#".contains(c)))
    }
}

// ── Identity Service ──

#[derive(Debug)]
pub struct DefaultIdentityService {
    store: Mutex<HashMap<String, String>>,
    provider: String,
    version: String,
}
impl DefaultIdentityService {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
            provider: "pandora".into(),
            version: "0.2.0".into(),
        }
    }
}
impl Default for DefaultIdentityService {
    fn default() -> Self {
        Self::new()
    }
}
impl Service for DefaultIdentityService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Identity
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}
impl IdentityService for DefaultIdentityService {
    fn persist(&self, identity: &str) -> Result<(), pandora_types::PandoraError> {
        self.store
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .insert(identity.into(), identity.into());
        Ok(())
    }
    fn resurrect(&self, identity: &str) -> Result<String, pandora_types::PandoraError> {
        self.store
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .get(identity)
            .cloned()
            .ok_or(pandora_types::PandoraError::NotFound(format!(
                "Identity not found: {identity}"
            )))
    }
    fn fork(&self, identity: &str, name: &str) -> Result<String, pandora_types::PandoraError> {
        let mut store = self
            .store
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
        let original = store
            .get(identity)
            .ok_or(pandora_types::PandoraError::NotFound(format!(
                "Identity not found: {identity}"
            )))?
            .clone();
        let forked_id = format!("{name}--{identity}");
        store.insert(forked_id.clone(), original);
        Ok(forked_id)
    }
    fn merge(&self, source: &str, target: &str) -> Result<(), pandora_types::PandoraError> {
        let mut store = self
            .store
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
        let src = store
            .get(source)
            .ok_or(pandora_types::PandoraError::NotFound(format!(
                "Source not found: {source}"
            )))?
            .clone();
        store.insert(target.into(), src);
        Ok(())
    }
}

// ── Workflow Service ──

#[derive(Debug)]
pub struct DefaultWorkflowService {
    provider: String,
    version: String,
}
impl DefaultWorkflowService {
    pub fn new() -> Self {
        Self {
            provider: "pandora".into(),
            version: "0.2.0".into(),
        }
    }
}
impl Default for DefaultWorkflowService {
    fn default() -> Self {
        Self::new()
    }
}
impl Service for DefaultWorkflowService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Custom("workflow".into())
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}
impl PlanningService for DefaultWorkflowService {
    fn plan(&self, goal: &str) -> Result<String, pandora_types::PandoraError> {
        Ok(format!("wf-{:x}", goal.len()))
    }
    fn dag(&self, plan_id: &str) -> Result<Vec<String>, pandora_types::PandoraError> {
        Ok(vec![format!("{plan_id}: start -> execute -> finish")])
    }
    fn retry_plan(
        &self,
        plan_id: &str,
        _failed: &str,
    ) -> Result<String, pandora_types::PandoraError> {
        Ok(format!("retry-{plan_id}"))
    }
    fn topology(&self, plan_id: &str) -> Result<String, pandora_types::PandoraError> {
        Ok(format!("Workflow {plan_id}: sequential"))
    }
}

// ── Provider Registry ──

#[derive(Debug)]
pub struct DefaultProviderRegistryService {
    provider: String,
    version: String,
}
impl DefaultProviderRegistryService {
    pub fn new() -> Self {
        Self {
            provider: "pandora".into(),
            version: "0.2.0".into(),
        }
    }
}
impl Default for DefaultProviderRegistryService {
    fn default() -> Self {
        Self::new()
    }
}
impl Service for DefaultProviderRegistryService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Provider
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}
impl ProviderService for DefaultProviderRegistryService {
    fn list_models(&self) -> Result<Vec<String>, pandora_types::PandoraError> {
        Ok(vec!["ollama/default".into()])
    }
    fn health(&self) -> Result<String, pandora_types::PandoraError> {
        Ok("operational".into())
    }
    fn context_limit(&self, _m: &str) -> Result<usize, pandora_types::PandoraError> {
        Ok(8192)
    }
    fn cost(&self, _m: &str) -> Result<f64, pandora_types::PandoraError> {
        Ok(0.0)
    }
    fn latency(&self, _m: &str) -> Result<f64, pandora_types::PandoraError> {
        Ok(1000.0)
    }
    fn invoke(&self, model: &str, prompt: &str) -> Result<String, pandora_types::PandoraError> {
        Ok(format!(
            "[{model}] simulated: {}",
            &prompt[..prompt.len().min(100)]
        ))
    }
}

// ── Scheduler Service ──

#[derive(Debug)]
pub struct DefaultSchedulerService {
    jobs: Mutex<HashMap<String, (String, String)>>,
    provider: String,
    version: String,
}
impl DefaultSchedulerService {
    pub fn new() -> Self {
        Self {
            jobs: Mutex::new(HashMap::new()),
            provider: "pandora".into(),
            version: "0.2.0".into(),
        }
    }
}
impl Default for DefaultSchedulerService {
    fn default() -> Self {
        Self::new()
    }
}
impl Service for DefaultSchedulerService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Scheduler
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}
impl SchedulerService for DefaultSchedulerService {
    fn schedule(&self, spec: &str, action: &str) -> Result<String, pandora_types::PandoraError> {
        let job_id = format!("job-{:x}", spec.len() + action.len());
        self.jobs
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .insert(job_id.clone(), (spec.into(), action.into()));
        Ok(job_id)
    }
    fn cancel(&self, job_id: &str) -> Result<(), pandora_types::PandoraError> {
        self.jobs
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .remove(job_id)
            .ok_or(pandora_types::PandoraError::NotFound(format!(
                "Job not found: {job_id}"
            )))?;
        Ok(())
    }
    fn list(&self) -> Result<Vec<(String, String, String)>, pandora_types::PandoraError> {
        Ok(self
            .jobs
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .iter()
            .map(|(id, (s, a))| (id.clone(), s.clone(), a.clone()))
            .collect())
    }
    fn history(&self, _id: &str) -> Result<Vec<(String, String)>, pandora_types::PandoraError> {
        Ok(vec![])
    }
}

// ── Ledger Service ──

#[derive(Debug)]
pub struct DefaultLedgerService {
    log: Mutex<Vec<String>>,
    provider: String,
    version: String,
}
impl DefaultLedgerService {
    pub fn new() -> Self {
        Self {
            log: Mutex::new(Vec::new()),
            provider: "pandora".into(),
            version: "0.2.0".into(),
        }
    }
}
impl Default for DefaultLedgerService {
    fn default() -> Self {
        Self::new()
    }
}
impl Service for DefaultLedgerService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Storage
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}
impl StorageService for DefaultLedgerService {
    fn read(&self, path: &str) -> Result<Vec<u8>, pandora_types::PandoraError> {
        Ok(self
            .log
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .iter()
            .filter(|e| e.contains(path))
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes())
    }
    fn write(&self, _p: &str, data: &[u8]) -> Result<(), pandora_types::PandoraError> {
        self.log
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .push(String::from_utf8_lossy(data).to_string());
        Ok(())
    }
    fn delete(&self, _p: &str) -> Result<(), pandora_types::PandoraError> {
        Ok(())
    }
    fn list(&self, _p: &str) -> Result<Vec<String>, pandora_types::PandoraError> {
        Ok(self
            .log
            .lock()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?
            .iter()
            .map(|e| e.split('\n').next().unwrap_or(e).to_string())
            .collect())
    }
}
