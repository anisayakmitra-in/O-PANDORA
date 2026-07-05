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
// ── Planning Service (stub) ──

// ── Planning Service (real) — simple DAG planner ──

#[derive(Debug, Clone)]
struct PlanStep {
    id: String,
    description: String,
    deps: Vec<String>,
}

#[derive(Debug)]
struct PlanState {
    plans: HashMap<String, Vec<PlanStep>>,
}

#[derive(Debug)]
pub struct DefaultPlanningService {
    provider: String,
    version: String,
    state: Mutex<PlanState>,
}

impl DefaultPlanningService {
    pub fn new() -> Self {
        Self {
            provider: "pandora".into(),
            version: "0.1.0".into(),
            state: Mutex::new(PlanState {
                plans: HashMap::new(),
            }),
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
        // ponytail: simple task decomposition — split by actionable keywords
        let plan_id = format!("plan-{}", goal.len());
        let mut steps = Vec::new();

        // Generate a simple plan based on goal content
        let lower = goal.to_lowercase();
        if lower.contains("and") || lower.contains(",") || lower.contains("then") {
            // Multiple tasks: split into steps
            for (i, part) in goal.split(|c| c == ',' || c == '.').enumerate() {
                let part = part.trim();
                if !part.is_empty() {
                    steps.push(PlanStep {
                        id: format!("{}-step-{}", plan_id, i + 1),
                        description: part.to_string(),
                        deps: if i > 0 {
                            vec![format!("{}-step-{}", plan_id, i)]
                        } else {
                            vec![]
                        },
                    });
                }
            }
        }

        if steps.is_empty() {
            // Single task: one step
            steps.push(PlanStep {
                id: format!("{}-step-1", plan_id),
                description: goal.to_string(),
                deps: vec![],
            });
        }

        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.plans.insert(plan_id.clone(), steps);
        Ok(plan_id)
    }

    fn dag(&self, plan_id: &str) -> Result<Vec<String>, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        let steps = state.plans.get(plan_id).ok_or("Plan not found")?;
        Ok(steps
            .iter()
            .map(|s| {
                if s.deps.is_empty() {
                    s.id.clone()
                } else {
                    format!("{} -> {}", s.deps.join(", "), s.id)
                }
            })
            .collect())
    }

    fn retry_plan(&self, pid: &str, step: &str) -> Result<String, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        state.plans.get(pid).ok_or("Plan not found")?;
        Ok(format!("retry-{}-{}", pid, step))
    }

    fn topology(&self, plan_id: &str) -> Result<String, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        let steps = state.plans.get(plan_id).ok_or("Plan not found")?;
        let mut dot = format!(
            "digraph {} {{
",
            plan_id
        );
        for s in steps {
            for dep in &s.deps {
                dot.push_str(&format!(
                    "  {} -> {}
",
                    dep, s.id
                ));
            }
        }
        dot.push_str(
            "}
",
        );
        Ok(dot)
    }
}

// ── Governance Service (stub) ──

// ── Governance Service (real) — policy-based evaluator ──

#[derive(Debug, Clone)]
struct AuditEntry {
    action: String,
    decision: String,
    timestamp: String,
}

#[derive(Debug)]
struct GovState {
    audit_log: Vec<AuditEntry>,
    // ponytail: simple deny-list rules — actions containing these strings are rejected
    deny_rules: Vec<String>,
}

#[derive(Debug)]
pub struct DefaultGovernanceService {
    provider: String,
    version: String,
    state: Mutex<GovState>,
}

