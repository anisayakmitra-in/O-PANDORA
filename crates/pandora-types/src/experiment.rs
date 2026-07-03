//! Experiment Engine — Pandora's scientific method.
//!
//! Powers provider ranking, workflow optimization, GEPA, DSR, engineering, research.
//!
//! Experiment -> Variant A/B/C -> Providers -> Hardware -> Execution ->
//! Benchmarks -> Statistics -> Winner -> ANUBIS.
//!
//! Everything is reproducible, comparable, and auditable.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// A variant in an experiment — one configuration to test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentVariant {
    pub id: String,
    pub label: String,
    pub provider: String,
    pub model: String,
    pub domain: String,
    pub parameters: HashMap<String, String>,
}

impl ExperimentVariant {
    pub fn new(label: impl Into<String>, provider: impl Into<String>, model: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            id: format!("var-{:x}", 42u64),
            label: label.into(),
            provider: provider.into(),
            model: model.into(),
            domain: domain.into(),
            parameters: HashMap::new(),
        }
    }
}

/// A single run of a variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantRun {
    pub variant_id: String,
    pub run_number: u32,
    pub success: bool,
    pub score: f64,
    pub latency_ms: f64,
    pub tokens_used: usize,
    pub cost: f64,
    pub retries: u32,
    pub error_message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Full experiment definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub domain: String,
    pub variants: Vec<ExperimentVariant>,
    pub runs_per_variant: u32,
    pub created_at: DateTime<Utc>,
    pub status: ExperimentStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExperimentStatus {
    Draft,
    Running,
    Completed,
    Failed,
}

/// Statistical results for one variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantResult {
    pub variant_id: String,
    pub label: String,
    pub mean_score: f64,
    pub median_score: f64,
    pub std_dev: f64,
    pub min_score: f64,
    pub max_score: f64,
    pub mean_latency_ms: f64,
    pub mean_cost: f64,
    pub success_rate: f64,
    pub run_count: u32,
    pub confidence_interval: (f64, f64),
    pub rank: usize,
}

impl VariantResult {
    pub fn new(variant_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            variant_id: variant_id.into(),
            label: label.into(),
            mean_score: 0.0, median_score: 0.0, std_dev: 0.0,
            min_score: 0.0, max_score: 0.0,
            mean_latency_ms: 0.0, mean_cost: 0.0, success_rate: 0.0,
            run_count: 0, confidence_interval: (0.0, 0.0), rank: 0,
        }
    }
}

/// Final experiment report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentReport {
    pub experiment_id: String,
    pub experiment_name: String,
    pub domain: String,
    pub variant_count: usize,
    pub total_runs: usize,
    pub winner: String,
    pub winner_score: f64,
    pub variant_results: Vec<VariantResult>,
    pub recommendation: String,
    pub completed_at: DateTime<Utc>,
}

/// The Experiment Engine.
pub struct ExperimentEngine {
    experiments: HashMap<String, Experiment>,
    runs: Vec<VariantRun>,
    max_runs: usize,
}

impl ExperimentEngine {
    pub fn new() -> Self { Self { experiments: HashMap::new(), runs: Vec::new(), max_runs: 100_000 } }

    /// Define a new experiment with variants.
    pub fn define(&mut self, name: impl Into<String>, domain: impl Into<String>, variants: Vec<ExperimentVariant>, runs_per_variant: u32) -> String {
        let id = format!("exp-{:x}", 42u64);
        let exp = Experiment {
            id: id.clone(),
            name: name.into(),
            description: String::new(),
            domain: domain.into(),
            variants,
            runs_per_variant,
            created_at: Utc::now(),
            status: ExperimentStatus::Draft,
        };
        self.experiments.insert(id.clone(), exp);
        id
    }

    /// Start an experiment.
    pub fn start(&mut self, experiment_id: &str) -> Result<(), String> {
        if let Some(exp) = self.experiments.get_mut(experiment_id) {
            exp.status = ExperimentStatus::Running;
            Ok(())
        } else {
            Err(format!("experiment '{}' not found", experiment_id))
        }
    }

    /// Record a single run result.
    pub fn record_run(&mut self, run: VariantRun) {
        self.runs.push(run);
        while self.runs.len() > self.max_runs { self.runs.remove(0); }
    }

