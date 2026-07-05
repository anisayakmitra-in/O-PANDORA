//! Pandora Dependency Graph — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub file: String,

    pub imports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    pub nodes: HashMap<String, DependencyNode>,
}

pub struct DependencyGraphEngine;

impl DependencyGraphEngine {
    pub fn analyze(files: &[(String, String)]) -> DependencyGraph {
        let mut nodes = HashMap::new();

        for (file_name, source) in files {
            let mut imports = Vec::new();

            for line in source.lines() {
                let trimmed = line.trim();

                if trimmed.starts_with("use ") {
                    imports.push(trimmed.replace("use ", "").replace(";", ""));
                }
            }

            println!("[GRAPH] {} imports={}", file_name, imports.len());

            nodes.insert(
                file_name.clone(),
                DependencyNode {
                    file: file_name.clone(),

                    imports,
                },
            );
        }

        DependencyGraph { nodes }
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn register(&mut self, node: DependencyNode) {
        self.nodes.insert(node.file.clone(), node);
    }

    pub fn dependencies(&self, key: &str) -> Vec<String> {
        self.nodes
            .get(key)
            .map(|n| n.imports.clone())
            .unwrap_or_default()
    }
}
