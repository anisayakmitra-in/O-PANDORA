use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessGene {
    pub name: String,
    pub gene_id: String,
    pub parent_gene: Option<String>,
    pub generation: usize,

    // strengths
    pub domains: Vec<String>,

    // evolution metrics
    pub avg_score: f32,
    pub total_runs: usize,

    // runtime traits
    pub supports_tools: bool,
    pub supports_memory: bool,
    pub supports_subagents: bool,

    // future expansion
    pub tags: Vec<String>,
}
