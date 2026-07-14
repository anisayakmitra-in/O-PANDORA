//! Pandora Gene Orchestrator — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneCapsule {
    pub gene_id: String,

    pub specialization: String,

    pub survivability: f64,

    pub governance_score: f64,

    pub activation_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaHarness {
    pub harness_id: String,

    pub topology: String,

    pub stability: f64,

    pub recursion_limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneExecutionPlan {
    pub selected_gene: String,

    pub selected_harness: String,

    pub deployment_mode: String,

    pub approved: bool,
}

pub struct GeneOrchestrator;

impl GeneOrchestrator {
    pub fn orchestrate(
        workload: &str,

        genes: &[GeneCapsule],

        harnesses: &[MetaHarness],
    ) -> Option<GeneExecutionPlan> {
        println!("[GENE] workload={}", workload);

        let mut best_gene = None;

        let mut best_score = 0.0;

        for gene in genes {
            println!("[GENE] evaluating {}", gene.gene_id);

            let score = (gene.survivability * 0.45) + (gene.governance_score * 0.40)
                - (gene.activation_cost * 0.15);

            if workload.contains(&gene.specialization) && score > best_score {
                best_score = score;

                best_gene = Some(gene.clone());
            }
        }

        let selected_gene = best_gene?;

        let harness = harnesses
            .iter()
            .max_by(|a, b| a.stability.partial_cmp(&b.stability).unwrap())?;

        let deployment = if harness.recursion_limit > 8 {
            "deep-recursive"
        } else {
            "stable-execution"
        };

        Some(GeneExecutionPlan {
            selected_gene: selected_gene.gene_id,

            selected_harness: harness.harness_id.clone(),

            deployment_mode: deployment.into(),

            approved: selected_gene.governance_score > 0.75,
        })
    }
}