    /// Analyze results and compute statistics.
    pub fn analyze(&mut self, experiment_id: &str) -> Result<ExperimentReport, String> {
        let exp = self.experiments.get(experiment_id).ok_or_else(|| format!("experiment '{}' not found", experiment_id))?;

        let mut results: Vec<VariantResult> = Vec::new();
        for variant in &exp.variants {
            let variant_runs: Vec<&VariantRun> = self.runs.iter()
                .filter(|r| r.variant_id == variant.id)
                .collect();

            if variant_runs.is_empty() { continue; }

            let scores: Vec<f64> = variant_runs.iter().map(|r| r.score).collect();
            let count = scores.len() as u32;
            let mean = scores.iter().sum::<f64>() / scores.len() as f64;
            let mut sorted = scores.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median = if sorted.len() % 2 == 0 {
                (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
            } else {
                sorted[sorted.len() / 2]
            };
            let variance = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / scores.len() as f64;
            let std_dev = variance.sqrt();
            let success_count = variant_runs.iter().filter(|r| r.success).count() as f64;
            let success_rate = success_count / scores.len() as f64;
            let avg_latency = variant_runs.iter().map(|r| r.latency_ms).sum::<f64>() / scores.len() as f64;
            let avg_cost = variant_runs.iter().map(|r| r.cost).sum::<f64>() / scores.len() as f64;
            let ci = 1.96 * std_dev / (scores.len() as f64).sqrt();
            let ci_lower = (mean - ci).max(0.0);
            let ci_upper = (mean + ci).min(100.0);

            results.push(VariantResult {
                variant_id: variant.id.clone(),
                label: variant.label.clone(),
                mean_score: mean,
                median_score: median,
                std_dev,
                min_score: sorted.first().copied().unwrap_or(0.0),
                max_score: sorted.last().copied().unwrap_or(0.0),
                mean_latency_ms: avg_latency,
                mean_cost: avg_cost,
                success_rate,
                run_count: count,
                confidence_interval: (ci_lower, ci_upper),
                rank: 0,
            });
        }

        // Sort by mean score descending and assign ranks
        results.sort_by(|a, b| b.mean_score.partial_cmp(&a.mean_score).unwrap_or(std::cmp::Ordering::Equal));
        for (i, result) in results.iter_mut().enumerate() {
            result.rank = i + 1;
        }

        let winner = results.first().map(|w| w.label.clone()).unwrap_or_default();
        let winner_score = results.first().map(|w| w.mean_score).unwrap_or(0.0);
        let total_runs: usize = results.iter().map(|r| r.run_count as usize).sum();

        let recommendation = if winner_score > 80.0 {
            format!("Strongly recommend '{}' (score: {:.1}). High confidence.", winner, winner_score)
        } else if winner_score > 60.0 {
            format!("Recommend '{}' (score: {:.1}). Moderate confidence.", winner, winner_score)
        } else {
            format!("No clear winner. Best variant '{}' scored {:.1}. Consider more runs.", winner, winner_score)
        };

        let exp_name = exp.name.clone();
        let exp_domain = exp.domain.clone();
        let variant_count = exp.variants.len();
        let variant_count2_copy = variant_count;

        if let Some(e) = self.experiments.get_mut(experiment_id) {
            e.status = ExperimentStatus::Completed;
        }

        Ok(ExperimentReport {
            experiment_id: experiment_id.to_string(),
            experiment_name: exp_name,
            domain: exp_domain,
            variant_count: variant_count2_copy,
            total_runs,
            winner,
            winner_score,
            variant_results: results,
            recommendation,
            completed_at: Utc::now(),
        })
    }

    /// List all experiments.
    pub fn list(&self) -> Vec<&Experiment> {
        let mut v: Vec<&Experiment> = self.experiments.values().collect();
        v.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        v
    }

    pub fn get_experiment(&self, id: &str) -> Option<&Experiment> { self.experiments.get(id) }
    pub fn experiment_count(&self) -> usize { self.experiments.len() }
    pub fn total_runs(&self) -> usize { self.runs.len() }
}

impl Default for ExperimentEngine { fn default() -> Self { Self::new() } }

