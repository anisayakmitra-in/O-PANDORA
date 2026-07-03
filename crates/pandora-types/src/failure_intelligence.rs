//! Failure Intelligence Engine — constitutional failure clustering and root cause analysis.
//!
//! Ingests execution telemetry, clusters failures by root cause,
//! generates structured reports, and feeds Knowledge Distillation.
//!
//! Two variants:
//!   - Standard: stores distilled knowledge for ANUBIS
//!   - Parliament: monitors failures across all services, triggers governance actions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Classification of a failure.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FailureClass {
    Reasoning,
    Tool,
    Provider,
    Harness,
    Gene,
    Memory,
    Execution,
    Security,
    Network,
    Sandbox,
    Model,
    Policy,
    Environment,
    User,
    Unknown,
}

impl FailureClass {
    pub fn name(&self) -> &'static str {
        match self {
            FailureClass::Reasoning => "reasoning",
            FailureClass::Tool => "tool",
            FailureClass::Provider => "provider",
            FailureClass::Harness => "harness",
            FailureClass::Gene => "gene",
            FailureClass::Memory => "memory",
            FailureClass::Execution => "execution",
            FailureClass::Security => "security",
            FailureClass::Network => "network",
            FailureClass::Sandbox => "sandbox",
            FailureClass::Model => "model",
            FailureClass::Policy => "policy",
            FailureClass::Environment => "environment",
            FailureClass::User => "user",
            FailureClass::Unknown => "unknown",
        }
    }
}

/// A single failure occurrence from execution telemetry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureRecord {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub failure_class: FailureClass,
    pub service: String,
    pub domain: String,
    pub provider: String,
    pub model: String,
    pub error_message: String,
    pub stack_trace: String,
    pub input_hash: String,
    pub latency_ms: u64,
    pub retries: u32,
    pub trace_id: String,
    pub span_id: String,
}

impl FailureRecord {
    pub fn new(service: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            id: format!("fail-{:x}", 42u64),
            timestamp: Utc::now(),
            failure_class: FailureClass::Unknown,
            service: service.into(),
            domain: domain.into(),
            provider: String::new(),
            model: String::new(),
            error_message: String::new(),
            stack_trace: String::new(),
            input_hash: String::new(),
            latency_ms: 0,
            retries: 0,
            trace_id: String::new(),
            span_id: String::new(),
        }
    }
}

/// A clustered root cause identified by the Failure Intelligence Engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCause {
    pub id: String,
    pub failure_class: FailureClass,
    pub service: String,
    pub domain: String,
    pub root_cause_label: String,
    pub description: String,
    pub frequency: u64,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub avg_latency_ms: f64,
    pub avg_retries: f64,
    pub confidence: f64,
    pub related_root_causes: Vec<String>,
    pub sample_failure_ids: Vec<String>,
}

impl RootCause {
    pub fn new(label: impl Into<String>, failure_class: FailureClass) -> Self {
        Self {
            id: format!("rc-{:x}", 42u64),
            failure_class,
            service: String::new(),
            domain: String::new(),
            root_cause_label: label.into(),
            description: String::new(),
            frequency: 0,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            avg_latency_ms: 0.0,
            avg_retries: 0.0,
            confidence: 0.0,
            related_root_causes: Vec::new(),
            sample_failure_ids: Vec::new(),
        }
    }
}

/// A structured report from failure analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureReport {
    pub id: String,
    pub root_cause: String,
    pub failure_class: FailureClass,
    pub service: String,
    pub domain: String,
    pub frequency: u64,
    pub confidence: f64,
    pub description: String,
    pub suggested_fixes: Vec<String>,
    pub estimated_gain: f64,
    pub risk: String,
    pub timestamp: DateTime<Utc>,
}

impl FailureReport {
    pub fn new(root_cause: impl Into<String>, failure_class: FailureClass) -> Self {
        Self {
            id: format!("report-{:x}", 42u64),
            root_cause: root_cause.into(),
            failure_class,
            service: String::new(),
            domain: String::new(),
            frequency: 0,
            confidence: 0.0,
            description: String::new(),
            suggested_fixes: Vec::new(),
            estimated_gain: 0.0,
            risk: "low".to_string(),
            timestamp: Utc::now(),
        }
    }
}