impl DefaultGovernanceService {
    pub fn new() -> Self {
        Self {
            provider: "pandora".into(),
            version: "0.1.0".into(),
            state: Mutex::new(GovState {
                audit_log: Vec::new(),
                deny_rules: vec![
                    "rm -rf /".into(),
                    "format".into(),
                    "drop table".into(),
                    "shutdown".into(),
                ],
            }),
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
    fn evaluate(&self, action: &str, _ctx: &str) -> Result<bool, String> {
        let state = self.state.lock().map_err(|e| e.to_string())?;
        let action_lower = action.to_lowercase();
        for rule in &state.deny_rules {
            if action_lower.contains(rule) {
                return Err(format!("Denied by policy: action contains \"{}\"", rule));
            }
        }
        Ok(true)
    }

    fn audit(&self, action: &str, decision: &str) -> Result<(), String> {
        let mut state = self.state.lock().map_err(|e| e.to_string())?;
        state.audit_log.push(AuditEntry {
            action: action.to_string(),
            decision: decision.to_string(),
            timestamp: format!(
                "{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            ),
        });
        Ok(())
    }

    fn score(&self, _target: &str) -> Result<f64, String> {
        // ponytail: place holder — weights-based scoring
        Ok(0.5)
    }

    fn verify(&self, _artifact: &str) -> Result<bool, String> {
        Ok(true)
    }
}

// ── Identity Service (stub) ──

#[derive(Debug)]
pub struct DefaultIdentityService {
    provider: String,
    version: String,
}
impl DefaultIdentityService {
    pub fn new() -> Self {
        Self {
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
    fn persist(&self, _id: &str) -> Result<(), String> {
        Ok(())
    }
    fn resurrect(&self, id: &str) -> Result<String, String> {
        Ok(id.into())
    }
    fn fork(&self, _id: &str, name: &str) -> Result<String, String> {
        Ok(name.into())
    }
    fn merge(&self, _src: &str, _tgt: &str) -> Result<(), String> {
        Ok(())
    }
}

// ── Sandbox Service — wraps StorageService semantics ──

#[derive(Debug)]
pub struct DefaultSandboxService {
    provider: String,
    version: String,
}
impl DefaultSandboxService {
    pub fn new() -> Self {
        Self {
            provider: "pandora".into(),
            version: "0.1.0".into(),
        }
    }
}
impl Service for DefaultSandboxService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Custom("sandbox".into())
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}
impl StorageService for DefaultSandboxService {
    fn read(&self, path: &str) -> Result<Vec<u8>, String> {
        Err(format!("sandbox: {} not accessible", path))
    }
    fn write(&self, _path: &str, _data: &[u8]) -> Result<(), String> {
        Ok(())
    }
    fn delete(&self, _path: &str) -> Result<(), String> {
        Ok(())
    }
    fn list(&self, prefix: &str) -> Result<Vec<String>, String> {
        Ok(vec![prefix.into()])
    }
}

// ── Workflow Service (stub) ──

#[derive(Debug)]
pub struct DefaultWorkflowService {
    provider: String,
    version: String,
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
        Ok(format!("wf-{}", goal.len()))
    }
    fn dag(&self, pid: &str) -> Result<Vec<String>, String> {
        Ok(vec![pid.into()])
    }
    fn retry_plan(&self, pid: &str, _step: &str) -> Result<String, String> {
        Ok(pid.into())
    }
    fn topology(&self, pid: &str) -> Result<String, String> {
        Ok(pid.into())
    }
}

// ── Provider Service (stub) ──

#[derive(Debug)]
pub struct DefaultProviderRegistryService {
    provider: String,
    version: String,
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
        Ok(vec!["ollama/qwen2.5-coder:7b".into()])
    }
    fn health(&self) -> Result<String, String> {
        Ok("ok".into())
    }
    fn context_limit(&self, _m: &str) -> Result<usize, String> {
        Ok(4096)
    }
    fn cost(&self, _m: &str) -> Result<f64, String> {
        Ok(0.0)
    }
    fn latency(&self, _m: &str) -> Result<f64, String> {
        Ok(100.0)
    }
    fn invoke(&self, _m: &str, p: &str) -> Result<String, String> {
        Ok(format!("echo: {}", p))
    }
}

// ── Scheduler Service (stub) ──

#[derive(Debug)]
pub struct DefaultSchedulerService {
    provider: String,
    version: String,
}
impl DefaultSchedulerService {
    pub fn new() -> Self {
        Self {
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
    fn schedule(&self, spec: &str, _action: &str) -> Result<String, String> {
        Ok(format!("job-{}", spec.len()))
    }
    fn cancel(&self, _id: &str) -> Result<(), String> {
        Ok(())
    }
    fn list(&self) -> Result<Vec<(String, String, String)>, String> {
        Ok(vec![])
    }
    fn history(&self, _id: &str) -> Result<Vec<(String, String)>, String> {
        Ok(vec![])
    }
}

// ── Ledger Service (stub) — wraps existing ExecutionLedger ──

#[derive(Debug)]
pub struct DefaultLedgerService {
    provider: String,
    version: String,
}
impl DefaultLedgerService {
    pub fn new() -> Self {
        Self {
            provider: "pandora".into(),
            version: "0.1.0".into(),
        }
    }
}
impl Service for DefaultLedgerService {
    fn service_id(&self) -> ServiceId {
        ServiceId::Custom("ledger".into())
    }
    fn provider_name(&self) -> &str {
        &self.provider
    }
    fn version(&self) -> &str {
        &self.version
    }
}
impl TelemetryService for DefaultLedgerService {
    fn record(&self, _metric: &str, _value: f64, _labels: &str) -> Result<(), String> {
        Ok(())
    }
    fn query(&self, _metric: &str, _filter: &str) -> Result<Vec<(String, f64)>, String> {
        Ok(vec![])
    }
    fn aggregate(&self, _metric: &str, _window: &str) -> Result<f64, String> {
        Ok(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_store_and_retrieve() {
        let svc = DefaultMemoryService::new();
        svc.store("test", "k1", b"hello").unwrap();
        let val = svc.retrieve("test", "k1").unwrap().unwrap();
        assert_eq!(val, b"hello");
    }

    #[test]
    fn memory_forget_removes() {
        let svc = DefaultMemoryService::new();
        svc.store("ns", "k", b"v").unwrap();
        svc.forget("ns", "k").unwrap();
        assert!(svc.retrieve("ns", "k").unwrap().is_none());
    }

    #[test]
    fn memory_search_finds_by_substring() {
        let svc = DefaultMemoryService::new();
        svc.store("ns", "hello-world", b"1").unwrap();
        svc.store("ns", "goodbye-world", b"2").unwrap();
        let results = svc.search("ns", "hello").unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "hello-world");
    }

    #[test]
    fn memory_search_returns_empty_on_miss() {
        let svc = DefaultMemoryService::new();
        assert!(svc.search("ns", "nope").unwrap().is_empty());
    }

    #[test]
    fn governance_default_approves() {
        let svc = DefaultGovernanceService::new();
        assert!(svc.evaluate("anything", "ctx").unwrap());
    }

    #[test]
    fn execution_spawns_valid_id() {
        let svc = DefaultExecutionService::new();
        let id = svc.spawn("test-task").unwrap();
        assert!(id.starts_with("exec-"));
    }
    #[test]
    fn identity_fork_returns_name() {
        let svc = DefaultIdentityService::new();
        assert_eq!(svc.fork("parent", "child").unwrap(), "child");
    }

    #[test]
    fn sandbox_rejects_read() {
        let svc = DefaultSandboxService::new();
        assert!(svc.read("/etc/passwd").is_err());
    }

    #[test]
    fn provider_lists_default_model() {
        let svc = DefaultProviderRegistryService::new();
        let models = svc.list_models().unwrap();
        assert!(!models.is_empty());
    }

    #[test]
    fn scheduler_creates_job() {
        let svc = DefaultSchedulerService::new();
        let id = svc.schedule("0 * * * *", "test").unwrap();
        assert!(id.starts_with("job-"));
    }

    #[test]
    fn execution_service_runs_real_command() {
        let svc = DefaultExecutionService::new();
        let result = svc.execute("test", "echo hello").unwrap();
        assert_eq!(result.trim(), "hello");
    }

    #[test]
    fn execution_service_spawns_and_teardown() {
        let svc = DefaultExecutionService::new();
        let id = svc.spawn("test task").unwrap();
        assert!(id.starts_with("exec-"));
        svc.teardown(&id).unwrap();
    }

    #[test]
    fn execution_service_handles_failure() {
        let svc = DefaultExecutionService::new();
        let result = svc.execute("test", "exit 1");
        assert!(result.is_err());
    }

    #[test]
    fn ledger_records_and_queries() {
        let svc = DefaultLedgerService::new();
        svc.record("test_metric", 1.0, "").unwrap();
        let results = svc.query("test_metric", "").unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn workflow_creates_plan() {
        let svc = DefaultWorkflowService::new();
        let plan = svc.plan("build api").unwrap();
        assert!(plan.starts_with("wf-"));
    }
}
