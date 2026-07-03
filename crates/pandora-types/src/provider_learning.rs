//! Provider Learning Engine — continuous empirical model evaluation.
//!
//! Every execution updates model profiles with per-domain scores.
//! Feeds the Capability Resolution Engine with live data.
//! Separate from the Benchmark Engine (controlled tests) vs Provider Learning (real usage).
//!
//! Tracked domains per model:
//!   Coding, Research, Math, EDA, Vision, Robotics, Embedded, Firmware,
//!   Reasoning, Planning, ToolUse, Cost, Latency, Reliability, ContextQuality

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// A single observation from one execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelObservation {
    pub model: String,
    pub provider: String,
    pub domain: String,
    pub score: f64,
    pub latency_ms: f64,
    pub cost: f64,
    pub tokens_used: usize,
    pub success: bool,
    pub retries: u32,
    pub timestamp: String,
}

impl ModelObservation {
    pub fn new(model: impl Into<String>, provider: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            provider: provider.into(),
            domain: domain.into(),
            score: 0.0,
            latency_ms: 0.0,
            cost: 0.0,
            tokens_used: 0,
            success: true,
            retries: 0,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Aggregated profile for a model across all observed domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProfile {
    pub model: String,
    pub provider: String,
    /// Per-domain scores (0-100).
    pub domain_scores: HashMap<String, f64>,
    /// Per-domain observation counts.
    pub domain_counts: HashMap<String, u64>,
    /// Average latency across all domains.
    pub avg_latency_ms: f64,
    /// Average cost per token.
    pub avg_cost_per_token: f64,
    /// Overall success rate (0.0 - 1.0).
    pub success_rate: f64,
    /// Total observations.
    pub total_observations: u64,
    /// Most recent observation timestamp.
    pub last_updated: String,
}

impl ModelProfile {
    pub fn new(model: impl Into<String>, provider: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            provider: provider.into(),
            domain_scores: HashMap::new(),
            domain_counts: HashMap::new(),
            avg_latency_ms: 0.0,
            avg_cost_per_token: 0.0,
            success_rate: 1.0,
            total_observations: 0,
            last_updated: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn score_for(&self, domain: &str) -> Option<f64> {
        self.domain_scores.get(domain).copied()
    }

    pub fn confidence(&self, domain: &str) -> f64 {
        let count = self.domain_counts.get(domain).copied().unwrap_or(0);
        (count as f64 / (count as f64 + 10.0)).min(1.0) // asymptotic to 1.0
    }
}

/// The Provider Learning Engine — observes every execution and builds model profiles.
#[derive(Debug, Clone)]
pub struct ProviderLearningEngine {
    /// All known model profiles, keyed by model name.
    profiles: HashMap<String, ModelProfile>,
    /// Recent observations for trend analysis.
    recent_observations: Vec<ModelObservation>,
    max_observations: usize,
}

impl ProviderLearningEngine {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            recent_observations: Vec::new(),
            max_observations: 10_000,
        }
    }

    pub fn with_max_observations(max: usize) -> Self {
        Self { profiles: HashMap::new(), recent_observations: Vec::new(), max_observations: max }
    }

    /// Record an observation from an execution.
    pub fn observe(&mut self, obs: ModelObservation) {
        let model = obs.model.clone();
        let domain = obs.domain.clone();
        let score = obs.score;

        // Trim recent observations if needed
        self.recent_observations.push(obs);
        while self.recent_observations.len() > self.max_observations {
            self.recent_observations.remove(0);
        }

        // Update profile
        let profile = self.profiles.entry(model).or_insert_with(|| {
            ModelProfile::new(&domain, "unknown")
        });

        profile.total_observations += 1;
        profile.last_updated = chrono::Utc::now().to_rfc3339();

        // Update domain score (running average)
        let count = profile.domain_counts.get(&domain).copied().unwrap_or(0);
        let current = profile.domain_scores.get(&domain).copied().unwrap_or(score);
        let new_score = if count == 0 {
            score
        } else {
            (current * count as f64 + score) / (count as f64 + 1.0)
        };
        profile.domain_scores.insert(domain.clone(), new_score);
        profile.domain_counts.insert(domain, count + 1);
    }

    /// Get the profile for a specific model.
    pub fn profile(&self, model: &str) -> Option<&ModelProfile> {
        self.profiles.get(model)
    }

    /// Rank all known models for a specific domain.
    pub fn rank_for_domain(&self, domain: &str) -> Vec<(String, f64, f64)> {
        let mut results: Vec<(String, f64, f64)> = self.profiles.values()
            .filter_map(|p| p.score_for(domain).map(|s| (p.model.clone(), s, p.confidence(domain))))
            .collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Rank models by weighted combination of domain scores.
    pub fn rank_weighted(&self, weights: &HashMap<String, f64>) -> Vec<(String, f64)> {
        let mut results: Vec<(String, f64)> = self.profiles.values()
            .map(|p| {
                let total_weight: f64 = weights.iter()
                    .map(|(domain, w)| p.score_for(domain).unwrap_or(50.0) * w)
                    .sum();
                let weight_sum: f64 = weights.values().sum();
                let score = if weight_sum > 0.0 { total_weight / weight_sum } else { 0.0 };
                (p.model.clone(), score)
            })
            .collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    /// Get all known models.
    pub fn known_models(&self) -> Vec<&str> {
        self.profiles.keys().map(|s| s.as_str()).collect()
    }

    /// Get trend for a model in a domain (recent N scores).
    pub fn trend(&self, model: &str, domain: &str, count: usize) -> Vec<(String, f64)> {
        self.recent_observations.iter()
            .filter(|o| o.model == model && o.domain == domain)
            .rev()
            .take(count)
            .map(|o| (o.timestamp.clone(), o.score))
            .collect()
    }

    /// Total observations across all models.
    pub fn total_observations(&self) -> u64 {
        self.profiles.values().map(|p| p.total_observations).sum()
    }

    /// Number of models tracked.
    pub fn model_count(&self) -> usize {
        self.profiles.len()
    }
}

impl Default for ProviderLearningEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_observations() -> ProviderLearningEngine {
        let mut engine = ProviderLearningEngine::new();
        let models = vec!["claude-sonnet-4", "qwen3-coder", "gpt-4o", "deepseek-coder-v2"];
        let domains = vec!["coding", "reasoning", "rust", "research"];

        for (mi, model) in models.iter().enumerate() {
            for domain in &domains {
                let mut obs = ModelObservation::new(*model, "test", *domain);
                obs.score = 70.0 + (mi as f64 * 5.0) + (domain.len() as f64 % 15.0);
                obs.success = mi < 3; // last model has failures
                obs.latency_ms = 100.0 + (mi as f64 * 50.0);
                // Add multiple observations
                for _ in 0..3 {
                    engine.observe(obs.clone());
                }
            }
        }
        engine
    }

    #[test]
    fn observe_creates_profile() {
        let mut engine = ProviderLearningEngine::new();
        let obs = ModelObservation::new("claude-sonnet-4", "anthropic", "coding");
        engine.observe(obs);
        assert_eq!(engine.model_count(), 1);
        assert!(engine.profile("claude-sonnet-4").is_some());
    }

    #[test]
    fn rank_for_domain_returns_sorted() {
        let engine = sample_observations();
        let ranked = engine.rank_for_domain("rust");
        assert!(!ranked.is_empty());
        // Highest score first
        for i in 0..ranked.len().saturating_sub(1) {
            assert!(ranked[i].1 >= ranked[i+1].1);
        }
    }

    #[test]
    fn trend_returns_recent_scores() {
        let mut engine = ProviderLearningEngine::new();
        for i in 0..5 {
            let mut obs = ModelObservation::new("qwen", "ollama", "coding");
            obs.score = 80.0 + i as f64;
            engine.observe(obs);
        }
        let trend = engine.trend("qwen", "coding", 3);
        assert_eq!(trend.len(), 3);
        // Most recent (highest score) first
        assert!(trend[0].1 >= trend[trend.len()-1].1);
    }

    #[test]
    fn confidence_increases_with_observations() {
        let mut engine = ProviderLearningEngine::new();
        let mut obs = ModelObservation::new("test-model", "test", "math");
        obs.score = 85.0;
        engine.observe(obs.clone());
        let c1 = engine.profile("test-model").unwrap().confidence("math");

        for _ in 0..20 {
            engine.observe(obs.clone());
        }
        let c2 = engine.profile("test-model").unwrap().confidence("math");
        assert!(c2 > c1);
    }

    #[test]
    fn weighted_ranking_works() {
        let engine = sample_observations();
        let mut weights = HashMap::new();
        weights.insert("coding".to_string(), 0.6);
        weights.insert("reasoning".to_string(), 0.4);
        let ranked = engine.rank_weighted(&weights);
        assert!(!ranked.is_empty());
    }

    #[test]
    fn empty_engine_returns_nothing() {
        let engine = ProviderLearningEngine::new();
        assert!(engine.known_models().is_empty());
        assert!(engine.rank_for_domain("any").is_empty());
    }
}
