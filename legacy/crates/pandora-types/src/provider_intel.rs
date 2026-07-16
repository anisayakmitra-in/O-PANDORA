//! Provider Intelligence Engine — continuous provider evaluation.
//!
//! Replaces simple "fastest/cheapest" policies with learned intelligence.
//! Tracks latency, success rate, cost, throughput, and capability coverage
//! for every provider, updated on each execution.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Per-provider intelligence snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderIntel {
    pub provider_id: String,
    pub model: String,

    // Timing
    pub avg_latency_ms: f64,
    pub p50_latency_ms: u64,
    pub p95_latency_ms: u64,
    pub p99_latency_ms: u64,

    // Success
    pub total_requests: u64,
    pub successful_requests: u64,
    pub success_rate: f64,
    pub error_categories: HashMap<String, u64>,

    // Cost
    pub avg_cost_usd: f64,
    pub total_cost_usd: f64,
    pub tokens_per_dollar: f64,

    // Quality
    pub avg_evaluator_score: f64,
    pub hallucination_suspects: u64,
    pub completion_rate: f64, // % of responses that finished (not truncated)

    // Capability
    pub capabilities_tested: Vec<String>,
    pub capability_scores: HashMap<String, f64>,

    // Recency
    pub last_used: SystemTime,
    pub last_failure: Option<SystemTime>,
    pub consecutive_failures: u32,
}

impl Default for ProviderIntel {
    fn default() -> Self {
        Self {
            provider_id: String::new(),
            model: String::new(),
            avg_latency_ms: 0.0,
            p50_latency_ms: 0,
            p95_latency_ms: 0,
            p99_latency_ms: 0,
            total_requests: 0,
            successful_requests: 0,
            success_rate: 1.0,
            error_categories: HashMap::new(),
            avg_cost_usd: 0.0,
            total_cost_usd: 0.0,
            tokens_per_dollar: 0.0,
            avg_evaluator_score: 0.0,
            hallucination_suspects: 0,
            completion_rate: 1.0,
            capabilities_tested: vec![],
            capability_scores: HashMap::new(),
            last_used: SystemTime::UNIX_EPOCH,
            last_failure: None,
            consecutive_failures: 0,
        }
    }
}

/// The intelligence engine — tracks all providers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderIntelligenceEngine {
    pub providers: HashMap<String, ProviderIntel>,
    pub version: u32,
}

impl ProviderIntelligenceEngine {
    pub fn new() -> Self { Self::default() }

    /// Record a successful execution.
    pub fn record_success(
        &mut self,
        provider: &str,
        model: &str,
        latency_ms: u64,
        cost_usd: f64,
        tokens: usize,
        evaluator_score: f64,
    ) {
        let intel = self.ensure(provider, model);
        intel.total_requests += 1;
        intel.successful_requests += 1;
        intel.consecutive_failures = 0;

        // Rolling average latency
        let n = intel.total_requests as f64;
        intel.avg_latency_ms = (intel.avg_latency_ms * (n - 1.0) + latency_ms as f64) / n;

        // Percentiles — simple max-update approach (full histogram needs a library)
        if latency_ms > intel.p95_latency_ms { intel.p95_latency_ms = latency_ms; }
        if latency_ms > intel.p99_latency_ms { intel.p99_latency_ms = latency_ms; }
        if intel.p50_latency_ms == 0 || latency_ms < intel.p50_latency_ms { intel.p50_latency_ms = latency_ms; }

        intel.total_cost_usd += cost_usd;
        intel.avg_cost_usd = intel.total_cost_usd / n;
        if cost_usd > 0.0 && tokens > 0 {
            intel.tokens_per_dollar = tokens as f64 / cost_usd;
        }

        // Evaluator score (rolling average)
        intel.avg_evaluator_score = (intel.avg_evaluator_score * (n - 1.0) + evaluator_score) / n;

        intel.success_rate = intel.successful_requests as f64 / n;
        intel.last_used = SystemTime::now();
    }

