use crate::gene::GeneManifest;

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

