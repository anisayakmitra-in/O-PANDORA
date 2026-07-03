//! Capability Graph Engine — dependency-aware capability reasoning.

use petgraph::algo::has_path_connecting;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityNode {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub tags: Vec<String>,
    pub installed: bool,
    pub provider_hint: Option<String>,
    pub install_cost: f64,
}

impl CapabilityNode {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        category: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            category: category.into(),
            description: String::new(),
            tags: Vec::new(),
            installed: false,
            provider_hint: None,
            install_cost: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CapabilityGraph {
    graph: DiGraph<CapabilityNode, String>,
    node_indices: HashMap<String, NodeIndex>,
}

impl CapabilityGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: CapabilityNode) {
        let id = node.id.clone();
        let idx = self.graph.add_node(node);
        self.node_indices.insert(id, idx);
    }

    pub fn add_dependency(
        &mut self,
        from: &str,
        to: &str,
        relationship: &str,
    ) -> Result<(), String> {
        let from_idx = self
            .node_indices
            .get(from)
            .ok_or_else(|| format!("node '{}' not found", from))?;
        let to_idx = self
            .node_indices
            .get(to)
            .ok_or_else(|| format!("node '{}' not found", to))?;
        self.graph
            .add_edge(*from_idx, *to_idx, relationship.to_string());
        Ok(())
    }

    pub fn is_available(&self, id: &str) -> bool {
        self.node_indices
            .get(id)
            .map(|idx| self.graph[*idx].installed)
            .unwrap_or(false)
    }

    pub fn dependencies_of(&self, id: &str) -> Vec<&CapabilityNode> {
        let start = match self.node_indices.get(id) {
            Some(idx) => *idx,
            None => return Vec::new(),
        };
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        queue.push_back(start);
        visited.insert(start);
        while let Some(current) = queue.pop_front() {
            for neighbor in self
                .graph
                .neighbors_directed(current, petgraph::Direction::Outgoing)
            {
                if visited.insert(neighbor) {
                    result.push(&self.graph[neighbor]);
                    queue.push_back(neighbor);
                }
            }
        }
        result
    }

    pub fn missing_capabilities(&self, id: &str) -> Vec<&CapabilityNode> {
        let mut missing: Vec<&CapabilityNode> = self
            .dependencies_of(id)
            .into_iter()
            .filter(|n| !n.installed)
            .collect();
        if let Some(idx) = self
            .graph
            .node_indices()
            .find(|idx| self.graph[*idx].id == id)
        {
            let node = &self.graph[idx];
            if !node.installed {
                missing.push(node);
            }
        }
        missing
    }

    pub fn find_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        let from_idx = self.node_indices.get(from)?;
        let to_idx = self.node_indices.get(to)?;
        if has_path_connecting(&self.graph, *from_idx, *to_idx, None) {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            let mut parent: HashMap<NodeIndex, NodeIndex> = HashMap::new();
            queue.push_back(*from_idx);
            visited.insert(*from_idx);
            while let Some(current) = queue.pop_front() {
                if current == *to_idx {
                    let mut path = Vec::new();
                    let mut node = current;
                    path.push(self.graph[node].id.clone());
                    while let Some(p) = parent.get(&node) {
                        path.push(self.graph[*p].id.clone());
                        node = *p;
                    }
                    path.reverse();
                    return Some(path);
                }
                for neighbor in self
                    .graph
                    .neighbors_directed(current, petgraph::Direction::Outgoing)
                {
                    if visited.insert(neighbor) {
                        parent.insert(neighbor, current);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        None
    }

    pub fn suggest_install(&self, id: &str) -> Vec<(&CapabilityNode, Vec<&CapabilityNode>)> {
        self.missing_capabilities(id)
            .into_iter()
            .map(|cap| {
                let deps = self.dependencies_of(&cap.id);
                (cap, deps)
            })
            .collect()
    }

    pub fn by_category(&self, category: &str) -> Vec<&CapabilityNode> {
        self.graph
            .node_weights()
            .filter(|n| n.category == category)
            .collect()
    }

    pub fn get(&self, id: &str) -> Option<&CapabilityNode> {
        self.node_indices.get(id).map(|idx| &self.graph[*idx])
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut CapabilityNode> {
        self.node_indices.get(id).map(|idx| &mut self.graph[*idx])
    }

    pub fn mark_installed(&mut self, id: &str) {
        if let Some(node) = self.get_mut(id) {
            node.installed = true;
        }
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl Default for CapabilityGraph {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CapabilityGraphEngine {
    graph: CapabilityGraph,
}

impl CapabilityGraphEngine {
    pub fn new() -> Self {
        Self {
            graph: CapabilityGraph::new(),
        }
    }

    pub fn graph(&self) -> &CapabilityGraph {
        &self.graph
    }
    pub fn graph_mut(&mut self) -> &mut CapabilityGraph {
        &mut self.graph
    }

    pub fn analyze_task(&self, task: &str, domain: &str) -> TaskCapabilityAnalysis {
        let relevant_caps = self.graph.by_category(domain);
        let available = relevant_caps.iter().filter(|c| c.installed).count();
        let missing = relevant_caps.iter().filter(|c| !c.installed).count();
        TaskCapabilityAnalysis {
            task: task.to_string(),
            domain: domain.to_string(),
            total_required: relevant_caps.len(),
            available,
            missing,
            missing_capabilities: relevant_caps
                .iter()
                .filter(|c| !c.installed)
                .map(|c| c.name.clone())
                .collect(),
            suggestion: if missing > 0 {
                format!(
                    "Missing {} capabilities in {}. Consider installing: {}",
                    missing,
                    domain,
                    relevant_caps
                        .iter()
                        .filter(|c| !c.installed)
                        .map(|c| c.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            } else {
                format!(
                    "All {} capabilities in {} are available.",
                    relevant_caps.len(),
                    domain
                )
            },
        }
    }

    pub fn build_standard(&mut self) {
        self.add("verilog", "Verilog", "eda", true, "yosys");
        self.add("vhdl", "VHDL", "eda", true, "ghdl");
        let _simulation = self.add("simulation", "Simulation", "eda", false, "verilator");
        let _synthesis = self.add("synthesis", "Synthesis", "eda", false, "yosys");
        let _timing = self.add(
            "timing-analysis",
            "Timing Analysis",
            "eda",
            false,
            "opensta",
        );
        let _fpga = self.add(
            "fpga-programming",
            "FPGA Programming",
            "eda",
            false,
            "nextpnr",
        );
        let _waveform = self.add(
            "waveform-viewer",
            "Waveform Viewer",
            "eda",
            false,
            "gtkwave",
        );

        let _ = self
            .graph
            .add_dependency("simulation", "waveform-viewer", "requires");
        let _ = self
            .graph
            .add_dependency("synthesis", "timing-analysis", "requires");
        let _ = self
            .graph
            .add_dependency("fpga-programming", "synthesis", "requires");
        let _ = self
            .graph
            .add_dependency("simulation", "verilog", "requires");

        // Rust capabilities
        self.add("rust-compiler", "Rust Compiler", "rust", true, "rustc");
        self.add("clippy", "Clippy", "rust", true, "clippy");
        let _wasm = self.add("wasm-target", "WASM Target", "rust", false, "wasm-pack");

        let _ = self
            .graph
            .add_dependency("clippy", "rust-compiler", "requires");
        let _ = self
            .graph
            .add_dependency("wasm-target", "clippy", "requires");
    }

    fn add(&mut self, id: &str, name: &str, category: &str, installed: bool, hint: &str) -> String {
        let mut node = CapabilityNode::new(id, name, category);
        node.installed = installed;
        node.provider_hint = Some(hint.to_string());
        self.graph.add_node(node);
        id.to_string()
    }
}

impl Default for CapabilityGraphEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCapabilityAnalysis {
    pub task: String,
    pub domain: String,
    pub total_required: usize,
    pub available: usize,
    pub missing: usize,
    pub missing_capabilities: Vec<String>,
    pub suggestion: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> CapabilityGraphEngine {
        let mut engine = CapabilityGraphEngine::new();
        engine.build_standard();
        engine
    }

    #[test]
    fn graph_creation() {
        let engine = sample();
        assert!(engine.graph().node_count() > 0);
    }

    #[test]
    fn dependency_traversal() {
        let engine = sample();
        let deps = engine.graph().dependencies_of("wasm-target");
        assert!(!deps.is_empty());
        assert!(deps.iter().any(|n| n.name == "Clippy"));
    }

    #[test]
    fn missing_capabilities_detection() {
        let engine = sample();
        let missing = engine.graph().missing_capabilities("wasm-target");
        assert!(missing.iter().any(|n| n.name == "WASM Target"));
    }

    #[test]
    fn task_analysis() {
        let engine = sample();
        let analysis = engine.analyze_task("design cpu", "eda");
        assert!(analysis.total_required > 0);
    }

    #[test]
    fn path_finding() {
        let engine = sample();
        let path = engine.graph().find_path("wasm-target", "rust-compiler");
        assert!(path.is_some());
    }

    #[test]
    fn mark_installed() {
        let mut engine = sample();
        engine.graph_mut().mark_installed("wasm-target");
        assert!(engine.graph().is_available("wasm-target"));
    }

    #[test]
    fn suggest_install() {
        let engine = sample();
        let suggestions = engine.graph().suggest_install("wasm-target");
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn by_category() {
        let engine = sample();
        let eda = engine.graph().by_category("eda");
        let rust = engine.graph().by_category("rust");
        assert!(!eda.is_empty() && !rust.is_empty());
    }
}