    /// Record a failure with category.
    pub fn record_failure(&mut self, provider: &str, model: &str, _latency_ms: u64, error: &str) {
        let intel = self.ensure(provider, model);
        intel.total_requests += 1;
        intel.consecutive_failures += 1;
        *intel.error_categories.entry(error.to_string()).or_default() += 1;
        intel.success_rate = intel.successful_requests as f64 / intel.total_requests as f64;
        intel.last_failure = Some(SystemTime::now());
    }

    /// Score a provider for the given requirements. Higher is better.
    pub fn score(&self, provider: &str, model: &str, _prefer_speed: bool, require_reliability: bool) -> f64 {
        let intel = match self.providers.get(&format!("{}:{}", provider, model)) {
            Some(i) => i,
            None => return 0.0, // Unknown provider
        };

        let mut score = 0.0_f64;

        // Base: success rate (0-30 points)
        score += intel.success_rate * 30.0;

        // Latency: faster is better (0-25 points)
        if intel.avg_latency_ms > 0.0 {
            let latency_score = (1000.0 / intel.avg_latency_ms).min(1.0);
            score += latency_score * 25.0;
        }

        // Cost: cheaper is better — inverted (0-15 points)
        if intel.avg_cost_usd > 0.0 {
            let cost_score = (0.01 / intel.avg_cost_usd).min(1.0);
            score += cost_score * 15.0;
        } else {
            score += 15.0; // Free is great
        }

        // Reliability bonus (0-20 points)
        if require_reliability {
            score += intel.success_rate * 20.0;
            // Penalize recent failures
            score -= (intel.consecutive_failures as f64 * 5.0).min(20.0);
        }

        // Quality bonus — evaluator scores (0-10 points)
        score += intel.avg_evaluator_score * 10.0;

        score.max(0.0)
    }

    /// Find the best provider for given requirements.
    pub fn best(&self, prefer_speed: bool, require_reliability: bool) -> Option<(&str, &str)> {
        self.providers.iter()
            .filter(|(_, i)| i.success_rate > 0.5 && i.consecutive_failures < 3)
            .map(|(key, i)| (key, self.score(&i.provider_id, &i.model, prefer_speed, require_reliability)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(key, _)| {
                let parts: Vec<&str> = key.splitn(2, ':').collect();
                (parts[0], parts.get(1).copied().unwrap_or("unknown"))
            })
    }

    fn ensure(&mut self, provider: &str, model: &str) -> &mut ProviderIntel {
        let key = format!("{}:{}", provider, model);
        self.providers.entry(key).or_insert_with(|| ProviderIntel {
            provider_id: provider.to_string(),
            model: model.to_string(),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_success() {
        let mut engine = ProviderIntelligenceEngine::new();
        engine.record_success("ollama", "llama3.2", 150, 0.0, 500, 0.8);
        let score = engine.score("ollama", "llama3.2", true, false);
        assert!(score > 0.0);
    }

    #[test]
    fn failure_penalty() {
        let mut engine = ProviderIntelligenceEngine::new();
        engine.record_failure("ollama", "llama3.2", 500, "timeout");
        engine.record_failure("ollama", "llama3.2", 500, "timeout");
        engine.record_failure("ollama", "llama3.2", 500, "timeout");
        let best = engine.best(true, true);
        // After 3 consecutive failures, should not be selected
        assert!(best.is_none());
    }

    #[test]
    fn comparative_selection() {
        let mut engine = ProviderIntelligenceEngine::new();
        engine.record_success("fast", "model1", 10, 0.0, 100, 0.9);
        engine.record_success("slow", "model2", 5000, 0.0, 100, 0.9);
        let best = engine.best(true, false);
        assert!(best.is_some());
        // Should prefer fast over slow
        assert_eq!(best.unwrap().0, "fast");
    }
}
