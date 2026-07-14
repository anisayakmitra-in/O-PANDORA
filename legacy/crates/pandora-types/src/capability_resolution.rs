//! Capability Resolution Engine.
//!
//! Takes an intent/capability request and returns the best provider
//! by matching capabilities, benchmark scores, constraints, and lease
//! availability.
//!
//! Pipeline: CapabilityRequest → Domain Scoring → Provider Matching
//! → Constraint Filtering → Benchmark Integration → Ranking → Lease

use std::collections::HashMap;

/// A request for capability resolution.
#[derive(Debug, Clone)]
pub struct CapabilityRequest {
    pub domain: String,
    pub task_type: String,
    pub constraints: CapabilityConstraints,
}

/// Constraints for capability resolution.
#[derive(Debug, Clone, Default)]
pub struct CapabilityConstraints {
    pub max_cost: Option<f64>,
    pub min_score: Option<f64>,
    pub max_latency_ms: Option<u64>,
    pub require_offline: bool,
    pub require_tools: bool,
    pub require_vision: bool,
    pub min_context: Option<usize>,
    pub preferred_models: Vec<String>,
}

/// A candidate from capability resolution.
#[derive(Debug, Clone)]
pub struct CapabilityCandidate {
    pub model: String,
    pub provider: String,
    pub overall_score: f64,
    pub domain_scores: HashMap<String, f64>,
    pub cost: f64,
    pub latency_ms: f64,
    pub context_limit: usize,
    pub supports_tools: bool,
    pub supports_vision: bool,
}

/// Scoring weights for capability matching.
#[derive(Debug, Clone)]
pub struct ResolutionWeights {
    pub benchmark_score: f64,
    pub cost_efficiency: f64,
    pub latency: f64,
    pub context_capacity: f64,
}

impl Default for ResolutionWeights {
    fn default() -> Self {
        Self { benchmark_score: 0.5, cost_efficiency: 0.2, latency: 0.15, context_capacity: 0.15 }
    }
}

/// A provider entry with benchmark data.
#[derive(Debug, Clone)]
pub struct ProviderEntry {
    pub model: String,
    pub provider: String,
    pub domain_scores: HashMap<String, f64>,
    pub cost: f64,
    pub latency_ms: f64,
    pub context_limit: usize,
    pub supports_tools: bool,
    pub supports_vision: bool,
    pub is_offline: bool,
}

/// Per-domain benchmark averages.
#[derive(Debug, Clone, Default)]
pub struct DomainBenchmarks {
    pub scores: HashMap<String, f64>,
}

/// Registry of known providers and their benchmark data.
#[derive(Debug, Clone, Default)]
pub struct ProviderBenchmarkRegistry {
    pub providers: Vec<ProviderEntry>,
    pub domain_rankings: HashMap<String, DomainBenchmarks>,
}

impl ProviderBenchmarkRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, entry: ProviderEntry) {
        for (domain, score) in &entry.domain_scores {
            self.domain_rankings.entry(domain.clone()).or_default().scores.insert(entry.model.clone(), *score);
        }
        self.providers.push(entry);
    }

    pub fn ranked_for_domain(&self, domain: &str) -> Vec<(String, f64)> {
        let mut results: Vec<(String, f64)> = self.providers.iter()
            .filter_map(|p| p.domain_scores.get(domain).map(|s| (p.model.clone(), *s)))
            .collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}

/// The Capability Resolution Engine — Pandora's heart.
pub struct CapabilityResolutionEngine {
    weights: ResolutionWeights,
    registry: ProviderBenchmarkRegistry,
}

impl CapabilityResolutionEngine {
    pub fn new() -> Self { Self { weights: ResolutionWeights::default(), registry: ProviderBenchmarkRegistry::new() } }
    pub fn with_weights(weights: ResolutionWeights) -> Self { Self { weights, registry: ProviderBenchmarkRegistry::new() } }
    pub fn registry(&self) -> &ProviderBenchmarkRegistry { &self.registry }
    pub fn registry_mut(&mut self) -> &mut ProviderBenchmarkRegistry { &mut self.registry }

    pub fn resolve(&self, request: &CapabilityRequest) -> Vec<CapabilityCandidate> {
        let mut candidates: Vec<CapabilityCandidate> = self.registry.providers.iter()
            .filter(|p| self.meets_constraints(p, &request.constraints))
            .map(|p| self.score_candidate(p, request))
            .collect();
        candidates.sort_by(|a, b| b.overall_score.partial_cmp(&a.overall_score).unwrap_or(std::cmp::Ordering::Equal));
        candidates
    }

