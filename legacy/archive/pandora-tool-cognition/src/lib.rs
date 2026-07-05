//! Pandora Tool Cognition — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapability {
    pub tool_name: String,

    pub reasoning_score: f64,

    pub automation_score: f64,

    pub reliability_score: f64,

    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSelection {
    pub tool_name: String,

    pub suitability: f64,

    pub rationale: String,
}

pub struct ToolCognitionEngine;

impl ToolCognitionEngine {
    pub fn select(workload: &str, tools: &[ToolCapability]) -> Vec<ToolSelection> {
        println!("[TOOLS] workload={}", workload);

        let mut selected = Vec::new();

        for tool in tools {
            let mut score = (tool.reasoning_score * 0.35)
                + (tool.automation_score * 0.35)
                + (tool.reliability_score * 0.30);

            for domain in &tool.domains {
                if workload.contains(domain) {
                    score += 0.15;
                }
            }

            println!("[TOOLS] {} score={}", tool.tool_name, score);

            selected.push(ToolSelection {
                tool_name: tool.tool_name.clone(),

                suitability: score,

                rationale: format!("{} optimized for {}", tool.tool_name, workload),
            });
        }

        selected.sort_by(|a, b| b.suitability.partial_cmp(&a.suitability).unwrap());

        selected
    }
}
