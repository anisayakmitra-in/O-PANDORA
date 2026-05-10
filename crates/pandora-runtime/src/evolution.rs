use crate::gene::GeneManifest;

use std::fs;

pub fn promote_winner(
    gene: &mut GeneManifest,
    winner: &EvolutionCandidate,
) {

    let archive_path =
        format!(
            "genes/archive/{}-gen-{}.json",
            gene.name,
            gene.lineage.generation
        );

    let archived =
        serde_json::to_string_pretty(
            gene
        )
        .unwrap();

    fs::write(
        archive_path,
        archived,
    )
    .unwrap();

    gene.instructions =
        winner.instructions.clone();

    gene.lineage.generation += 1;

    gene.lineage.mutation =
        "winner-promotion"
        .to_string();
}

#[derive(Debug, Clone)]
pub struct EvolutionCandidate {

    pub instructions: String,

    pub generation: u32,

    pub fitness: f32,
}

pub fn generate_candidates(
    gene: &GeneManifest,
    variants: usize,
) -> Vec<EvolutionCandidate> {

    let mut candidates =
        Vec::new();

    for index in 0..variants {

        let instructions =
            format!(
                "{} [variant-{}]",
                gene.instructions,
                index
            );

        let candidate =
            EvolutionCandidate {

                instructions,

                generation:
                    gene.lineage.generation + 1,

                fitness:
                    0.0,
            };

        candidates.push(
            candidate
        );
    }

    candidates
}

pub fn evaluate_candidate(
    candidate: &mut EvolutionCandidate,
) {

    let length_score =
        candidate.instructions.len()
        as f32;

    candidate.fitness =
        length_score / 10.0;
}
pub fn select_winner(
    candidates: &[EvolutionCandidate],
) -> EvolutionCandidate {

    let mut winner =
        candidates[0].clone();

    for candidate in candidates {

        if candidate.fitness >
            winner.fitness {

            winner =
                candidate.clone();
        }
    }

    winner
}