    pub fn resolve_domain(&self, domain: &str) -> Vec<CapabilityCandidate> {
        self.resolve(&CapabilityRequest { domain: domain.into(), task_type: "general".into(), constraints: CapabilityConstraints::default() })
    }

    fn score_candidate(&self, entry: &ProviderEntry, request: &CapabilityRequest) -> CapabilityCandidate {
        let domain_score = entry.domain_scores.get(&request.domain).copied().unwrap_or(50.0);
        let overall = self.weights.benchmark_score * (domain_score / 100.0)
            + self.weights.cost_efficiency * self.normalize_cost(entry.cost)
            + self.weights.latency * self.normalize_latency(entry.latency_ms)
            + self.weights.context_capacity * self.normalize_context(entry.context_limit);
        CapabilityCandidate {
            model: entry.model.clone(), provider: entry.provider.clone(),
            overall_score: overall * 100.0,
            domain_scores: entry.domain_scores.clone(),
            cost: entry.cost, latency_ms: entry.latency_ms,
            context_limit: entry.context_limit,
            supports_tools: entry.supports_tools, supports_vision: entry.supports_vision,
        }
    }

    fn meets_constraints(&self, entry: &ProviderEntry, c: &CapabilityConstraints) -> bool {
        if let Some(max) = c.max_cost { if entry.cost > max { return false; } }
        if let Some(min) = c.min_score {
            let avg = entry.domain_scores.values().sum::<f64>() / entry.domain_scores.len().max(1) as f64;
            if avg < min { return false; }
        }
        if let Some(max) = c.max_latency_ms { if entry.latency_ms > max as f64 { return false; } }
        if c.require_tools && !entry.supports_tools { return false; }
        if c.require_vision && !entry.supports_vision { return false; }
        if let Some(min) = c.min_context { if entry.context_limit < min { return false; } }
        true
    }

    fn normalize_cost(&self, cost: f64) -> f64 { if cost <= 0.0 { 1.0 } else { (1.0 / (1.0 + cost)).min(1.0) } }
    fn normalize_latency(&self, ms: f64) -> f64 { if ms <= 0.0 { 1.0 } else { (1.0 / (1.0 + ms / 1000.0)).min(1.0) } }
    fn normalize_context(&self, ctx: usize) -> f64 { (ctx as f64 / 128_000.0).min(1.0) }
}

impl Default for CapabilityResolutionEngine { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_engine() -> CapabilityResolutionEngine {
        let mut e = CapabilityResolutionEngine::new();
        e.registry_mut().register(ProviderEntry {
            model: "claude-sonnet-4".into(), provider: "anthropic".into(),
            domain_scores: HashMap::from([("rust".into(), 95.0), ("reasoning".into(), 92.0), ("coding".into(), 93.0)]),
            cost: 0.015, latency_ms: 800.0, context_limit: 200000,
            supports_tools: true, supports_vision: true, is_offline: false,
        });
        e.registry_mut().register(ProviderEntry {
            model: "qwen3-coder".into(), provider: "ollama".into(),
            domain_scores: HashMap::from([("rust".into(), 88.0), ("coding".into(), 90.0), ("python".into(), 91.0)]),
            cost: 0.0, latency_ms: 200.0, context_limit: 32000,
            supports_tools: true, supports_vision: false, is_offline: true,
        });
        e
    }

    #[test]
    fn resolve_returns_ranked() {
        let results = sample_engine().resolve_domain("rust");
        assert_eq!(results.len(), 2);
        assert!(results[0].overall_score >= results[1].overall_score);
    }

    #[test]
    fn constraints_filter_providers() {
        let mut e = CapabilityResolutionEngine::new();
        e.registry_mut().register(ProviderEntry {
            model: "cheap".into(), provider: "local".into(),
            domain_scores: HashMap::from([("coding".into(), 60.0)]),
            cost: 0.001, latency_ms: 100.0, context_limit: 4096,
            supports_tools: false, supports_vision: false, is_offline: true,
        });
        assert!(e.resolve(&CapabilityRequest {
            domain: "coding".into(), task_type: "general".into(),
            constraints: CapabilityConstraints { min_score: Some(80.0), ..Default::default() },
        }).is_empty());
    }

    #[test]
    fn domain_rankings_work() {
        let ranked = sample_engine().registry().ranked_for_domain("rust");
        assert_eq!(ranked[0].0, "claude-sonnet-4");
    }

    #[test]
    fn cost_scoring_prefers_free() {
        let e = CapabilityResolutionEngine::new();
        assert!(e.normalize_cost(0.0) > e.normalize_cost(1.0));
    }
}
