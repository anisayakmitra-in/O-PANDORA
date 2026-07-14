//! Provider database — captures observed metrics for evidence-driven selection.
//! Every provider call records latency, tokens, success, cost, and health.
//! ExecutionController reads from this instead of hardcoded policies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// A single observation of a provider call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderObservation {
    pub provider: String,
    pub model: String,
    pub latency_ms: u64,
    pub tokens_used: usize,
    pub success: bool,
    pub cost_usd: f64,
    pub timestamp: SystemTime,
}

/// Aggregated metrics for a provider+model pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetrics {
    pub provider: String,
    pub model: String,
    pub avg_latency_ms: f64,
    pub avg_tokens_per_sec: f64,
    pub success_rate: f64,
    pub total_calls: u64,
    pub total_cost_usd: f64,
    pub last_seen: SystemTime,
    pub last_healthy: Option<SystemTime>,
}

/// Provider database — accumulates observations, computes metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDb {
    observations: Vec<ProviderObservation>,
    metrics: HashMap<(String, String), ProviderMetrics>,
}

impl ProviderDb {
    pub fn new() -> Self { Self { observations: Vec::new(), metrics: HashMap::new() } }

    /// Record an observation and update metrics.
    pub fn record(&mut self, obs: ProviderObservation) {
        let key = (obs.provider.clone(), obs.model.clone());
        let entry = self.metrics.entry(key).or_insert(ProviderMetrics {
            provider: obs.provider.clone(), model: obs.model.clone(), avg_latency_ms: 0.0,
            avg_tokens_per_sec: 0.0, success_rate: 1.0, total_calls: 0, total_cost_usd: 0.0,
            last_seen: obs.timestamp, last_healthy: if obs.success { Some(obs.timestamp) } else { None },
        });
        let n = entry.total_calls as f64;
        entry.avg_latency_ms = (entry.avg_latency_ms * n + obs.latency_ms as f64) / (n + 1.0);
        let tps = if obs.latency_ms > 0 { obs.tokens_used as f64 / obs.latency_ms as f64 * 1000.0 } else { 0.0 };
        entry.avg_tokens_per_sec = (entry.avg_tokens_per_sec * n + tps) / (n + 1.0);
        entry.total_calls += 1;
        entry.total_cost_usd += obs.cost_usd;
        entry.success_rate = (entry.success_rate * n + if obs.success { 1.0 } else { 0.0 }) / (n + 1.0);
        entry.last_seen = obs.timestamp;
        if obs.success { entry.last_healthy = Some(obs.timestamp); }
        self.observations.push(obs);
    }

    /// Get metrics for a specific provider+model.
    pub fn metrics(&self, provider: &str, model: &str) -> Option<&ProviderMetrics> { self.metrics.get(&(provider.into(), model.into())) }

    /// Get the best provider+model by criteria.
    pub fn best(&self, criteria: &str) -> Option<(&str, &str)> {
        self.metrics.iter().filter(|(_, m)| m.total_calls > 0).max_by(|(_, a), (_, b)| match criteria {
            "fastest" => b.avg_latency_ms.total_cmp(&a.avg_latency_ms),
            "cheapest" => b.total_cost_usd.total_cmp(&a.total_cost_usd),
            "reliable" => b.success_rate.total_cmp(&a.success_rate),
            _ => b.avg_tokens_per_sec.total_cmp(&a.avg_tokens_per_sec),
        }).map(|((p, m), _)| (p.as_str(), m.as_str()))
    }

    pub fn all_metrics(&self) -> Vec<&ProviderMetrics> { self.metrics.values().collect() }
    pub fn observation_count(&self) -> usize { self.observations.len() }
    pub fn is_empty(&self) -> bool { self.metrics.is_empty() }
}

impl Default for ProviderDb { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn record_and_metrics() { let mut db = ProviderDb::new(); db.record(ProviderObservation { provider: "ollama".into(), model: "qwen".into(), latency_ms: 100, tokens_used: 50, success: true, cost_usd: 0.0, timestamp: SystemTime::now() }); let m = db.metrics("ollama", "qwen").unwrap(); assert_eq!(m.total_calls, 1); assert!(m.avg_tokens_per_sec > 0.0); }
    #[test] fn best_by_speed() { let mut db = ProviderDb::new(); db.record(ProviderObservation { provider: "ollama".into(), model: "qwen".into(), latency_ms: 50, tokens_used: 100, success: true, cost_usd: 0.0, timestamp: SystemTime::now() }); db.record(ProviderObservation { provider: "openai".into(), model: "gpt4".into(), latency_ms: 200, tokens_used: 500, success: true, cost_usd: 0.02, timestamp: SystemTime::now() }); let best = db.best("fastest"); assert!(best.is_some()); assert_eq!(best.unwrap().0, "ollama"); }
    #[test] fn empty_db() { let db = ProviderDb::new(); assert!(db.is_empty()); assert!(db.best("fastest").is_none()); }
}
