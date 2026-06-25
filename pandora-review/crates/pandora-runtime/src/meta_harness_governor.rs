use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaHarnessGovernor {
    pub harness_id: String,

    pub domain: String,

    pub recursion_limit: usize,

    pub survivability_threshold: f64,

    pub approved_tools: Vec<String>,

    pub approved_models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernedGene {
    pub gene_id: String,

    pub specialization: String,

    pub governance_score: f64,

    pub survivability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceExecution {
    pub harness: String,

    pub gene: String,

    pub approved: bool,

    pub execution_mode: String,

    pub oversight_required: bool,
}

pub struct MetaHarnessExecutionGovernor;

impl MetaHarnessExecutionGovernor {
    pub fn authorize(
        workload: &str,

        governor: &MetaHarnessGovernor,

        genes: &[GovernedGene],
    ) -> Option<GovernanceExecution> {
        println!("[HARNESS] workload={}", workload);

        println!("[HARNESS] governor={}", governor.harness_id);

        let mut selected = None;

        let mut highest = 0.0;

        for gene in genes {
            println!("[HARNESS] evaluating gene={}", gene.gene_id);

            if !workload.contains(&gene.specialization) {
                continue;
            }

            let score = (gene.governance_score * 0.55) + (gene.survivability * 0.45);

            if score > highest {
                highest = score;

                selected = Some(gene.clone());
            }
        }

        let gene = selected?;

        let approved = gene.survivability >= governor.survivability_threshold;

        let execution_mode = if governor.recursion_limit > 10 {
            "deep-recursive"
        } else if governor.recursion_limit > 5 {
            "controlled-recursive"
        } else {
            "stable-execution"
        };

        let oversight_required = gene.governance_score < 0.80;

        Some(GovernanceExecution {
            harness: governor.harness_id.clone(),

            gene: gene.gene_id.clone(),

            approved,

            execution_mode: execution_mode.into(),

            oversight_required,
        })
    }
}
