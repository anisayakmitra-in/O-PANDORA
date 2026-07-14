//! Execution Provenance Graph — canonical runtime record.
//!
//! Every execution produces a directed acyclic graph (DAG) of nodes and edges.
//! DecisionLog, replay, inspect, telemetry, and analytics are all projections
//! over this single graph. The graph is append-only — once recorded, nodes and
//! edges are immutable.
//!
//! # Architecture
//!
//! ```text
//! Task
//!  │
//!  ▼
//! ExecutionPlan
//!  │
//!  ▼
//! Workflow
//!  │
//!  ├──────────────┐
//!  ▼              ▼
//! Harness A    Harness B
//!  │              │
//!  ▼              ▼
//! Gene A        Gene B
//!  │              │
//!  └──────┬───────┘
//!         ▼
//!      Provider
//!         │
//!         ▼
//!     Evaluator
//!         │
//!         ▼
//!     Decision
//!         │
//!         ▼
//!     Outcome
//!         │
//!         ▼
//!     Session
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A node in the execution provenance graph.
/// Each node represents a distinct runtime artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceNode {
    /// Unique node ID (e.g. "task-24af31", "plan-24af31").
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// The kind of node.
    pub kind: NodeKind,
    /// Arbitrary key-value metadata (stage output, scores, timings).
    pub metadata: HashMap<String, String>,
}

/// The kind of a provenance node.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeKind {
    Task,
    ExecutionPlan,
    Workflow,
    Harness,
    Gene,
    Provider,
    Evaluator,
    Decision,
    Outcome,
    Session,
    /// Extended by packages for custom node types.
    Custom(String),
}

/// A directed edge between two provenance nodes.
/// Records WHY the transition happened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEdge {
    /// Source node ID.
    pub from: String,
    /// Target node ID.
    pub to: String,
    /// Why this edge exists (e.g. "selected by controller").
    pub reason: String,
    /// Optional confidence score for probabilistic edges.
    pub confidence: Option<f32>,
    /// Optional latency/duration for timed edges.
    pub duration_ms: Option<u64>,
}

impl ProvenanceEdge {
    pub fn new(from: impl Into<String>, to: impl Into<String>, reason: impl Into<String>) -> Self {
        Self { from: from.into(), to: to.into(), reason: reason.into(), confidence: None, duration_ms: None }
    }
    pub fn with_confidence(mut self, c: f32) -> Self { self.confidence = Some(c); self }
    pub fn with_duration(mut self, ms: u64) -> Self { self.duration_ms = Some(ms); self }
}

/// The execution provenance graph — a DAG of nodes and directed edges.
///
/// This is the **canonical runtime record**. Everything else (DecisionLog,
/// replay, inspect, telemetry) is a projection over this graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionProvenanceGraph {
    /// All nodes in the graph.
    pub nodes: HashMap<String, ProvenanceNode>,
    /// All edges in the graph.
    pub edges: Vec<ProvenanceEdge>,
    /// The execution ID this graph belongs to.
    pub execution_id: String,
}

impl ExecutionProvenanceGraph {
    pub fn new(execution_id: impl Into<String>) -> Self {
        Self { nodes: HashMap::new(), edges: Vec::new(), execution_id: execution_id.into() }
    }

    /// Add a node to the graph. Returns the node ID.
    pub fn add_node(&mut self, kind: NodeKind, id: impl Into<String>, label: impl Into<String>) -> String {
        let id = id.into();
        let node = ProvenanceNode { id: id.clone(), label: label.into(), kind, metadata: HashMap::new() };
        self.nodes.insert(id.clone(), node);
        id
    }

    /// Add a node with metadata.
    pub fn add_node_with_meta(&mut self, kind: NodeKind, id: impl Into<String>, label: impl Into<String>, meta: HashMap<String, String>) -> String {
        let id = id.into();
        let node = ProvenanceNode { id: id.clone(), label: label.into(), kind, metadata: meta };
        self.nodes.insert(id.clone(), node);
        id
    }

    /// Add an edge between two nodes.
    pub fn add_edge(&mut self, edge: ProvenanceEdge) {
        self.edges.push(edge);
    }

    /// Convenience: add a directed edge from one node id to another.
    pub fn connect(&mut self, from: impl Into<String>, to: impl Into<String>, reason: impl Into<String>) {
        self.edges.push(ProvenanceEdge::new(from, to, reason));
    }

    /// Get all outgoing edges from a node.
    pub fn outgoing(&self, node_id: &str) -> Vec<&ProvenanceEdge> {
        self.edges.iter().filter(|e| e.from == node_id).collect()
    }

    /// Get all incoming edges to a node.
    pub fn incoming(&self, node_id: &str) -> Vec<&ProvenanceEdge> {
        self.edges.iter().filter(|e| e.to == node_id).collect()
    }

    /// Get a node by ID.
    pub fn node(&self, id: &str) -> Option<&ProvenanceNode> {
        self.nodes.get(id)
    }

    /// Find all nodes of a given kind.
    pub fn nodes_by_kind(&self, kind: NodeKind) -> Vec<&ProvenanceNode> {
        self.nodes.values().filter(|n| n.kind == kind).collect()
    }

    /// Get the root node (Task kind) if any.
    pub fn root(&self) -> Option<&ProvenanceNode> {
        self.nodes.values().find(|n| n.kind == NodeKind::Task)
    }

    /// Number of nodes in the graph.

