use serde::{Deserialize, Serialize};

use crate::swarm_genome::SwarmGenome;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmPhenotype {
    pub phenotype_id: String,

    pub active_capabilities: Vec<String>,

    pub execution_bias: String,

    pub survivability_score: f32,
}

pub struct PhenotypeEngine;

impl PhenotypeEngine {
    pub fn express(genome: &SwarmGenome) -> SwarmPhenotype {
        println!("[PHENOTYPE] expressing {}", genome.genome_id);

        let execution_bias = if genome.fitness > 0.90 {
            "aggressive-scaling"
        } else {
            "stable-execution"
        };

        SwarmPhenotype {
            phenotype_id: format!("{}-phenotype", genome.genome_id),

            active_capabilities: genome.traits.clone(),

            execution_bias: execution_bias.into(),

            survivability_score: genome.fitness * 1.1,
        }
    }
}
