//! Pandora Swarm Genome — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmGenome {
    pub genome_id: String,

    pub traits: Vec<String>,

    pub fitness: f32,

    pub generation: u32,
}

pub struct GenomeEngine;

impl GenomeEngine {
    pub fn mutate(genome: &SwarmGenome) -> SwarmGenome {
        let mut traits = genome.traits.clone();

        traits.push("adaptive-routing".into());

        println!("[GENOME] mutating {}", genome.genome_id);

        SwarmGenome {
            genome_id: format!("{}-mutated", genome.genome_id),

            traits,

            fitness: genome.fitness + 0.04,

            generation: genome.generation + 1,
        }
    }
}