// =========================================================================
// Standard Failure Intelligence Engine (ANUBIS-bound)
// =========================================================================

pub struct FailureIntelligenceEngine {
    failures: Vec<FailureRecord>,
    root_causes: Vec<RootCause>,
    reports: Vec<FailureReport>,
    max_failures: usize,
}

impl FailureIntelligenceEngine {
    pub fn new() -> Self {
        Self {
            failures: Vec::new(),
            root_causes: Vec::new(),
            reports: Vec::new(),
            max_failures: 100_000,
        }
    }

    /// Ingest a failure from execution telemetry.
    pub fn ingest(&mut self, record: FailureRecord) {
        self.failures.push(record);
        while self.failures.len() > self.max_failures {
            self.failures.remove(0);
        }
    }

    /// Cluster failures and identify root causes.
    pub fn cluster(&mut self) -> Vec<RootCause> {
        let mut clusters: HashMap<String, Vec<&FailureRecord>> = HashMap::new();
        for failure in &self.failures {
            let key = format!(
                "{}:{}:{}",
                failure.service,
                failure.domain,
                failure.error_message.chars().take(80).collect::<String>()
            );
            clusters.entry(key).or_default().push(failure);
        }

        self.root_causes = clusters
            .into_iter()
            .map(|(key, records)| {
                let first = records[0];
                let freq = records.len() as u64;
                RootCause {
                    id: format!("rc-{:x}", rand::random::<u64>()),
                    failure_class: first.failure_class,
                    service: first.service.clone(),
                    domain: first.domain.clone(),
                    root_cause_label: key.clone(),
                    description: format!("{} failures in {}:{}", freq, first.service, first.domain),
                    frequency: freq,
                    first_seen: records
                        .iter()
                        .map(|r| r.timestamp)
                        .min()
                        .unwrap_or(Utc::now()),
                    last_seen: records
                        .iter()
                        .map(|r| r.timestamp)
                        .max()
                        .unwrap_or(Utc::now()),
                    avg_latency_ms: records.iter().map(|r| r.latency_ms as f64).sum::<f64>()
                        / records.len() as f64,
                    avg_retries: records.iter().map(|r| r.retries as f64).sum::<f64>()
                        / records.len() as f64,
                    confidence: (freq as f64 / (freq as f64 + 10.0)).min(0.99),
                    related_root_causes: Vec::new(),
                    sample_failure_ids: records.iter().take(5).map(|r| r.id.clone()).collect(),
                }
            })
            .collect();
        self.root_causes.clone()
    }

    /// Generate structured reports from root causes.
    pub fn generate_reports(&self) -> Vec<FailureReport> {
        self.root_causes
            .iter()
            .filter(|rc| rc.frequency >= 3)
            .map(|rc| FailureReport {
                id: format!("report-{:x}", rand::random::<u64>()),
                root_cause: rc.root_cause_label.clone(),
                failure_class: rc.failure_class,
                service: rc.service.clone(),
                domain: rc.domain.clone(),
                frequency: rc.frequency,
                confidence: rc.confidence,
                description: rc.description.clone(),
                suggested_fixes: vec!["Investigate and patch".to_string()],
                estimated_gain: (rc.frequency as f64 * 0.1).min(100.0),
                risk: if rc.frequency > 10 {
                    "high".to_string()
                } else {
                    "medium".to_string()
                },
                timestamp: Utc::now(),
            })
            .collect()
    }

    /// Get all root causes, sorted by frequency descending.
    pub fn root_causes_sorted(&self) -> Vec<&RootCause> {
        let mut v: Vec<&RootCause> = self.root_causes.iter().collect();
        v.sort_by(|a, b| b.frequency.cmp(&a.frequency));
        v
    }

    /// Get all reports.
    pub fn reports(&self) -> &[FailureReport] {
        &self.reports
    }
    pub fn root_cause_count(&self) -> usize {
        self.root_causes.len()
    }
    pub fn failure_count(&self) -> usize {
        self.failures.len()
    }

