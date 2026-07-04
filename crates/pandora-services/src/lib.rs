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
        Self { store: Mutex::new(HashMap::new()), provider: "pandora".into(), version: "0.1.0".into() }
    }
}

impl Service for DefaultMemoryService {
    fn service_id(&self) -> ServiceId { ServiceId::Memory }
    fn provider_name(&self) -> &str { &self.provider }
    fn version(&self) -> &str { &self.version }
}

impl MemoryService for DefaultMemoryService {
    fn store(&self, namespace: &str, key: &str, value: &[u8]) -> Result<(), String> {
        let mut map = self.store.lock().map_err(|e| e.to_string())?;
        map.entry(namespace.to_string()).or_default().insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<Vec<u8>>, String> {
        let map = self.store.lock().map_err(|e| e.to_string())?;
        Ok(map.get(namespace).and_then(|ns| ns.get(key).cloned()))
    }

    fn forget(&self, namespace: &str, key: &str) -> Result<(), String> {
        let mut map = self.store.lock().map_err(|e| e.to_string())?;
        if let Some(ns) = map.get_mut(namespace) { ns.remove(key); }
        Ok(())
    }

    fn search(&self, namespace: &str, query: &str) -> Result<Vec<String>, String> {
        let map = self.store.lock().map_err(|e| e.to_string())?;
        let q = query.to_lowercase();
        Ok(map.get(namespace).map(|ns| {
            ns.keys().filter(|k| k.to_lowercase().contains(&q)).cloned().collect()
        }).unwrap_or_default())
    }

    fn archive(&self, _namespace: &str, _key: &str) -> Result<(), String> { Ok(()) }
    fn summarize(&self, _namespace: &str) -> Result<String, String> { Ok("in-memory".into()) }
}

// ── Execution Service (stub) ──

#[derive(Debug)]
pub struct DefaultExecutionService {
    provider: String,
    version: String,
}

impl DefaultExecutionService {
    pub fn new() -> Self { Self { provider: "pandora".into(), version: "0.1.0".into() } }
}

impl Service for DefaultExecutionService {
    fn service_id(&self) -> ServiceId { ServiceId::Execution }
    fn provider_name(&self) -> &str { &self.provider }
    fn version(&self) -> &str { &self.version }
}

impl ExecutionService for DefaultExecutionService {
    fn spawn(&self, task: &str) -> Result<String, String> { Ok(format!("exec-{}", task.len())) }
    fn execute(&self, id: &str, cmd: &str) -> Result<String, String> { Ok(format!("{}: {}", id, cmd)) }
    fn checkpoint(&self, _id: &str) -> Result<(), String> { Ok(()) }
    fn restore(&self, _id: &str, _cp: &str) -> Result<(), String> { Ok(()) }
    fn teardown(&self, _id: &str) -> Result<(), String> { Ok(()) }
}

// ── Planning Service (stub) ──

#[derive(Debug)]
pub struct DefaultPlanningService {
    provider: String,
    version: String,
}

impl DefaultPlanningService {
    pub fn new() -> Self { Self { provider: "pandora".into(), version: "0.1.0".into() } }
}

impl Service for DefaultPlanningService {
    fn service_id(&self) -> ServiceId { ServiceId::Planning }
    fn provider_name(&self) -> &str { &self.provider }
    fn version(&self) -> &str { &self.version }
}

impl PlanningService for DefaultPlanningService {
    fn plan(&self, goal: &str) -> Result<String, String> { Ok(format!("plan-{}", goal.len())) }
    fn dag(&self, plan_id: &str) -> Result<Vec<String>, String> { Ok(vec![plan_id.to_string()]) }
    fn retry_plan(&self, pid: &str, _step: &str) -> Result<String, String> { Ok(pid.to_string()) }
    fn topology(&self, plan_id: &str) -> Result<String, String> { Ok(plan_id.to_string()) }
}

// ── Governance Service (stub) ──

#[derive(Debug)]
pub struct DefaultGovernanceService {
    provider: String,
    version: String,
}

impl DefaultGovernanceService {
    pub fn new() -> Self { Self { provider: "pandora".into(), version: "0.1.0".into() } }
}

impl Service for DefaultGovernanceService {
    fn service_id(&self) -> ServiceId { ServiceId::Governance }
    fn provider_name(&self) -> &str { &self.provider }
    fn version(&self) -> &str { &self.version }
}

impl GovernanceService for DefaultGovernanceService {
    fn evaluate(&self, _action: &str, _ctx: &str) -> Result<bool, String> { Ok(true) }
    fn audit(&self, _action: &str, _decision: &str) -> Result<(), String> { Ok(()) }
    fn score(&self, _target: &str) -> Result<f64, String> { Ok(0.5) }
    fn verify(&self, _artifact: &str) -> Result<bool, String> { Ok(true) }
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
}
