// ponytail: one file, all default service implementations.
// Contracts in pandora-types, implementations here. Parliament registers these.

use pandora_types::services::*;
use std::collections::HashMap;
use std::sync::Mutex;

// ── Memory Service (in-memory) ──

#[derive(Debug)]
pub struct DefaultMemoryService {
    store: Mutex<HashMap<String, HashMap<String, Vec<u8>>>>,
    provider: String,
    version: String,
}

impl Default for DefaultMemoryService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultMemoryService {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
            provider: "pandora".into(),
            version: "0.1.0".into(),
        }
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
    fn store(&self, namespace: &str, key: &str, value: &[u8]) -> Result<(), String> {
        let mut map = self.store.lock().map_err(|e| e.to_string())?;
        map.entry(namespace.to_string())
            .or_default()
            .insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<Vec<u8>>, String> {
        let map = self.store.lock().map_err(|e| e.to_string())?;
        Ok(map.get(namespace).and_then(|ns| ns.get(key).cloned()))
    }

    fn forget(&self, namespace: &str, key: &str) -> Result<(), String> {
        let mut map = self.store.lock().map_err(|e| e.to_string())?;
        if let Some(ns) = map.get_mut(namespace) {
            ns.remove(key);
        }
        Ok(())
    }

    fn search(&self, namespace: &str, query: &str) -> Result<Vec<String>, String> {
        let map = self.store.lock().map_err(|e| e.to_string())?;
        let q = query.to_lowercase();
        Ok(map
            .get(namespace)
            .map(|ns| {
                ns.keys()
                    .filter(|k| k.to_lowercase().contains(&q))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default())
    }

    fn archive(&self, _namespace: &str, _key: &str) -> Result<(), String> {
        Ok(())
    }
    fn summarize(&self, _namespace: &str) -> Result<String, String> {
        Ok("in-memory".into())
    }
}

// ── Execution Service (real) — with subprocess and checkpoint/restore ──

use std::process::Command;
#[derive(Debug)]
struct ExecState {
    spawned: HashMap<String, String>,
    checkpoints: HashMap<String, Vec<(String, String)>>,
}

#[derive(Debug)]
pub struct DefaultExecutionService {
    provider: String,
    version: String,
    state: Mutex<ExecState>,
}

impl Default for DefaultExecutionService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultExecutionService {
    pub fn new() -> Self {
        Self {
            provider: "pandora".into(),
            version: "0.1.0".into(),
            state: Mutex::new(ExecState {
                spawned: HashMap::new(),
                checkpoints: HashMap::new(),
            }),
        }
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
    fn spawn(&self, task: &str) -> Result<String, String> {
        let id = format!("exec-{}", task.len());
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.spawned.insert(id.clone(), task.to_string());
        Ok(id)
    }

    fn execute(&self, _id: &str, cmd: &str) -> Result<String, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| format!("execution failed: {}", e))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(if stderr.is_empty() {
                format!("exit code: {}", output.status)
            } else {
                stderr.to_string()
            });
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn checkpoint(&self, id: &str) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        let task = state
            .spawned
            .get(id)
            .ok_or(format!("unknown: {}", id))?
            .clone();
        state
            .checkpoints
            .entry(id.to_string())
            .or_default()
            .push((format!("cp-{}", id), task));
        Ok(())
    }

    fn restore(&self, id: &str, _cp: &str) -> Result<(), String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        state
            .checkpoints
            .get(id)
            .ok_or(format!("no cps for {}", id))?;
        Ok(())
    }

    fn teardown(&self, id: &str) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.spawned.remove(id);
        state.checkpoints.remove(id);
        Ok(())
    }
}

// ── Planning Service — generates execution plans ──

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
                description: goal.to_string(),
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

#[derive(Debug)]
pub struct DefaultPlanningService {
    engine: PlanningEngine,
    provider: String,
    version: String,
}

