//! Every service in Pandora is defined by a contract trait.
//! Implementations are replaceable. Code depends on traits, never on concrete implementations.
//! Each service has exactly one responsibility.

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
    Custom(String),
}

impl ServiceId {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceId::Memory => "memory",
            ServiceId::Execution => "execution",
            ServiceId::Planning => "planning",
            ServiceId::Governance => "governance",
            ServiceId::Evolution => "evolution",
            ServiceId::Identity => "identity",
            ServiceId::Security => "security",
            ServiceId::Provider => "provider",
            ServiceId::Benchmark => "benchmark",
            ServiceId::Scheduler => "scheduler",
            ServiceId::Telemetry => "telemetry",
            ServiceId::Storage => "storage",
            ServiceId::Communication => "communication",
            ServiceId::Custom(s) => s,
        }
    }
}

impl std::fmt::Display for ServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Base trait for all constitutional services.
pub trait Service: Send + Sync + std::fmt::Debug {
    fn service_id(&self) -> ServiceId;
    fn provider_name(&self) -> &str;
    fn version(&self) -> &str;
    fn health(&self) -> Result<(), String> {
        Ok(())
    }
}

// --- Memory Service ---
pub trait MemoryService: Service {
    fn store(&self, namespace: &str, key: &str, value: &[u8]) -> Result<(), String>;
    fn retrieve(&self, namespace: &str, key: &str) -> Result<Option<Vec<u8>>, String>;
    fn forget(&self, namespace: &str, key: &str) -> Result<(), String>;
    fn search(&self, namespace: &str, query: &str) -> Result<Vec<String>, String>;
    fn archive(&self, namespace: &str, key: &str) -> Result<(), String>;
    fn summarize(&self, namespace: &str) -> Result<String, String>;
}

// --- Execution Service ---
pub trait ExecutionService: Service {
    fn spawn(&self, task: &str) -> Result<String, String>;
    fn execute(&self, execution_id: &str, command: &str) -> Result<String, String>;
    fn checkpoint(&self, execution_id: &str) -> Result<(), String>;
    fn restore(&self, execution_id: &str, checkpoint_id: &str) -> Result<(), String>;
    fn teardown(&self, execution_id: &str) -> Result<(), String>;
}

// --- Planning Service ---
pub trait PlanningService: Service {
    fn plan(&self, goal: &str) -> Result<String, String>;
    fn dag(&self, plan_id: &str) -> Result<Vec<String>, String>;
    fn retry_plan(&self, plan_id: &str, failed_step: &str) -> Result<String, String>;
    fn topology(&self, plan_id: &str) -> Result<String, String>;
}

// --- Governance Service ---
pub trait GovernanceService: Service {
    fn evaluate(&self, action: &str, context: &str) -> Result<bool, String>;
    fn audit(&self, action: &str, decision: &str) -> Result<(), String>;
    fn score(&self, target: &str) -> Result<f64, String>;
    fn verify(&self, artifact: &str) -> Result<bool, String>;
}

// --- Evolution Service ---
pub trait EvolutionService: Service {
    fn mutate(&self, gene: &str) -> Result<String, String>;
    fn crossover(&self, parent_a: &str, parent_b: &str) -> Result<String, String>;
    fn select(&self, population: &[String], scores: &[f64]) -> Result<String, String>;
    fn promote(&self, gene: &str) -> Result<(), String>;
}

// --- Identity Service ---
pub trait IdentityService: Service {
    fn persist(&self, identity: &str) -> Result<(), String>;
    fn resurrect(&self, identity: &str) -> Result<String, String>;
    fn fork(&self, identity: &str, name: &str) -> Result<String, String>;
    fn merge(&self, source: &str, target: &str) -> Result<(), String>;
}

// --- Security Service ---
pub trait SecurityService: Service {
    fn authenticate(&self, credentials: &str) -> Result<String, String>;
    fn authorize(&self, principal: &str, action: &str) -> Result<bool, String>;
    fn isolate(&self, context: &str) -> Result<(), String>;
}

// --- Provider Service (Model Providers) ---
pub trait ProviderService: Service {
    fn list_models(&self) -> Result<Vec<String>, String>;
    fn health(&self) -> Result<String, String>;
    fn context_limit(&self, model: &str) -> Result<usize, String>;
    fn cost(&self, model: &str) -> Result<f64, String>;
    fn latency(&self, model: &str) -> Result<f64, String>;
    fn invoke(&self, model: &str, prompt: &str) -> Result<String, String>;
    fn supports_tools(&self) -> bool {
        false
    }
    fn supports_images(&self) -> bool {
        false
    }
    fn supports_reasoning(&self) -> bool {
        false
    }
}

// --- Benchmark Service ---
pub trait BenchmarkService: Service {
    fn record(&self, model: &str, task: &str, score: f64, metadata: &str) -> Result<(), String>;
    fn query(&self, model: &str, task: &str) -> Result<Vec<(String, f64)>, String>;
    fn compare(&self, models: &[String], task: &str) -> Result<Vec<(String, f64)>, String>;
    fn trend(&self, model: &str, task: &str) -> Result<Vec<(String, f64)>, String>;
}

// --- Scheduler Service ---
pub trait SchedulerService: Service {
    fn schedule(&self, spec: &str, action: &str) -> Result<String, String>;
    fn cancel(&self, job_id: &str) -> Result<(), String>;
    fn list(&self) -> Result<Vec<(String, String, String)>, String>;
    fn history(&self, job_id: &str) -> Result<Vec<(String, String)>, String>;
}

// --- Telemetry Service ---
pub trait TelemetryService: Service {
    fn record(&self, metric: &str, value: f64, labels: &str) -> Result<(), String>;
    fn query(&self, metric: &str, filter: &str) -> Result<Vec<(String, f64)>, String>;
    fn aggregate(&self, metric: &str, window: &str) -> Result<f64, String>;
}

// --- Storage Service ---
pub trait StorageService: Service {
    fn read(&self, path: &str) -> Result<Vec<u8>, String>;
    fn write(&self, path: &str, data: &[u8]) -> Result<(), String>;
    fn delete(&self, path: &str) -> Result<(), String>;
    fn list(&self, prefix: &str) -> Result<Vec<String>, String>;
}

// --- Communication Service ---
pub trait CommunicationService: Service {
    fn send(&self, recipient: &str, message: &str) -> Result<(), String>;
    fn broadcast(&self, channel: &str, message: &str) -> Result<(), String>;
    fn subscribe(&self, channel: &str) -> Result<String, String>;
    fn unsubscribe(&self, subscription_id: &str) -> Result<(), String>;
}
