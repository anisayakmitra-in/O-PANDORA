use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_seconds: u64,
    pub exponential_backoff: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            backoff_seconds: 5,
            exponential_backoff: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionBudget {
    pub max_runtime_seconds: u64,
    pub max_retries: u32,
    pub max_invocations: u64,
}

impl Default for ExecutionBudget {
    fn default() -> Self {
        Self {
            max_runtime_seconds: 300, // 5 minute default watchdog
            max_retries: 3,
            max_invocations: 1, // 1 for one-shot, higher for recurring
        }
    }
}