impl Default for DefaultPlanningService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultPlanningService {
    pub fn new() -> Self {
        Self {
            engine: PlanningEngine::new(),
            provider: "pandora".into(),
            version: "0.1.0".into(),
        }
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
    fn plan(&self, goal: &str) -> Result<String, String> {
        let steps = self.engine.decompose(goal);
        let plan_id = format!("plan-{:x}", goal.len());
        let mut map = self.engine.plans.lock().map_err(|e| e.to_string())?;
        map.insert(plan_id.clone(), steps);
        Ok(plan_id)
    }

    fn dag(&self, plan_id: &str) -> Result<Vec<String>, String> {
        let map = self.engine.plans.lock().map_err(|e| e.to_string())?;
        let steps = map
            .get(plan_id)
            .ok_or_else(|| format!("Plan not found: {}", plan_id))?;
        Ok(steps
            .iter()
            .map(|s| format!("{} -> [{}]", s.id, s.depends_on.join(",")))
            .collect())
    }

    fn retry_plan(&self, plan_id: &str, failed_step: &str) -> Result<String, String> {
        let map = self.engine.plans.lock().map_err(|e| e.to_string())?;
        let steps = map
            .get(plan_id)
            .ok_or_else(|| format!("Plan not found: {}", plan_id))?;
        let retry_id = format!("{}-retry", plan_id);
        let mut retry_steps: Vec<PlanStep> = steps
            .iter()
            .skip_while(|s| s.id != failed_step)
            .cloned()
            .collect();
        for s in &mut retry_steps {
            s.id = format!("retry-{}", s.id);
        }
        Ok(format!(
            "Retry plan {}: {} steps",
            retry_id,
            retry_steps.len()
        ))
    }

    fn topology(&self, plan_id: &str) -> Result<String, String> {
        let dag = self.dag(plan_id)?;
        Ok(format!("Plan {}:  {}", plan_id, dag.join("  ")))
    }
}

// ── ExecutionController — owns execution decisions ──
// ponytail: one coherent runtime controller instead of LoopEngine + RetryEngine + DelegationEngine.

use pandora_types::decision::{Decision, DecisionLog};

/// Controls execution flow: decide retry, stop, switch, delegate.
/// Lives inside ExecutionService as its decision-making layer.
#[derive(Debug)]
pub struct ExecutionController {
    pub decision_log: DecisionLog,
    max_retries: u32,
}

impl Default for ExecutionController {
    fn default() -> Self {
        Self::new()
    }
}

impl ExecutionController {
    pub fn new() -> Self {
        Self {
            decision_log: DecisionLog::new(),
            max_retries: 3,
        }
    }

    /// Check if an action requires human approval via Governance service.
    pub fn needs_approval(&self, action: &str) -> bool {
        // ponytail: shell execution and provider writes require approval
        action.contains("shell") || action.contains("write_file") || action.contains("provider")
    }

    /// Evaluate output and return control decision.
    pub fn decide_next(&self, output: &str, attempt: u32) -> &str {
        if self.should_retry(attempt, output) {
            return "retry";
        }
        match self.evaluate(output) {
            Evaluation::Accept => "complete",
            Evaluation::Retry(_) => "retry",
            Evaluation::SwitchProvider(_p) => "failover",
            Evaluation::Escalate(_) => "escalate",
        }
    }

    /// Select a provider based on the plan's provider_policy.
    /// Policies: fastest, cheapest, best, local_only, privacy, balanced
    pub fn select_provider<'a>(&self, policy: &str, candidates: &[(&'a str, u64)]) -> Option<&'a str> {
        if candidates.is_empty() { return None; }
        match policy {
            "local_only" => candidates.iter().find(|(name, _)| name.contains(&"ollama") || name.contains(&"llamacpp")).map(|(n, _)| *n),
            "fastest" => candidates.iter().min_by_key(|(_, lat)| lat).map(|(n, _)| *n),
            "cheapest" => candidates.first().map(|(n, _)| *n),
            "privacy" => candidates.iter().find(|(name, _)| name.contains(&"ollama")).map(|(n, _)| *n),
            "balanced" | _ => candidates.first().map(|(n, _)| *n),
        }
    }

    pub fn set_max_retries(&mut self, n: u32) {
        self.max_retries = n;
    }

    /// Record a decision with alternatives.
    pub fn decide(&mut self, stage: &str, chosen: &str, reason: &str, rejected: Vec<(&str, &str)>) {
        let mut d = Decision::new(stage, chosen, reason);
        for (name, reason) in rejected {
            d = d.reject(name, reason);
        }
        self.decision_log.record(d);
    }

    /// Whether to retry after a failure, based on retry count and output.
    pub fn should_retry(&self, attempt: u32, output: &str) -> bool {
        // ponytail: retry on empty output up to max_retries
        if attempt >= self.max_retries {
            return false;
        }
        output.is_empty() || output.contains("[ERROR]")
    }

    /// Choose a fallback provider when the primary fails.
    pub fn select_fallback<'a>(&self, primary: &str, available: &'a [&str]) -> Option<&'a str> {
        available.iter().find(|&&p| p != primary).copied()
    }

    /// Evaluate if the output meets quality criteria.
    pub fn evaluate(&self, output: &str) -> Evaluation {
        if output.is_empty() {
            return Evaluation::Retry("empty output".into());
        }
        if output.len() < 5 {
            return Evaluation::Retry("output too short".into());
        }
        Evaluation::Accept
    }
}

