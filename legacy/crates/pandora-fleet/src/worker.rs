//! Fleet worker contracts. Network execution is disabled until an authenticated protocol is available.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCapability {
    pub provider: String,
    pub models: Vec<String>,
    pub sandbox_level: u8,
    pub max_concurrency: usize,
}

impl Default for WorkerCapability {
    fn default() -> Self {
        Self {
            provider: "ollama".into(),
            models: vec![
                std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "ollama/default".into())
            ],
            sandbox_level: 0,
            max_concurrency: 4,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkerHealth {
    pub status: String,
    pub uptime_secs: u64,
    pub tasks_completed: u64,
}

pub struct WorkerState {
    pub capability: WorkerCapability,
    pub uptime: std::time::Instant,
    pub completed: std::sync::atomic::AtomicU64,
}

pub async fn serve_worker(_addr: &str, _cap: WorkerCapability) -> Result<(), anyhow::Error> {
    anyhow::bail!("Fleet network execution is disabled pending authenticated protocol support")
}

#[cfg(test)]
mod tests {
    use super::{serve_worker, WorkerCapability};

    #[tokio::test]
    async fn network_execution_returns_disabled_without_binding() {
        let result = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            serve_worker("127.0.0.1:0", WorkerCapability::default()),
        )
        .await
        .expect("disabled worker must return without serving");
        let error = result.expect_err("network worker must be disabled");
        assert!(error.to_string().contains("network execution is disabled"));
    }
}
