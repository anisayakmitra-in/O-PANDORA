//! Service contract traits — every constitutional service is defined
//! by a replaceable trait. Code depends on traits, never on concrete
//! implementations. Each trait has exactly one responsibility.

/// Identifies which service a component belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServiceId {
    Memory,
    Execution,
    Planning,
    Governance,
    Evolution,
    Identity,
    Security,
    Provider,
    Benchmark,
    Scheduler,
    Telemetry,
    Storage,
    Communication,
    /// User-defined service.
    Custom(String),
}

impl ServiceId {
    /// Human-readable string representation.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Memory => "memory",
            Self::Execution => "execution",
            Self::Planning => "planning",
            Self::Governance => "governance",
            Self::Evolution => "evolution",
            Self::Identity => "identity",
            Self::Security => "security",
            Self::Provider => "provider",
            Self::Benchmark => "benchmark",
            Self::Scheduler => "scheduler",
            Self::Telemetry => "telemetry",
            Self::Storage => "storage",
            Self::Communication => "communication",
            Self::Custom(s) => s,
        }
    }
}

impl std::fmt::Display for ServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Base trait for all constitutional services.
///
/// Every service must be `Send + Sync + Debug` to support concurrent
/// access. Implementations are expected to be long-lived singletons
/// registered with the service registry.
pub trait Service: Send + Sync + std::fmt::Debug {
    /// Unique identifier for this service.
    fn service_id(&self) -> ServiceId;
    /// Provider/implementation name, e.g. "default-memory".
    fn provider_name(&self) -> &str;
    /// Semantic version of this implementation.
    fn version(&self) -> &str;
    /// Health check. Returns `Ok(())` if healthy.
    fn health(&self) -> Result<(), String> {
        Ok(())
    }
}

/// Key-value memory for short-term and long-term storage.
pub trait MemoryService: Service {
    fn store(&self, namespace: &str, key: &str, value: &[u8]) -> Result<(), String>;
    fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<Vec<u8>>, String>;
    fn forget(&self, namespace: &str, key: &str) -> Result<(), String>;
    fn search(&self, namespace: &str, query: &str) -> Result<Vec<String>, String>;
    fn archive(&self, namespace: &str, key: &str) -> Result<(), String>;
    fn summarize(&self, namespace: &str) -> Result<String, String>;
}

/// Execution lifecycle — spawn, run, checkpoint, restore, teardown.
pub trait ExecutionService: Service {
    fn spawn(&self, task: &str) -> Result<String, String>;
    fn execute(&self, execution_id: &str, command: &str) -> Result<String, String>;
    fn checkpoint(&self, execution_id: &str) -> Result<(), String>;
    fn restore(&self, execution_id: &str, checkpoint_id: &str) -> Result<(), String>;
    fn teardown(&self, execution_id: &str) -> Result<(), String>;
}

/// Planning — decompose goals into DAG workflows.
pub trait PlanningService: Service {
    fn plan(&self, goal: &str) -> Result<String, String>;
    fn dag(&self, plan_id: &str) -> Result<Vec<String>, String>;
    fn retry_plan(&self, plan_id: &str, failed_step: &str) -> Result<String, String>;
    fn topology(&self, plan_id: &str) -> Result<String, String>;
}

/// Governance — evaluate, audit, score, and verify actions.
pub trait GovernanceService: Service {
    fn evaluate(&self, action: &str, context: &str) -> Result<bool, String>;
    fn audit(&self, action: &str, decision: &str) -> Result<(), String>;
    fn score(&self, target: &str) -> Result<f64, String>;
    fn verify(&self, artifact: &str) -> Result<bool, String>;
}

/// Identity — persist, resurrect, fork, and merge agent identities.
pub trait IdentityService: Service {
    fn persist(&self, identity: &str) -> Result<(), String>;
    fn resurrect(&self, identity: &str) -> Result<String, String>;
    fn fork(&self, identity: &str, name: &str) -> Result<String, String>;
    fn merge(&self, source: &str, target: &str) -> Result<(), String>;
}

/// Security — authenticate, authorize, and isolate contexts.
pub trait SecurityService: Service {
    fn authenticate(&self, credentials: &str) -> Result<String, String>;
    fn authorize(&self, principal: &str, action: &str) -> Result<bool, String>;
    fn isolate(&self, context: &str) -> Result<(), String>;
}

/// Model provider — list models, check health, invoke inference.
pub trait ProviderService: Service {
    fn list_models(&self) -> Result<Vec<String>, String>;
    fn health(&self) -> Result<String, String>;
    fn context_limit(&self, model: &str) -> Result<usize, String>;
    fn cost(&self, model: &str) -> Result<f64, String>;
    fn latency(&self, model: &str) -> Result<f64, String>;
    fn invoke(&self, model: &str, prompt: &str) -> Result<String, String>;
    /// Whether this provider supports tool/function calls.
    fn supports_tools(&self) -> bool { false }
    /// Whether this provider supports image inputs.
    fn supports_images(&self) -> bool { false }
    /// Whether this provider supports reasoning/chain-of-thought.
    fn supports_reasoning(&self) -> bool { false }
}

/// Benchmark — record and query performance scores.
pub trait BenchmarkService: Service {
    fn record(&self, model: &str, task: &str, score: f64, metadata: &str) -> Result<(), String>;
    fn query(&self, model: &str, task: &str) -> Result<Vec<(String, f64)>, String>;
    fn compare(&self, models: &[String], task: &str) -> Result<Vec<(String, f64)>, String>;
    fn trend(&self, model: &str, task: &str) -> Result<Vec<(String, f64)>, String>;
}

/// Scheduler — schedule, cancel, list, and history for cron-like jobs.
pub trait SchedulerService: Service {
    fn schedule(&self, spec: &str, action: &str) -> Result<String, String>;
    fn cancel(&self, job_id: &str) -> Result<(), String>;
    fn list(&self) -> Result<Vec<(String, String, String)>, String>;
    fn history(&self, job_id: &str) -> Result<Vec<(String, String)>, String>;
}

/// Telemetry — record, query, and aggregate metrics.
pub trait TelemetryService: Service {
    fn record(&self, metric: &str, value: f64, labels: &str) -> Result<(), String>;
    fn query(&self, metric: &str, filter: &str) -> Result<Vec<(String, f64)>, String>;
    fn aggregate(&self, metric: &str, window: &str) -> Result<f64, String>;
}

/// Storage — read, write, delete, list files/blobs.
pub trait StorageService: Service {
    fn read(&self, path: &str) -> Result<Vec<u8>, String>;
    fn write(&self, path: &str, data: &[u8]) -> Result<(), String>;
    fn delete(&self, path: &str) -> Result<(), String>;
    fn list(&self, prefix: &str) -> Result<Vec<String>, String>;
}

/// Communication — send messages, broadcast, subscribe to channels.
pub trait CommunicationService: Service {
    fn send(&self, recipient: &str, message: &str) -> Result<(), String>;
    fn broadcast(&self, channel: &str, message: &str) -> Result<(), String>;
    fn subscribe(&self, channel: &str) -> Result<String, String>;
    fn unsubscribe(&self, subscription_id: &str) -> Result<(), String>;
}