    /// Produce ANUBIS-compatible distilled knowledge.
    pub fn distill_for_anubis(&self) -> DistilledFailureKnowledge {
        let reports = self.generate_reports();
        DistilledFailureKnowledge {
            total_failures: self.failures.len() as u64,
            total_root_causes: self.root_causes.len() as u64,
            cluster_summary: self
                .root_causes
                .iter()
                .map(|rc| {
                    format!(
                        "[{}] {} ({} occurrences, {:.0}% confidence)",
                        rc.failure_class.name(),
                        rc.root_cause_label,
                        rc.frequency,
                        rc.confidence * 100.0
                    )
                })
                .collect(),
            reports: reports
                .iter()
                .map(|r| {
                    format!(
                        "[{}] {}: {} (x{}, risk: {})",
                        r.failure_class.name(),
                        r.root_cause,
                        r.description,
                        r.frequency,
                        r.risk
                    )
                })
                .collect(),
            generated_at: Utc::now(),
        }
    }
}

impl Default for FailureIntelligenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Compact distilled failure knowledge for ANUBIS storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistilledFailureKnowledge {
    pub total_failures: u64,
    pub total_root_causes: u64,
    pub cluster_summary: Vec<String>,
    pub reports: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

// =========================================================================
// Parliament Variant — constitutional failure monitoring
// =========================================================================

/// A governance action triggered by failure intelligence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureGovernanceAction {
    pub action_id: String,
    pub root_cause_id: String,
    pub action_type: String,
    pub description: String,
    pub target_service: String,
    pub target_domain: String,
    pub severity: String,
    pub proposed_policy: String,
    pub requires_approval: bool,
    pub timestamp: DateTime<Utc>,
}

impl FailureGovernanceAction {
    pub fn new(action_type: impl Into<String>, target_service: impl Into<String>) -> Self {
        Self {
            action_id: format!("fga-{:x}", 42u64),
            root_cause_id: String::new(),
            action_type: action_type.into(),
            description: String::new(),
            target_service: target_service.into(),
            target_domain: String::new(),
            severity: "medium".to_string(),
            proposed_policy: String::new(),
            requires_approval: true,
            timestamp: Utc::now(),
        }
    }
}

/// Parliament's constitutional failure monitor.
///
/// Watches failures across all services and generates governance proposals.
/// If a failure pattern exceeds thresholds, triggers quarantine or policy changes.
pub struct ParliamentFailureMonitor {
    engine: FailureIntelligenceEngine,
    governance_actions: Vec<FailureGovernanceAction>,
    thresholds: HashMap<String, f64>,
}

impl ParliamentFailureMonitor {
    pub fn new() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert("max_failure_rate".to_string(), 0.1); // 10%
        thresholds.insert("max_latency_degradation".to_string(), 2.0); // 2x normal
        thresholds.insert("min_confidence_for_action".to_string(), 0.6);
        Self {
            engine: FailureIntelligenceEngine::new(),
            governance_actions: Vec::new(),
            thresholds,
        }
    }

    /// Ingest a failure from any service.
    pub fn ingest(&mut self, record: FailureRecord) {
        self.engine.ingest(record);
    }

    /// Run failure analysis and generate governance actions if thresholds exceeded.
    pub fn analyze(&mut self) -> Vec<FailureGovernanceAction> {
        let root_causes = self.engine.cluster();
        let mut actions = Vec::new();

        for rc in &root_causes {
            if rc.confidence
                < self
                    .thresholds
                    .get("min_confidence_for_action")
                    .copied()
                    .unwrap_or(0.5)
            {
                continue;
            }

            // Generate action based on frequency
            if rc.frequency > 10 {
                let mut action = FailureGovernanceAction::new("quarantine", &rc.service);
                action.root_cause_id = rc.id.clone();
                action.description = format!(
                    "Automatic quarantine: {} failures in {}:{}",
                    rc.frequency, rc.service, rc.domain
                );
                action.severity = "high".to_string();
                action.requires_approval = true;
                actions.push(action);
            } else if rc.frequency > 3 {
                let mut action = FailureGovernanceAction::new("investigate", &rc.service);
                action.root_cause_id = rc.id.clone();
                action.description = format!(
                    "Investigate: {} failures in {}:{}",
                    rc.frequency, rc.service, rc.domain
                );
                action.severity = "medium".to_string();
                actions.push(action);
            }
        }

        self.governance_actions.extend(actions.clone());
        actions
    }

    /// Get distilled knowledge report for Parliament review.
    pub fn distill_for_parliament(&self) -> ParliamentFailureReport {
        let distilled = self.engine.distill_for_anubis();
        ParliamentFailureReport {
            total_failures: distilled.total_failures,
            total_root_causes: distilled.total_root_causes,
            clusters: distilled.cluster_summary,
            reports: distilled.reports,
            active_governance_actions: self.governance_actions.len(),
            pending_actions: self
                .governance_actions
                .iter()
                .filter(|a| a.requires_approval)
                .count(),
            generated_at: Utc::now(),
        }
    }

    pub fn governance_actions(&self) -> &[FailureGovernanceAction] {
        &self.governance_actions
    }
    pub fn set_threshold(&mut self, key: impl Into<String>, value: f64) {
        self.thresholds.insert(key.into(), value);
    }
}