/// Result of evaluating execution output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evaluation {
    Accept,
    Retry(String),
    SwitchProvider(String),
    Escalate(String),
}

// ── Governance Service — policy evaluation and audit ──

#[derive(Debug)]
pub struct DefaultGovernanceService {
    allowed_actions: Mutex<Vec<String>>,
    audit_log: Mutex<Vec<(String, String)>>,
    provider: String,
    version: String,
}

impl Default for DefaultGovernanceService {
    fn default() -> Self {
        Self::new()
    }
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
            version: "0.1.0".into(),
        }
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
    fn evaluate(&self, action: &str, _context: &str) -> Result<bool, String> {
        let allowed = self.allowed_actions.lock().map_err(|e| e.to_string())?;
        Ok(allowed.iter().any(|a| action.contains(a.as_str())))
    }

    fn audit(&self, action: &str, decision: &str) -> Result<(), String> {
        self.audit_log
            .lock()
            .map_err(|e| e.to_string())?
            .push((action.into(), decision.into()));
        Ok(())
    }

    fn score(&self, target: &str) -> Result<f64, String> {
        let len = target.len() as f64;
        Ok(1.0 - (len / 1000.0).min(0.9))
    }

    fn verify(&self, artifact: &str) -> Result<bool, String> {
        Ok(!artifact.is_empty()
            && artifact
                .chars()
                .all(|c| c.is_alphanumeric() || c.is_whitespace() || ".!?-_/@#".contains(c)))
    }
}

// ── Identity Service — identity lifecycle ──

#[derive(Debug)]
pub struct DefaultIdentityService {
    store: Mutex<HashMap<String, String>>,
    provider: String,
    version: String,
}

impl Default for DefaultIdentityService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultIdentityService {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
            provider: "pandora".into(),
            version: "0.1.0".into(),
        }
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
    fn persist(&self, identity: &str) -> Result<(), String> {
        self.store
            .lock()
            .map_err(|e| e.to_string())?
            .insert(identity.into(), identity.into());
        Ok(())
    }

    fn resurrect(&self, identity: &str) -> Result<String, String> {
        self.store
            .lock()
            .map_err(|e| e.to_string())?
            .get(identity)
            .cloned()
            .ok_or_else(|| format!("Identity not found: {}", identity))
    }

    fn fork(&self, identity: &str, name: &str) -> Result<String, String> {
        let mut store = self.store.lock().map_err(|e| e.to_string())?;
        let original = store
            .get(identity)
            .ok_or_else(|| format!("Identity not found: {}", identity))?
            .clone();
        let forked_id = format!("{}--{}", name, identity);
        store.insert(forked_id.clone(), original);
        Ok(forked_id)
    }

    fn merge(&self, source: &str, target: &str) -> Result<(), String> {
        let mut store = self.store.lock().map_err(|e| e.to_string())?;
        let src = store
            .get(source)
            .ok_or_else(|| format!("Source not found: {}", source))?
            .clone();
        store.insert(target.into(), src);
        Ok(())
    }
}

// ── Workflow Service — sequential workflow execution ──

#[derive(Debug)]
pub struct DefaultWorkflowService {
    provider: String,
    version: String,
}

