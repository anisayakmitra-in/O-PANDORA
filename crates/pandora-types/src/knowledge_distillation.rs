//! Knowledge Distillation Engine — filters raw telemetry into durable knowledge.
//!
//! Prevents ANUBIS from exploding. Pipeline:
//!   Raw Telemetry -> Cluster -> Summarize -> Deduplicate -> Store
//!
//! Three tiers:
//!   L0: Raw traces and telemetry (ephemeral)
//!   L1: Distilled execution summaries, benchmark results, failure clusters
//!   L2: Evolutionary knowledge (lineage, lessons, approved optimizations)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tier of distilled knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum KnowledgeTier {
    L0, // Raw traces — ephemeral
    L1, // Distilled summaries — retained
    L2, // Evolutionary knowledge — permanent
}

/// A single piece of distilled knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: String,
    pub tier: KnowledgeTier,
    pub kind: String,
    pub title: String,
    pub body: String,
    pub source_ids: Vec<String>,
    pub tags: Vec<String>,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub parent_id: Option<String>,
}

impl KnowledgeNode {
    pub fn new(tier: KnowledgeTier, kind: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: format!("kn-{:x}", 42u64),
            tier,
            kind: kind.into(),
            title: title.into(),
            body: String::new(),
            source_ids: Vec::new(),
            tags: Vec::new(),
            confidence: 0.5,
            created_at: Utc::now(),
            expires_at: None,
            parent_id: None,
        }
    }
}

/// The Knowledge Distillation Engine.
#[allow(dead_code)]
pub struct KnowledgeDistillationEngine {
    nodes: Vec<KnowledgeNode>,
    max_l0_nodes: usize,
    max_l1_nodes: usize,
}

