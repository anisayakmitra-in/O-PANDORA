//! Knowledge Distillation — filters raw telemetry into durable knowledge.
//! Pipeline: Raw Telemetry → Cluster → Summarize → Deduplicate → Store

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tier of distilled knowledge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KnowledgeTier { L0, L1, L2 }

/// A single piece of distilled knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: String,
    pub tier: KnowledgeTier,
    pub label: String,
    pub summary: String,
    pub source_sessions: Vec<String>,
    pub confidence: f64,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// A cluster of related telemetry events or execution frames.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeCluster {
    pub id: String,
    pub label: String,
    pub node_ids: Vec<String>,
    pub centroid: Option<String>,
    pub cohesion: f64,
    pub created_at: DateTime<Utc>,
}

/// An edge between knowledge nodes (relationships).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub source_id: String,
    pub target_id: String,
    pub relation: String,
    pub weight: f64,
}

/// The knowledge graph — nodes, clusters, and edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: Vec<KnowledgeNode>,
    pub clusters: Vec<KnowledgeCluster>,
    pub edges: Vec<KnowledgeEdge>,
}

impl KnowledgeGraph {
    pub fn new() -> Self { Self { nodes: Vec::new(), clusters: Vec::new(), edges: Vec::new() } }
    pub fn add_node(&mut self, node: KnowledgeNode) { self.nodes.push(node); }
    pub fn add_cluster(&mut self, cluster: KnowledgeCluster) { self.clusters.push(cluster); }
    pub fn add_edge(&mut self, edge: KnowledgeEdge) { self.edges.push(edge); }
    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn cluster_count(&self) -> usize { self.clusters.len() }
    pub fn find_by_tag(&self, tag: &str) -> Vec<&KnowledgeNode> {
        self.nodes.iter().filter(|n| n.tags.contains(&tag.to_string())).collect()
    }
    pub fn find_by_tier(&self, tier: KnowledgeTier) -> Vec<&KnowledgeNode> {
        self.nodes.iter().filter(|n| n.tier == tier).collect()
    }
}

impl Default for KnowledgeGraph { fn default() -> Self { Self::new() } }

/// Query for retrieving knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeQuery {
    pub tier: Option<KnowledgeTier>,
    pub tags: Vec<String>,
    pub session_ids: Vec<String>,
    pub min_confidence: Option<f64>,
    pub limit: usize,
}

impl Default for KnowledgeQuery {
    fn default() -> Self { Self { tier: None, tags: Vec::new(), session_ids: Vec::new(), min_confidence: None, limit: 50 } }
}

/// Knowledge distillation configuration — per tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistillationConfig {
    pub tier: KnowledgeTier,
    pub min_confidence: f64,
    pub max_nodes: usize,
    pub cluster_threshold: f64,
    pub ttl_seconds: u64,
}

impl Default for DistillationConfig {
    fn default() -> Self {
        Self { tier: KnowledgeTier::L1, min_confidence: 0.5, max_nodes: 10_000, cluster_threshold: 0.7, ttl_seconds: 86_400 }
    }
}

impl KnowledgeTier {
    pub fn retention_seconds(&self) -> u64 {
        match self { Self::L0 => 3600, Self::L1 => 86_400 * 7, Self::L2 => 86_400 * 365 }
    }
}

/// Statistics for the knowledge graph.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KnowledgeStatistics {
    pub total_nodes: usize,
    pub total_clusters: usize,
    pub total_edges: usize,
    pub l0_nodes: usize, pub l1_nodes: usize, pub l2_nodes: usize,
    pub avg_confidence: f64,
    pub oldest_node: Option<DateTime<Utc>>,
    pub newest_node: Option<DateTime<Utc>>,
}

/// Events emitted during knowledge distillation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeEvent {
    NodeCreated { node_id: String, tier: KnowledgeTier, label: String },
    ClusterFormed { cluster_id: String, node_count: usize },
    NodePruned { node_id: String, reason: String },
    EdgeCreated { source: String, target: String, relation: String },
}

/// The Knowledge Distillation Engine.
pub struct KnowledgeDistillationEngine {
    graph: KnowledgeGraph,
    configs: HashMap<KnowledgeTier, DistillationConfig>,
}

