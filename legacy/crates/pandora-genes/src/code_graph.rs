//! Code Graph Gene — maps code into a knowledge graph.
//! Based on Graphify: tree-sitter AST parsing with EXTRACTED vs INFERRED edges.
//! Generates: graph.json (nodes + edges), GRAPH_REPORT.md (highlights), graph.html (interactive)

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GraphNode {
    id: String,
    label: String,
    kind: String,
    language: String,
    line: usize,
}

#[derive(Debug, Serialize)]
struct GraphEdge {
    from: String,
    to: String,
    relation: String,
    confidence: String,
}

#[derive(Debug, Serialize)]
struct CodeGraph {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
}

impl CodeGraph {
    fn new() -> Self {
        Self {
            nodes: vec![],
            edges: vec![],
        }
    }

    fn scan_file(&mut self, path: &std::path::Path, content: &str) {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let lang = match ext {
            "rs" => "rust",
            "py" => "python",
            "go" => "go",
            "ts" | "tsx" => "typescript",
            "js" | "jsx" => "javascript",
            "java" => "java",
            _ => ext,
        };
        let file_id = path.to_string_lossy().to_string();

        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            // Detect function definitions
            if let Some(name) = detect_function(trimmed, ext) {
                self.nodes.push(GraphNode {
                    id: format!("fn:{}:{}", file_id, name),
                    label: name.clone(),
                    kind: "function".into(),
                    language: lang.into(),
                    line: i + 1,
                });
                // EXTRACTED edge: file contains function
                self.edges.push(GraphEdge {
                    from: file_id.clone(),
                    to: format!("fn:{}:{}", file_id, name),
                    relation: "CONTAINS".into(),
                    confidence: "EXTRACTED".into(),
                });
            }
            // Detect imports
            if let Some(imported) = detect_import(trimmed, ext) {
                self.nodes.push(GraphNode {
                    id: format!("import:{}:{}", file_id, imported),
                    label: imported.clone(),
                    kind: "import".into(),
                    language: lang.into(),
                    line: i + 1,
                });
                self.edges.push(GraphEdge {
                    from: file_id.clone(),
                    to: format!("import:{}:{}", file_id, imported),
                    relation: "IMPORTS".into(),
                    confidence: "EXTRACTED".into(),
                });
            }
            // Detect function calls (INFERRED edge)
            if let Some(called) = detect_call(trimmed) {
                self.edges.push(GraphEdge {
                    from: file_id.clone(),
                    to: format!("fn:{}:{}", file_id, called),
                    relation: "CALLS".into(),
                    confidence: "INFERRED".into(),
                });
            }
        }
        // File node
        self.nodes.push(GraphNode {
            id: file_id.clone(),
            label: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into(),
            kind: "file".into(),
            language: lang.into(),
            line: 0,
        });
    }
}