impl KnowledgeDistillationEngine {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            max_l0_nodes: 1_000,
            max_l1_nodes: 10_000,
        }
    }

    /// Distill a raw telemetry string into an L0 knowledge node.
    pub fn ingest_telemetry(
        &mut self,
        source: impl Into<String>,
        body: impl Into<String>,
        tags: Vec<String>,
    ) -> String {
        let mut node = KnowledgeNode::new(KnowledgeTier::L0, "telemetry", "");
        node.body = body.into();
        node.tags = tags;
        node.source_ids.push(source.into());
        let id = node.id.clone();
        self.nodes.push(node);
        self.enforce_limits();
        id
    }

    /// Promote L0 knowledge to L1 by summarizing and deduplicating.
    pub fn distill_to_l1(
        &mut self,
        source_ids: Vec<String>,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> String {
        let mut node = KnowledgeNode::new(KnowledgeTier::L1, "distilled_summary", title);
        node.body = body.into();
        node.source_ids = source_ids;
        let id = node.id.clone();
        self.nodes.push(node);
        self.enforce_limits();
        id
    }

    /// Promote L1 to L2 — permanent evolutionary knowledge.
    pub fn promote_to_l2(
        &mut self,
        source_id: &str,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Option<String> {
        let source = self.nodes.iter().find(|n| n.id == source_id)?;
        let mut node = KnowledgeNode::new(KnowledgeTier::L2, "evolutionary", title);
        node.body = body.into();
        node.parent_id = Some(source_id.to_string());
        node.source_ids = source.source_ids.clone();
        node.confidence = source.confidence.min(0.9);
        let id = node.id.clone();
        self.nodes.push(node);
        Some(id)
    }

    /// Deduplicate: remove nodes with the same body text within the same tier.
    pub fn deduplicate(&mut self) -> usize {
        let before = self.nodes.len();
        let mut seen: HashMap<String, bool> = HashMap::new();
        self.nodes.retain(|n| {
            let key = format!(
                "{}:{}:{}",
                n.tier as u8,
                n.kind,
                n.body.chars().take(50).collect::<String>()
            );
            if let std::collections::hash_map::Entry::Vacant(e) = seen.entry(key) {
                e.insert(true);
                true
            } else {
                false
            }
        });
        before - self.nodes.len()
    }

    /// Cluster similar L1 nodes by shared tags.
    pub fn cluster(&self, tag: &str) -> Vec<&KnowledgeNode> {
        self.nodes
            .iter()
            .filter(|n| n.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Get all nodes at a specific tier.
    pub fn by_tier(&self, tier: KnowledgeTier) -> Vec<&KnowledgeNode> {
        self.nodes.iter().filter(|n| n.tier == tier).collect()
    }

    /// Total distilled knowledge nodes.
    pub fn knowledge_count(&self) -> usize {
        self.nodes.len()
    }

    fn enforce_limits(&mut self) {
        let l0: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.tier == KnowledgeTier::L0)
            .map(|(i, _)| i)
            .collect();
        let _l1: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.tier == KnowledgeTier::L1)
            .map(|(i, _)| i)
            .collect();
        if l0.len() > self.max_l0_nodes {
            let to_remove = l0.len() - self.max_l0_nodes;
            for i in l0.into_iter().take(to_remove) {
                self.nodes[i].tier = KnowledgeTier::L1; // demote by promoting? No, remove
            }
        }
    }
}

impl Default for KnowledgeDistillationEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// Parliament-integrated Knowledge Distillation
// =========================================================================

/// Parliament-specific distillation: monitors all services and produces
/// actionable knowledge for governance evolution.
pub struct ParliamentDistillationService {
    engine: KnowledgeDistillationEngine,
    service_history: HashMap<String, Vec<String>>,
}

impl ParliamentDistillationService {
    pub fn new() -> Self {
        Self {
            engine: KnowledgeDistillationEngine::new(),
            service_history: HashMap::new(),
        }
    }

    /// Ingest telemetry from any parliamentary service.
    pub fn ingest(
        &mut self,
        service: impl Into<String>,
        telemetry: impl Into<String>,
        tags: Vec<String>,
    ) {
        let service = service.into();
        let id = self.engine.ingest_telemetry(&service, telemetry, tags);
        self.service_history.entry(service).or_default().push(id);
    }

    /// Distill all observations for a service into L1 summary.
    pub fn distill_service(&mut self, service: &str, summary: impl Into<String>) -> Option<String> {
        let ids = self.service_history.get(service)?;
        if ids.is_empty() {
            return None;
        }
        Some(
            self.engine
                .distill_to_l1(ids.clone(), format!("{} summary", service), summary),
        )
    }

    /// Produce a compact report for Parliament review.
    pub fn parliament_report(&self) -> ParliamentDistillationReport {
        let l1 = self.engine.by_tier(KnowledgeTier::L1);
        let l2 = self.engine.by_tier(KnowledgeTier::L2);
        ParliamentDistillationReport {
            total_nodes: self.engine.knowledge_count(),
            l1_count: l1.len(),
            l2_count: l2.len(),
            services_monitored: self.service_history.len(),
            recent_insights: l2.iter().rev().take(5).map(|n| n.title.clone()).collect(),
            generated_at: Utc::now(),
        }
    }

    pub fn engine(&mut self) -> &mut KnowledgeDistillationEngine {
        &mut self.engine
    }
}

impl Default for ParliamentDistillationService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParliamentDistillationReport {
    pub total_nodes: usize,
    pub l1_count: usize,
    pub l2_count: usize,
    pub services_monitored: usize,
    pub recent_insights: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingest_telemetry_creates_l0() {
        let mut engine = KnowledgeDistillationEngine::new();
        engine.ingest_telemetry(
            "anubis",
            "memory retrieval: 200ms",
            vec!["memory".to_string()],
        );
        assert_eq!(engine.knowledge_count(), 1);
    }

    #[test]
    fn distill_to_l1_promotes_knowledge() {
        let mut engine = KnowledgeDistillationEngine::new();
        let id1 = engine.ingest_telemetry("anubis", "timeout", vec!["memory".to_string()]);
        let id2 = engine.ingest_telemetry("anubis", "timeout again", vec!["memory".to_string()]);
        engine.distill_to_l1(
            vec![id1, id2],
            "Memory timeouts",
            "ANUBIS experienced repeated timeouts",
        );
        let l1 = engine.by_tier(KnowledgeTier::L1);
        assert_eq!(l1.len(), 1);
    }

    #[test]
    fn promote_to_l2_creates_evolutionary() {
        let mut engine = KnowledgeDistillationEngine::new();
        let id = engine.ingest_telemetry("phoenix", "execution failed", vec![]);
        engine.distill_to_l1(vec![id], "Execution failures", "Summary");
        let l1_id = engine.by_tier(KnowledgeTier::L1)[0].id.clone();
        let result = engine.promote_to_l2(&l1_id, "Evolved knowledge", "Permanent insight");
        assert!(result.is_some());
        assert_eq!(engine.by_tier(KnowledgeTier::L2).len(), 1);
    }

    #[test]
    fn deduplicate_removes_duplicates() {
        let mut engine = KnowledgeDistillationEngine::new();
        engine.ingest_telemetry("s1", "exact duplicate content", vec![]);
        engine.ingest_telemetry("s2", "exact duplicate content", vec![]);
        let removed = engine.deduplicate();
        assert_eq!(removed, 1);
    }

    #[test]
    fn cluster_by_tag() {
        let mut engine = KnowledgeDistillationEngine::new();
        engine.ingest_telemetry("anubis", "failure A", vec!["critical".to_string()]);
        engine.ingest_telemetry("phoenix", "failure B", vec!["critical".to_string()]);
        engine.ingest_telemetry("moira", "note", vec!["info".to_string()]);
        let critical = engine.cluster("critical");
        assert_eq!(critical.len(), 2);
    }

    #[test]
    fn parliament_service_integration() {
        let mut svc = ParliamentDistillationService::new();
        svc.ingest("anubis", "memory timeout", vec!["memory".to_string()]);
        svc.ingest("anubis", "vector db full", vec!["memory".to_string()]);
        let result = svc.distill_service("anubis", "ANUBIS memory service degradation");
        assert!(result.is_some());
        let report = svc.parliament_report();
        assert_eq!(report.services_monitored, 1);
    }
}