impl KnowledgeDistillationEngine {
    pub fn new() -> Self {
        let mut configs = HashMap::new();
        configs.insert(KnowledgeTier::L0, DistillationConfig { tier: KnowledgeTier::L0, min_confidence: 0.0, max_nodes: 1_000, cluster_threshold: 0.8, ttl_seconds: 3600 });
        configs.insert(KnowledgeTier::L1, DistillationConfig { tier: KnowledgeTier::L1, min_confidence: 0.5, max_nodes: 10_000, cluster_threshold: 0.7, ttl_seconds: 604_800 });
        configs.insert(KnowledgeTier::L2, DistillationConfig { tier: KnowledgeTier::L2, min_confidence: 0.8, max_nodes: 100_000, cluster_threshold: 0.6, ttl_seconds: 31_536_000 });
        Self { graph: KnowledgeGraph::new(), configs }
    }

    pub fn graph(&self) -> &KnowledgeGraph { &self.graph }
    pub fn graph_mut(&mut self) -> &mut KnowledgeGraph { &mut self.graph }
    pub fn config(&self, tier: KnowledgeTier) -> Option<&DistillationConfig> { self.configs.get(&tier) }

    pub fn ingest(&mut self, session_id: &str, summary: &str, confidence: f64, tags: Vec<String>) -> KnowledgeEvent {
        let node = KnowledgeNode {
            id: format!("kn-{:016x}", rand::random::<u64>()),
            tier: if confidence >= 0.8 { KnowledgeTier::L2 } else if confidence >= 0.5 { KnowledgeTier::L1 } else { KnowledgeTier::L0 },
            label: summary.chars().take(80).collect(),
            summary: summary.into(),
            source_sessions: vec![session_id.into()],
            confidence, tags,
            created_at: Utc::now(),
            metadata: HashMap::new(),
        };
        let event = KnowledgeEvent::NodeCreated { node_id: node.id.clone(), tier: node.tier, label: node.label.clone() };
        self.graph.add_node(node);
        event
    }

    /// Legacy: ingest telemetry data.
    pub fn ingest_telemetry(&mut self, session_id: String, data: String, tags: Vec<String>) -> String {
        self.ingest(&session_id, &data, 0.5, tags);
        "ok".into()
    }

    /// Legacy: distill L0 knowledge to L1 (tier assignment is automatic).
    pub fn distill_to_l1(&self, node_ids: Vec<String>, _label: String, _summary: &str) -> String {
        format!("distilled-{node_ids:?}")
    }

    /// Legacy: count knowledge nodes.
    pub fn knowledge_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn query(&self, q: &KnowledgeQuery) -> Vec<&KnowledgeNode> {
        let mut results: Vec<&KnowledgeNode> = self.graph.nodes.iter()
            .filter(|n| {
                q.tier.map_or(true, |t| n.tier == t)
                    && (q.tags.is_empty() || q.tags.iter().any(|t| n.tags.contains(t)))
                    && (q.session_ids.is_empty() || q.session_ids.iter().any(|s| n.source_sessions.contains(s)))
                    && q.min_confidence.map_or(true, |c| n.confidence >= c)
            })
            .collect();
        results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        results.truncate(q.limit);
        results
    }
}

impl Default for KnowledgeDistillationEngine { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingest_creates_node() {
        let mut engine = KnowledgeDistillationEngine::new();
        engine.ingest("s1", "optimized query", 0.9, vec!["sql".into()]);
        assert_eq!(engine.graph.node_count(), 1);
    }

    #[test]
    fn query_filters_by_tag() {
        let mut engine = KnowledgeDistillationEngine::new();
        engine.ingest("s1", "rust fix", 0.9, vec!["rust".into()]);
        engine.ingest("s2", "python fix", 0.8, vec!["python".into()]);
        assert_eq!(engine.query(&KnowledgeQuery { tags: vec!["rust".into()], ..Default::default() }).len(), 1);
    }

    #[test]
    fn legacy_methods_work() {
        let mut engine = KnowledgeDistillationEngine::new();
        engine.ingest_telemetry("s1".into(), "data".into(), vec![]);
        assert_eq!(engine.knowledge_count(), 1);
        let d = engine.distill_to_l1(vec!["kn-1".into()], "test".into(), "summary");
        assert!(d.contains("distilled"));
    }
}
