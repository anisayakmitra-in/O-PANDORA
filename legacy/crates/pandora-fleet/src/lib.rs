//! Pandora Fleet — distributed execution infrastructure.
//!
//! Remote workers, fleet orchestration, network scheduling,
//! and distributed memory. Every remote worker runs a standard
//! PandoraRuntime. The FleetController manages the pool.
//!
//! ```text
//! CLI / API → FleetController → NetworkScheduler → RemoteWorker x N
//!                                                       ↓
//!                                              PandoraRuntime

use pandora_types::execution_plan::{ExecutionOutcome, ExecutionPlan};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

// ── Types ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapability {
    pub provider: String,
    pub model: String,
    pub harnesses: Vec<String>,
    pub genes: Vec<String>,
    pub sandbox_level: u8,
    pub max_concurrency: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerHealth {
    Online,
    Busy,
    Degraded(String),
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteWorker {
    pub id: String,
    pub endpoint: String,
    pub capability: WorkerCapability,
    pub health: WorkerHealth,
    pub current_load: usize,
    pub last_seen: SystemTime,
    pub total_executions: u64,
    pub avg_latency_ms: f64,
}

impl RemoteWorker {
    pub fn new(
        id: impl Into<String>,
        endpoint: impl Into<String>,
        capability: WorkerCapability,
    ) -> Self {
        Self {
            id: id.into(),
            endpoint: endpoint.into(),
            capability,
            health: WorkerHealth::Online,
            current_load: 0,
            last_seen: SystemTime::now(),
            total_executions: 0,
            avg_latency_ms: 0.0,
        }
    }
    pub fn can_handle(&self, plan: &ExecutionPlan) -> bool {
        if self.health == WorkerHealth::Offline {
            return false;
        }
        if self.current_load >= self.capability.max_concurrency {
            return false;
        }
        use pandora_types::execution_plan::SandboxLevel as SL;
        if matches!(plan.budget.sandbox_level, SL::Isolated) && self.capability.sandbox_level < 2 {
            return false;
        }
        if matches!(plan.budget.sandbox_level, SL::Restricted) && self.capability.sandbox_level < 1
        {
            return false;
        }
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledTask {
    pub task_id: String,
    pub plan: ExecutionPlan,
    pub assigned_worker: String,
    pub submitted_at: SystemTime,
    pub deadline: SystemTime,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed(ExecutionOutcome),
    Failed(String),
    TimedOut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub key: String,
    pub value: Vec<u8>,
    pub worker_id: String,
    pub stored_at: SystemTime,
    pub ttl: Option<Duration>,
}

// ── DistributedMemory ──

#[derive(Debug)]
pub struct DistributedMemory {
    store: Arc<RwLock<HashMap<String, MemoryEntry>>>,
}

impl DistributedMemory {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub async fn set(&self, key: &str, value: Vec<u8>, worker_id: &str, ttl: Option<Duration>) {
        self.store.write().await.insert(
            key.into(),
            MemoryEntry {
                key: key.into(),
                value,
                worker_id: worker_id.into(),
                stored_at: SystemTime::now(),
                ttl,
            },
        );
    }
    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let store = self.store.read().await;
        store.get(key).and_then(|e| {
            if let Some(ttl) = e.ttl {
                if SystemTime::now()
                    .duration_since(e.stored_at)
                    .unwrap_or(Duration::MAX)
                    > ttl
                {
                    return None;
                }
            }
            Some(e.value.clone())
        })
    }
    pub async fn delete(&self, key: &str) {
        self.store.write().await.remove(key);
    }
    pub async fn keys(&self) -> Vec<String> {
        self.store.read().await.keys().cloned().collect()
    }
    pub async fn entry_count(&self) -> usize {
        self.store.read().await.len()
    }
}
impl Default for DistributedMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ── NetworkScheduler ──

#[derive(Debug)]
pub struct NetworkScheduler {
    workers: Arc<RwLock<HashMap<String, RemoteWorker>>>,
}

impl NetworkScheduler {
    pub fn new() -> Self {
        Self {
            workers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub async fn register(&self, worker: RemoteWorker) {
        self.workers.write().await.insert(worker.id.clone(), worker);
    }
    pub async fn unregister(&self, id: &str) {
        self.workers.write().await.remove(id);
    }
    pub async fn schedule(&self, plan: &ExecutionPlan) -> Option<RemoteWorker> {
        let workers = self.workers.read().await;
        workers
            .values()
            .filter(|w| w.can_handle(plan))
            .max_by(|a, b| {
                let a_score = (a.current_load as f64) * 1000.0 + a.avg_latency_ms;
                let b_score = (b.current_load as f64) * 1000.0 + b.avg_latency_ms;
                b_score.total_cmp(&a_score).reverse()
            })
            .cloned()
    }
    pub async fn list_workers(&self) -> Vec<RemoteWorker> {
        self.workers.read().await.values().cloned().collect()
    }
    pub async fn update_health(&self, id: &str, health: WorkerHealth) {
        if let Some(w) = self.workers.write().await.get_mut(id) {
            w.health = health;
        }
    }
    pub async fn worker_count(&self) -> usize {
        self.workers.read().await.len()
    }
}
impl Default for NetworkScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// ── FleetController ──

#[derive(Debug)]
pub struct FleetController {
    pub scheduler: NetworkScheduler,
    pub memory: DistributedMemory,
    tasks: Arc<RwLock<HashMap<String, ScheduledTask>>>,
}

impl FleetController {
    pub fn new() -> Self {
        Self {
            scheduler: NetworkScheduler::new(),
            memory: DistributedMemory::new(),
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub async fn add_worker(&self, worker: RemoteWorker) {
        self.scheduler.register(worker).await;
    }

    /// Execute a plan across the fleet. Returns immediately with task status;
    /// the caller polls task_status() for completion.
    pub async fn execute(
        &self,
        plan: ExecutionPlan,
    ) -> Result<ScheduledTask, pandora_types::PandoraError> {
        let worker =
            self.scheduler
                .schedule(&plan)
                .await
                .ok_or(pandora_types::PandoraError::NotFound(
                    "No available workers".to_string(),
                ))?;
        let task_id = format!("task-{:016x}", rand::random::<u64>());
        let task = ScheduledTask {
            task_id: task_id.clone(),
            plan,
            assigned_worker: worker.id.clone(),
            submitted_at: SystemTime::now(),
            deadline: SystemTime::now() + Duration::from_secs(300),
            status: TaskStatus::Pending,
        };
        self.tasks.write().await.insert(task_id, task.clone());
        Ok(task)
    }

    /// Mark a task as completed with outcome (called by worker callback or poll).
    pub async fn complete_task(&self, task_id: &str, outcome: ExecutionOutcome) {
        if let Some(t) = self.tasks.write().await.get_mut(task_id) {
            t.status = TaskStatus::Completed(outcome);
        }
    }

    /// Mark a task as failed.
    pub async fn fail_task(&self, task_id: &str, error: &str) {
        if let Some(t) = self.tasks.write().await.get_mut(task_id) {
            t.status = TaskStatus::Failed(error.into());
        }
    }

    pub async fn task_status(&self, task_id: &str) -> Option<ScheduledTask> {
        self.tasks.read().await.get(task_id).cloned()
    }
    pub async fn list_tasks(&self) -> Vec<ScheduledTask> {
        self.tasks.read().await.values().cloned().collect()
    }
    pub async fn worker_count(&self) -> usize {
        self.scheduler.worker_count().await
    }
    pub async fn task_count(&self) -> usize {
        self.tasks.read().await.len()
    }
}
impl Default for FleetController {
    fn default() -> Self {
        Self::new()
    }
}

/// Perform a health check on all workers. Returns (worker_id, health) pairs.
/// Requires the `reqwest` feature for HTTP clients.
#[cfg(feature = "reqwest")]
pub async fn health_check_all(controller: &FleetController) -> Vec<(String, WorkerHealth)> {
    let workers = controller.scheduler.list_workers().await;
    let client = reqwest::Client::new();
    let mut results = Vec::new();
    for w in &workers {
        match client.get(format!("{}/health", w.endpoint)).send().await {
            Ok(r) if r.status().is_success() => {
                controller
                    .scheduler
                    .update_health(&w.id, WorkerHealth::Online)
                    .await;
                results.push((w.id.clone(), WorkerHealth::Online));
            }
            _ => {
                controller
                    .scheduler
                    .update_health(&w.id, WorkerHealth::Offline)
                    .await;
                results.push((w.id.clone(), WorkerHealth::Offline));
            }
        }
    }
    results
}

/// Compatibility entry point for fleet dispatch.
/// Network execution remains disabled until an authenticated protocol is available.
#[cfg(feature = "reqwest")]
pub async fn dispatch_task(
    _controller: &FleetController,
    _task_id: &str,
    _plan: &ExecutionPlan,
    _worker_endpoint: &str,
) -> Result<ExecutionOutcome, pandora_types::PandoraError> {
    Err(pandora_types::PandoraError::Internal(
        "Fleet network execution is disabled pending authenticated protocol support".into(),
    ))
}

// ── Tests ──

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_types::execution_plan::{
        ControlStrategy, EvaluatorKind, ExecutionBudget, StopCondition,
    };

    fn test_plan() -> ExecutionPlan {
        ExecutionPlan {
            instruction: "echo hi".into(),
            control_strategy: ControlStrategy::SingleShot,
            evaluator: EvaluatorKind::None,
            stop_conditions: vec![StopCondition::GoalMet],
            budget: ExecutionBudget::default(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn mem_set_get() {
        let m = DistributedMemory::new();
        m.set("k", b"v".to_vec(), "w1", None).await;
        assert_eq!(m.get("k").await, Some(b"v".to_vec()));
    }
    #[tokio::test]
    async fn mem_delete() {
        let m = DistributedMemory::new();
        m.set("k", b"v".to_vec(), "w1", None).await;
        m.delete("k").await;
        assert_eq!(m.get("k").await, None);
    }
    #[tokio::test]
    async fn mem_ttl() {
        let m = DistributedMemory::new();
        m.set("k", b"v".to_vec(), "w1", Some(Duration::from_millis(1)))
            .await;
        tokio::time::sleep(Duration::from_millis(5)).await;
        assert_eq!(m.get("k").await, None);
    }
    #[tokio::test]
    async fn scheduler_register() {
        let s = NetworkScheduler::new();
        s.register(worker("w1", 4)).await;
        assert_eq!(s.worker_count().await, 1);
    }
    #[tokio::test]
    async fn scheduler_finds_worker() {
        let s = NetworkScheduler::new();
        s.register(worker("w1", 4)).await;
        assert!(s.schedule(&test_plan()).await.is_some());
    }
    #[tokio::test]
    async fn scheduler_skips_full() {
        let s = NetworkScheduler::new();
        let mut w = worker("w1", 1);
        w.current_load = 1;
        s.register(w).await;
        assert!(s.schedule(&test_plan()).await.is_none());
    }
    #[tokio::test]
    async fn fleet_init() {
        let fc = FleetController::new();
        assert_eq!(fc.worker_count().await, 0);
    }
    #[tokio::test]
    async fn fleet_add_worker() {
        let fc = FleetController::new();
        fc.add_worker(worker("w1", 4)).await;
        assert_eq!(fc.worker_count().await, 1);
    }
    #[tokio::test]
    async fn fleet_execute_no_workers() {
        let fc = FleetController::new();
        assert!(fc.execute(test_plan()).await.is_err());
    }

    #[cfg(feature = "reqwest")]
    #[tokio::test]
    async fn network_dispatch_returns_disabled_without_sending() {
        let controller = FleetController::new();
        let error = dispatch_task(&controller, "task-1", &test_plan(), "http://127.0.0.1:9")
            .await
            .expect_err("network dispatch must be disabled");
        assert!(error.to_string().contains("network execution is disabled"));
    }

    fn worker(id: &str, max_conc: usize) -> RemoteWorker {
        RemoteWorker::new(
            id,
            format!("http://localhost:900{}", id.chars().last().unwrap_or('0')),
            WorkerCapability {
                provider: "ollama".into(),
                model: std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "".into()),
                harnesses: vec![],
                genes: vec![],
                sandbox_level: 0,
                max_concurrency: max_conc,
            },
        )
    }
}
pub mod worker;

impl Default for WorkerCapability {
    fn default() -> Self {
        Self {
            provider: String::new(),
            model: String::new(),
            harnesses: vec![],
            genes: vec![],
            sandbox_level: 0,
            max_concurrency: 4,
        }
    }
}