impl Default for ParliamentFailureMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Parliament-specific failure report for constitutional review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentFailureReport {
    pub total_failures: u64,
    pub total_root_causes: u64,
    pub clusters: Vec<String>,
    pub reports: Vec<String>,
    pub active_governance_actions: usize,
    pub pending_actions: usize,
    pub generated_at: DateTime<Utc>,
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_failure(
        service: &str,
        domain: &str,
        error: &str,
        class: FailureClass,
    ) -> FailureRecord {
        let mut f = FailureRecord::new(service, domain);
        f.failure_class = class;
        f.error_message = error.to_string();
        f.latency_ms = 500;
        f.retries = 3;
        f
    }

    #[test]
    fn ingest_failures() {
        let mut engine = FailureIntelligenceEngine::new();
        for i in 0..10 {
            let f = sample_failure(
                "anubis",
                "memory",
                &format!("timeout #{}", i),
                FailureClass::Memory,
            );
            engine.ingest(f);
        }
        assert_eq!(engine.failure_count(), 10);
    }

    #[test]
    fn clustering_groups_similar_failures() {
        let mut engine = FailureIntelligenceEngine::new();
        for _ in 0..5 {
            engine.ingest(sample_failure(
                "anubis",
                "memory",
                "vector_db_timeout",
                FailureClass::Memory,
            ));
        }
        for _ in 0..3 {
            engine.ingest(sample_failure(
                "phoenix",
                "execution",
                "provider_unavailable",
                FailureClass::Provider,
            ));
        }
        let rc = engine.cluster();
        assert!(rc.len() >= 2);
    }

    #[test]
    fn reports_generated_for_frequent_failures() {
        let mut engine = FailureIntelligenceEngine::new();
        for _ in 0..5 {
            engine.ingest(sample_failure(
                "anubis",
                "memory",
                "timeout",
                FailureClass::Memory,
            ));
        }
        engine.cluster();
        let reports = engine.generate_reports();
        assert!(!reports.is_empty());
    }

    #[test]
    fn parliament_monitor_governance_actions() {
        let mut monitor = ParliamentFailureMonitor::new();
        for _ in 0..15 {
            monitor.ingest(sample_failure(
                "anubis",
                "memory",
                "critical_failure",
                FailureClass::Memory,
            ));
        }
        let actions = monitor.analyze();
        assert!(!actions.is_empty());
        // High frequency should trigger quarantine
        assert!(actions.iter().any(|a| a.action_type == "quarantine"));
    }

    #[test]
    fn distill_for_anubis_produces_summary() {
        let mut engine = FailureIntelligenceEngine::new();
        for _ in 0..10 {
            engine.ingest(sample_failure(
                "anubis",
                "memory",
                "timeout",
                FailureClass::Memory,
            ));
        }
        engine.cluster();
        let distilled = engine.distill_for_anubis();
        assert_eq!(distilled.total_failures, 10);
        assert!(!distilled.cluster_summary.is_empty());
    }

    #[test]
    fn parliament_report_contains_governance_info() {
        let mut monitor = ParliamentFailureMonitor::new();
        for _ in 0..20 {
            monitor.ingest(sample_failure(
                "phoenix",
                "execution",
                "provider_down",
                FailureClass::Provider,
            ));
        }
        monitor.analyze();
        let report = monitor.distill_for_parliament();
        assert!(report.active_governance_actions > 0);
    }
}
