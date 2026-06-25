use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstFunction {
    pub name: String,

    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstAnalysis {
    pub functions: Vec<AstFunction>,

    pub total_lines: usize,
}

pub struct AstEngine;

impl AstEngine {
    pub fn analyze(source: &str) -> AstAnalysis {
        let mut functions = Vec::new();

        for (index, line) in source.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with("fn ") {
                let name = trimmed
                    .replace("fn ", "")
                    .split('(')
                    .next()
                    .unwrap_or("unknown")
                    .trim()
                    .to_string();

                println!("[AST] function={} line={}", name, index + 1);

                functions.push(AstFunction {
                    name,

                    line: index + 1,
                });
            }
        }

        AstAnalysis {
            functions,

            total_lines: source.lines().count(),
        }
    }
}
