use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationMetanoeticsNode {
    pub civilization_id: String,

    pub cognitive_metamorphosis_stability: f64,

    pub recursive_awareness_integrity: f64,

    pub consciousness_transition_coherence: f64,

    pub reflective_depth_expansion: f64,

    pub post_consciousness_alignment: f64,

    pub metanoetic_fragmentation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetanoeticDirective {
    pub civilization_id: String,

    pub self_transformation_verified: bool,

    pub recursive_awareness_stable: bool,

    pub consciousness_transition_preserved: bool,

    pub metanoetic_rehabilitation_required: bool,

    pub consciousness_collapse_detected: bool,

    pub metanoetic_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationMetanoeticsState {
    pub constitutional_self_transformation_integrity: f64,

    pub recursive_awareness_stability: f64,

    pub civilization_consciousness_coherence: f64,

    pub sovereign_metanoetics_stable: bool,

    pub directives: Vec<MetanoeticDirective>,
}

pub struct ConstitutionalCivilizationMetanoeticsEngine;

impl ConstitutionalCivilizationMetanoeticsEngine {
    pub fn transform(
        civilizations: &[CivilizationMetanoeticsNode],
    ) -> CivilizationMetanoeticsState {
        let mut directives = Vec::new();

        let mut transformation = 0.0;

        let mut awareness = 0.0;

        let mut coherence = 0.0;

        for civilization in civilizations {
            println!(
                "[METANOETICS] civilization={}",
                civilization.civilization_id
            );

            let metanoetic_score = (civilization.cognitive_metamorphosis_stability * 0.20)
                + (civilization.recursive_awareness_integrity * 0.20)
                + (civilization.consciousness_transition_coherence * 0.20)
                + (civilization.reflective_depth_expansion * 0.15)
                + (civilization.post_consciousness_alignment * 0.15)
                + ((1.0 - civilization.metanoetic_fragmentation) * 0.10);

            let self_transformation_verified = metanoetic_score > 0.86;

            let recursive_awareness_stable = civilization.recursive_awareness_integrity > 0.84;

            let consciousness_transition_preserved =
                civilization.consciousness_transition_coherence > 0.84;

            let metanoetic_rehabilitation_required = metanoetic_score < 0.74;

            let consciousness_collapse_detected = civilization.metanoetic_fragmentation > 0.82;

            directives.push(MetanoeticDirective {
                civilization_id: civilization.civilization_id.clone(),

                self_transformation_verified,

                recursive_awareness_stable,

                consciousness_transition_preserved,

                metanoetic_rehabilitation_required,

                consciousness_collapse_detected,

                metanoetic_score,
            });

            transformation += metanoetic_score;

            awareness += civilization.recursive_awareness_integrity;

            coherence += civilization.consciousness_transition_coherence;
        }

        let count = civilizations.len() as f64;

        let constitutional_self_transformation_integrity = transformation / count;

        let recursive_awareness_stability = awareness / count;

        let civilization_consciousness_coherence = coherence / count;

        let sovereign_metanoetics_stable = constitutional_self_transformation_integrity > 0.84
            && recursive_awareness_stability > 0.82
            && civilization_consciousness_coherence > 0.84;

        CivilizationMetanoeticsState {
            constitutional_self_transformation_integrity,

            recursive_awareness_stability,

            civilization_consciousness_coherence,

            sovereign_metanoetics_stable,

            directives,
        }
    }
}