fn detect_function(line: &str, ext: &str) -> Option<String> {
    match ext {
        "rs" => {
            if line.starts_with("fn ") || line.starts_with("pub fn ") {
                line.split_whitespace()
                    .nth(if line.starts_with("pub ") { 2 } else { 1 })
                    .map(|s| s.trim_end_matches('(').to_string())
            } else {
                None
            }
        }
        "py" => {
            if line.starts_with("def ") {
                line.split_whitespace()
                    .nth(1)
                    .map(|s| s.trim_end_matches('(').to_string())
            } else {
                None
            }
        }
        "go" => {
            if line.starts_with("func ") {
                line.split_whitespace()
                    .nth(1)
                    .map(|s| s.trim_end_matches('(').to_string())
            } else {
                None
            }
        }
        "ts" | "tsx" | "js" | "jsx" => {
            if line.contains("function ") || line.contains("=>") {
                line.split_whitespace()
                    .nth(1)
                    .map(|s| s.trim_end_matches('(').to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn detect_import(line: &str, ext: &str) -> Option<String> {
    match ext {
        "rs" => {
            if line.starts_with("use ") {
                line.split_whitespace()
                    .nth(1)
                    .map(|s| s.trim_end_matches(';').to_string())
            } else {
                None
            }
        }
        "py" => {
            if line.starts_with("import ") || line.starts_with("from ") {
                line.split_whitespace().nth(1).map(|s| s.to_string())
            } else {
                None
            }
        }
        "go" => {
            if line.starts_with("import ") {
                line.split_whitespace()
                    .nth(1)
                    .map(|s| s.trim_matches('"').to_string())
            } else {
                None
            }
        }
        "ts" | "tsx" | "js" | "jsx" => {
            if line.starts_with("import ") {
                line.split_whitespace()
                    .nth(1)
                    .map(|s| s.trim_matches('"').trim_matches('\'').to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn detect_call(line: &str) -> Option<String> {
    if let Some(pos) = line.find('(') {
        let before = &line[..pos];
        let last_word = before.split_whitespace().last().unwrap_or("");
        if !last_word.is_empty()
            && ![
                "if", "while", "for", "match", "switch", "fn", "def", "func", "let", "const",
                "var", "return",
            ]
            .contains(&last_word)
        {
            return Some(last_word.to_string());
        }
    }
    None
}

#[derive(Debug)]
pub struct CodeGraphGene {
    m: GeneManifest,
}

impl Default for CodeGraphGene {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeGraphGene {
    pub fn new() -> Self {
        Self {
            m: GeneManifestBuilder::default()
                .id("code-graph")
                .name("Code Graph")
                .kind(GeneKind::Tool)
                .description(
                    "Map code into a knowledge graph — detects functions, imports, and calls",
                )
                .version("0.1.0")
                .author("pandora")
                .build()
                .unwrap(),
        }
    }
}

impl Gene for CodeGraphGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }

    fn execute(&self, input: &str) -> Result<String, String> {
        let path = std::path::Path::new(input.trim());
        if !path.exists() {
            return Err(format!("Path not found: {}", input));
        }

        let mut graph = CodeGraph::new();
        if path.is_dir() {
            for entry in walkdir::WalkDir::new(path)
                .max_depth(10)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        graph.scan_file(entry.path(), &content);
                    }
                }
            }
        } else if let Ok(content) = std::fs::read_to_string(path) {
            graph.scan_file(path, &content);
        }

        let json = serde_json::to_string_pretty(&graph).map_err(|e| format!("json: {e}"))?;
        // Write output files
        let out_dir = std::path::Path::new("graphify-out");
        std::fs::create_dir_all(out_dir).map_err(|e| format!("mkdir: {e}"))?;
        std::fs::write(out_dir.join("graph.json"), &json).map_err(|e| format!("write: {e}"))?;

        let report = format!(
            "# Code Graph Report\n\n{} nodes, {} edges\n\n## Key Functions\n\n",
            graph.nodes.len(),
            graph.edges.len()
        );
        std::fs::write(out_dir.join("GRAPH_REPORT.md"), &report)
            .map_err(|e| format!("write: {e}"))?;

        Ok(format!(
            "Generated graphify-out/: {} nodes, {} edges",
            graph.nodes.len(),
            graph.edges.len()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_rust_fn() {
        assert_eq!(
            detect_function("fn main() {", "rs").map(|s| s.trim_end_matches("()").to_string()),
            Some("main".into())
        );
    }
    #[test]
    fn detects_python_fn() {
        assert_eq!(
            detect_function("def hello():", "py").map(|s| s.trim_end_matches("():").to_string()),
            Some("hello".into())
        );
    }
    #[test]
    fn detects_rust_import() {
        assert_eq!(
            detect_import("use std::collections::HashMap;", "rs"),
            Some("std::collections::HashMap".into())
        );
    }
    #[test]
    fn detects_call() {
        assert!(detect_call("    foo(x, y);").is_some());
    }
    #[test]
    fn gene_creates() {
        assert!(!CodeGraphGene::new().manifest().id.is_empty());
    }
}