impl Default for DefaultWorkflowService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultWorkflowService {
    pub fn new() -> Self {
        Self {
            provider: "pandora".into(),
            version: "0.1.0".into(),
        }
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
    fn plan(&self, goal: &str) -> Result<String, String> {
        Ok(format!("wf-{:x}", goal.len()))
    }
    fn dag(&self, plan_id: &str) -> Result<Vec<String>, String> {
        Ok(vec![format!("{}: start -> execute -> finish", plan_id)])
    }
    fn retry_plan(&self, plan_id: &str, _failed_step: &str) -> Result<String, String> {
        Ok(format!("retry-{}", plan_id))
    }
    fn topology(&self, plan_id: &str) -> Result<String, String> {
        Ok(format!("Workflow {}: sequential", plan_id))
    }
}

// ── Provider Registry Service ──

#[derive(Debug)]
pub struct DefaultProviderRegistryService {
    provider: String,
    version: String,
}

impl Default for DefaultProviderRegistryService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultProviderRegistryService {
    pub fn new() -> Self {
        Self {
            provider: "pandora".into(),
            version: "0.1.0".into(),
        }
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
    fn list_models(&self) -> Result<Vec<String>, String> {
        Ok(vec![
            "ollama/qwen2.5-coder:7b".into(),
            "ollama/llama3.2:latest".into(),
            "openai/gpt-4o".into(),
        ])
    }
    fn health(&self) -> Result<String, String> {
        Ok("operational".into())
    }
    fn context_limit(&self, _model: &str) -> Result<usize, String> {
        Ok(8192)
    }
    fn cost(&self, _model: &str) -> Result<f64, String> {
        Ok(0.0)
    }
    fn latency(&self, _model: &str) -> Result<f64, String> {
        Ok(1000.0)
    }
    fn invoke(&self, model: &str, prompt: &str) -> Result<String, String> {
        Ok(format!(
            "[{}] simulated: {}",
            model,
            &prompt[..prompt.len().min(100)]
        ))
    }
}

// ── Scheduler Service — in-memory job scheduling ──

#[derive(Debug)]
pub struct DefaultSchedulerService {
    jobs: Mutex<HashMap<String, (String, String)>>,
    provider: String,
    version: String,
}

impl Default for DefaultSchedulerService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultSchedulerService {
    pub fn new() -> Self {
        Self {
            jobs: Mutex::new(HashMap::new()),
            provider: "pandora".into(),
            version: "0.1.0".into(),
        }
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
    fn schedule(&self, spec: &str, action: &str) -> Result<String, String> {
        let job_id = format!("job-{:x}", spec.len() + action.len());
        self.jobs
            .lock()
            .map_err(|e| e.to_string())?
            .insert(job_id.clone(), (spec.into(), action.into()));
        Ok(job_id)
    }
    fn cancel(&self, job_id: &str) -> Result<(), String> {
        self.jobs
            .lock()
            .map_err(|e| e.to_string())?
            .remove(job_id)
            .ok_or_else(|| format!("Job not found: {}", job_id))?;
        Ok(())
    }
    fn list(&self) -> Result<Vec<(String, String, String)>, String> {
        let jobs = self.jobs.lock().map_err(|e| e.to_string())?;
        Ok(jobs
            .iter()
            .map(|(id, (spec, action))| (id.clone(), spec.clone(), action.clone()))
            .collect())
    }
    fn history(&self, job_id: &str) -> Result<Vec<(String, String)>, String> {
        Ok(vec![(job_id.into(), "scheduled".into())])
    }
}

// ── Ledger Service — in-memory execution log ──

#[derive(Debug)]
pub struct DefaultLedgerService {
    log: Mutex<Vec<String>>,
    provider: String,
    version: String,
}

impl Default for DefaultLedgerService {
    fn default() -> Self {
        Self::new()
    }
}

impl DefaultLedgerService {
    pub fn new() -> Self {
        Self {
            log: Mutex::new(Vec::new()),
            provider: "pandora".into(),
            version: "0.1.0".into(),
        }
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
    fn read(&self, path: &str) -> Result<Vec<u8>, String> {
        let log = self.log.lock().map_err(|e| e.to_string())?;
        Ok(log
            .iter()
            .filter(|e| e.contains(path))
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
            .into_bytes())
    }
    fn write(&self, _path: &str, data: &[u8]) -> Result<(), String> {
        self.log
            .lock()
            .map_err(|e| e.to_string())?
            .push(String::from_utf8_lossy(data).to_string());
        Ok(())
    }
    fn delete(&self, _path: &str) -> Result<(), String> {
        Ok(())
    }
    fn list(&self, _prefix: &str) -> Result<Vec<String>, String> {
        let log = self.log.lock().map_err(|e| e.to_string())?;
        Ok(log
            .iter()
            .map(|e| e.split('\n').next().unwrap_or(e).to_string())
            .collect())
    }
}