    /// Blast radius from Gortex — all nodes downstream of this one.
    pub fn blast_radius(&self, node_id: &str) -> Vec<&str> {
        self.edges.iter().filter(|e| e.from == node_id).map(|e| e.to.as_str()).collect()
    }
    pub fn node_count(&self) -> usize { self.nodes.len() }
    /// Number of edges in the graph.
    pub fn edge_count(&self) -> usize { self.edges.len() }

    /// Produce a textual representation suitable for `pandora graph`.
    pub fn render(&self) -> String {
        let mut out = format!("Execution Provenance Graph — {}\n\n", self.execution_id);
        // Group nodes by kind
        let mut by_kind: HashMap<NodeKind, Vec<&ProvenanceNode>> = HashMap::new();
        for node in self.nodes.values() {
            by_kind.entry(node.kind.clone()).or_default().push(node);
        }
        for (kind, nodes) in &by_kind {
            out.push_str(&format!("  {:?} ({}):\n", kind, nodes.len()));
            for node in nodes {
                out.push_str(&format!("    {} — {}\n", node.id, node.label));
                for (k, v) in &node.metadata {
                    out.push_str(&format!("      {}: {}\n", k, v));
                }
            }
            out.push('\n');
        }
        out.push_str("  Edges:\n");
        for edge in &self.edges {
            out.push_str(&format!("    {} → {}: {}\n", edge.from, edge.to, edge.reason));
        }
        out
    }
}

/// Projection: DecisionLog from the graph.
/// Walks the graph and extracts decision nodes and their edges.
pub fn extract_decision_log(graph: &ExecutionProvenanceGraph) -> Vec<String> {
    let mut log = Vec::new();
    let decisions = graph.nodes_by_kind(NodeKind::Decision);
    for d in decisions {
        let incoming = graph.incoming(&d.id);
        let outgoing = graph.outgoing(&d.id);
        let mut entry = format!("Decision: {} — {}", d.id, d.label);
        for e in &incoming {
            if let Some(src) = graph.node(&e.from) {
                entry.push_str(&format!("\n  From: {} ({:?})", src.label, src.kind));
            }
            entry.push_str(&format!("\n  Reason: {}", e.reason));
        }
        for e in &outgoing {
            if let Some(dst) = graph.node(&e.to) {
                entry.push_str(&format!("\n  To: {} ({:?})", dst.label, dst.kind));
            }
        }
        log.push(entry);
    }
    log
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> ExecutionProvenanceGraph {
        let mut g = ExecutionProvenanceGraph::new("exec-24af31");
        g.add_node(NodeKind::Task, "task-1", "Implement JWT auth");
        g.add_node(NodeKind::ExecutionPlan, "plan-1", "SingleShot with RustTests evaluator");
        g.add_node(NodeKind::Workflow, "wf-1", "auto-workflow (2 steps)");
        g.add_node(NodeKind::Harness, "harness-coding", "CodingDomainHarness");
        g.add_node(NodeKind::Gene, "gene-shell", "ShellGene");
        g.add_node(NodeKind::Provider, "provider-ollama", "Ollama (default-model)");
        g.add_node(NodeKind::Evaluator, "eval-rust", "RustTestsEvaluator");
        g.add_node(NodeKind::Decision, "dec-select-provider", "Provider Selection");
        g.add_node(NodeKind::Outcome, "outcome-1", "Completed — all tests pass");
        g.connect("task-1", "plan-1", "controller call");
        g.connect("plan-1", "wf-1", "workflow instantiation");
        g.connect("wf-1", "harness-coding", "domain dispatch");
        g.connect("harness-coding", "gene-shell", "gene resolution");
        g.connect("gene-shell", "provider-ollama", "provider execution");
        g.connect("provider-ollama", "dec-select-provider", "evaluation");
        g.connect("dec-select-provider", "outcome-1", "accepted (score 0.95)");
        g
    }

    #[test]
    fn graph_creation() {
        let g = sample_graph();
        assert_eq!(g.node_count(), 9);
        assert_eq!(g.edge_count(), 7);
    }

    #[test]
    fn graph_root() {
        let g = sample_graph();
        let root = g.root().unwrap();
        assert_eq!(root.kind, NodeKind::Task);
        assert_eq!(root.label, "Implement JWT auth");
    }

    #[test]
    fn outgoing_edges() {
        let g = sample_graph();
        let outgoing = g.outgoing("task-1");
        assert_eq!(outgoing.len(), 1);
        assert_eq!(outgoing[0].to, "plan-1");
    }

    #[test]
    fn incoming_edges() {
        let g = sample_graph();
        let incoming = g.incoming("outcome-1");
        assert_eq!(incoming.len(), 1);
        assert_eq!(incoming[0].from, "dec-select-provider");
    }

    #[test]
    fn nodes_by_kind() {
        let g = sample_graph();
        let kinds: Vec<NodeKind> = vec![NodeKind::Task, NodeKind::Decision, NodeKind::Outcome];
        for kind in &kinds {
            assert!(!g.nodes_by_kind(kind.clone()).is_empty(), "should have {:?} nodes", kind);
        }
    }

    #[test]
    fn decision_log_projection() {
        let g = sample_graph();
        let log = extract_decision_log(&g);
        assert!(!log.is_empty(), "should have at least 1 decision");
        assert!(log[0].contains("Decision:"));
    }

    #[test]
    fn graph_render_produces_text() {
        let g = sample_graph();
        let rendered = g.render();
        assert!(rendered.contains("exec-24af31"));
        assert!(rendered.contains("→"));
    }
}
