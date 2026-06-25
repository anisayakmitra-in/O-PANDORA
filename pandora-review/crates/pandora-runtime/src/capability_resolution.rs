use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDomain {
    pub domain: String,

    pub complexity: f64,

    pub governance_risk: f64,

    pub hardware_pressure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGene {
    pub gene_id: String,

    pub category: String,

    pub supported_domains: Vec<String>,

    pub governance_score: f64,

    pub execution_stability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResolution {
    pub selected_gene: String,

    pub selected_harness: String,

    pub governance_required: bool,

    pub heterogeneous_execution: bool,

    pub execution_topology: String,
}

pub struct CapabilityResolutionEngine;

impl CapabilityResolutionEngine {
    pub fn resolve(
        workload: &str,

        domains: &[CapabilityDomain],

        genes: &[CapabilityGene],
    ) -> Vec<CapabilityResolution> {
        let mut resolutions = Vec::new();

        println!("[CAPABILITY] workload={}", workload);

        for domain in domains {
            println!("[CAPABILITY] domain={}", domain.domain);

            let mut best_gene = None;

            let mut highest = 0.0;

            for gene in genes {
                if !gene.supported_domains.contains(&domain.domain) {
                    continue;
                }

                let score = (gene.governance_score * 0.55) + (gene.execution_stability * 0.45);

                if score > highest {
                    highest = score;

                    best_gene = Some(gene.clone());
                }
            }

            if let Some(gene) = best_gene {
                let harness = if domain.domain.contains("vlsi") {
                    "EDA-HARNESS"
                } else if domain.domain.contains("embedded") {
                    "EMBEDDED-HARNESS"
                } else if domain.domain.contains("quantum") {
                    "HARDWARE-HARNESS"
                } else if domain.domain.contains("compiler") {
                    "COMPILER-HARNESS"
                } else {
                    "GENERAL-HARNESS"
                };

                let topology = if domain.hardware_pressure > 0.80 {
                    "heterogeneous-distributed"
                } else if domain.complexity > 0.75 {
                    "recursive-governed"
                } else {
                    "stable-execution"
                };

                resolutions.push(CapabilityResolution {
                    selected_gene: gene.gene_id,

                    selected_harness: harness.into(),

                    governance_required: domain.governance_risk > 0.65,

                    heterogeneous_execution: domain.hardware_pressure > 0.70,

                    execution_topology: topology.into(),
                });
            }
        }

        resolutions
    }
}
