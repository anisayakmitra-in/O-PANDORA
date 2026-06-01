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
