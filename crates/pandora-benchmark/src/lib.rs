use chrono::{DateTime, Utc};
use pandora_types::services::{BenchmarkService, Service, ServiceId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRecord {
    pub model: String,
    pub domain: String,
    pub score: f64,
    pub duration_ms: u64,
    pub tokens_used: usize,
    pub cost: f64,
    pub errors: u32,
    pub retries: u32,
    pub success: bool,
    pub loop_count: u32,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

impl BenchmarkRecord {
    pub fn new(model: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            domain: domain.into(),
            score: 0.0,
            duration_ms: 0,
            tokens_used: 0,
            cost: 0.0,
            errors: 0,
            retries: 0,
            success: true,
            loop_count: 0,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct BenchmarkEngine {
    records: Vec<BenchmarkRecord>,
    max_records: usize,
}

impl BenchmarkEngine {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            max_records: 100_000,
        }
    }
    pub fn with_max_records(max: usize) -> Self {
        Self {
            records: Vec::new(),
            max_records: max,
        }
    }

    pub fn get_records(&self, model: &str, domain: &str) -> Vec<&BenchmarkRecord> {
        self.records
            .iter()
            .filter(|r| r.model == model && r.domain == domain)
            .collect()
    }

    pub fn average_score(&self, model: &str, domain: &str) -> Option<f64> {
        let records = self.get_records(model, domain);
        if records.is_empty() {
            return None;
        }
        Some(records.iter().map(|r| r.score).sum::<f64>() / records.len() as f64)
    }

    pub fn trend(&self, model: &str, domain: &str, count: usize) -> Vec<(String, f64)> {
        self.get_records(model, domain)
            .into_iter()
            .rev()
            .take(count)
            .map(|r| (r.timestamp.to_rfc3339(), r.score))
            .collect()
    }

    pub fn compare(&self, models: &[String], domain: &str) -> Vec<(String, f64)> {
        let mut results: Vec<(String, f64)> = models
            .iter()
            .filter_map(|m| self.average_score(m, domain).map(|s| (m.clone(), s)))
            .collect();
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    pub fn rankings(&self, domain: &str) -> Vec<(String, f64)> {
        let mut model_scores: HashMap<String, Vec<f64>> = HashMap::new();
        for record in &self.records {
            if record.domain == domain {
                model_scores
                    .entry(record.model.clone())
                    .or_default()
                    .push(record.score);
            }
        }
        let mut rankings: Vec<(String, f64)> = model_scores
            .into_iter()
            .map(|(model, scores)| (model, scores.iter().sum::<f64>() / scores.len() as f64))
            .collect();
        rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        rankings
    }

    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&self.records).unwrap_or_default()
    }
}

impl Default for BenchmarkEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for BenchmarkEngine {
    fn service_id(&self) -> ServiceId {
        ServiceId::Benchmark
    }
    fn provider_name(&self) -> &str {
        "pandora-benchmark"
    }
    fn version(&self) -> &str {
        "0.1.0"
    }
}

impl BenchmarkService for BenchmarkEngine {
    fn record(&self, model: &str, task: &str, score: f64, _metadata: &str) -> Result<(), String> {
        info!(model = %model, task = %task, score = %score, "benchmark record");
        Ok(())
    }
    fn query(&self, model: &str, task: &str) -> Result<Vec<(String, f64)>, String> {
        Ok(self.trend(model, task, 10))
    }
    fn compare(&self, models: &[String], task: &str) -> Result<Vec<(String, f64)>, String> {
        Ok(self.compare(models, task))
    }
    fn trend(&self, model: &str, task: &str) -> Result<Vec<(String, f64)>, String> {
        Ok(self.trend(model, task, 20))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> BenchmarkEngine {
        let mut e = BenchmarkEngine::new();
        for (i, m) in ["qwen", "deepseek", "llama3"].iter().enumerate() {
            for d in &["rust", "python", "eda"] {
                let mut r = BenchmarkRecord::new(*m, *d);
                r.score = 80.0 + (i as f64 * 5.0) + (d.len() as f64 % 10.0);
                r.success = true;
                e.records.push(r);
            }
        }
        e
    }

    #[test]
    fn average_score_works() {
        let e = sample();
        assert!(e.average_score("qwen", "rust").unwrap() > 0.0);
    }

    #[test]
    fn rankings_sorted() {
        let r = sample().rankings("rust");
        assert!(!r.is_empty());
        if r.len() >= 2 {
            assert!(r[0].1 >= r[1].1);
        }
    }

    #[test]
    fn compare_models() {
        let r = sample().compare(&["qwen".to_string(), "llama3".to_string()], "python");
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn empty_engine() {
        let e = BenchmarkEngine::new();
        assert!(e.average_score("x", "y").is_none());
        assert!(e.rankings("x").is_empty());
    }
}
