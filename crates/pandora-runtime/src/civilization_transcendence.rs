use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CivilizationTranscendenceNode {

    pub civilization_id:
        String,

    pub existential_stability:
        f64,

    pub constitutional_maturity:
        f64,

    pub recursive_introspection:
        f64,

    pub survivability_mastery:
        f64,

    pub governance_entropy_reduction:
        f64,

    pub transcendence_instability:
        f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct TranscendenceDirective {

    pub civilization_id:
        String,

    pub transcendence_authorized:
        bool,

    pub higher_order_transition_allowed:
        bool,

    pub constitutional_form_obsolete:
        bool,

    pub metamorphosis_stabilization_required:
        bool,

    pub transcendence_collapse_detected:
        bool,

    pub transcendence_score:
        f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CivilizationTranscendenceState {

    pub transcendence_integrity:
        f64,

    pub higher_order_stability:
        f64,

    pub civilization_maturation_coherence:
        f64,

    pub sovereign_transcendence_stable:
        bool,

    pub directives:
        Vec<
            TranscendenceDirective
        >,
}

pub struct ConstitutionalCivilizationTranscendenceEngine;

impl ConstitutionalCivilizationTranscendenceEngine {

    pub fn transcend(

        civilizations:
            &[CivilizationTranscendenceNode],
    )
        -> CivilizationTranscendenceState
    {

        let mut directives =
            Vec::new();

        let mut transcendence =
            0.0;

        let mut stability =
            0.0;

        let mut maturation =
            0.0;

        for civilization
            in civilizations
        {

            println!(
                "[TRANSCENDENCE] civilization={}",
                civilization.civilization_id
            );

            let transcendence_score =
                (
                    civilization
                        .existential_stability
                        * 0.20
                )
                + (
                    civilization
                        .constitutional_maturity
                        * 0.20
                )
                + (
                    civilization
                        .recursive_introspection
                        * 0.20
                )
                + (
                    civilization
                        .survivability_mastery
                        * 0.15
                )
                + (
                    civilization
                        .governance_entropy_reduction
                        * 0.15
                )
                + (
                    (1.0
                        - civilization
                            .transcendence_instability)
                        * 0.10
                );

            let transcendence_authorized =
                transcendence_score
                    > 0.90;

            let higher_order_transition_allowed =
                civilization
                    .constitutional_maturity
                        > 0.92;

            let constitutional_form_obsolete =
                civilization
                    .governance_entropy_reduction
                        > 0.94;

            let metamorphosis_stabilization_required =
                transcendence_score
                    < 0.78;

            let transcendence_collapse_detected =
                civilization
                    .transcendence_instability
                        > 0.82;

            directives.push(

                TranscendenceDirective {

                    civilization_id:
                        civilization
                            .civilization_id
                            .clone(),

                    transcendence_authorized,

                    higher_order_transition_allowed,

                    constitutional_form_obsolete,

                    metamorphosis_stabilization_required,

                    transcendence_collapse_detected,

                    transcendence_score,
                }
            );

            transcendence +=
                transcendence_score;

            stability +=
                civilization
                    .existential_stability;

            maturation +=
                civilization
                    .constitutional_maturity;
        }

        let count =
            civilizations.len() as f64;

        let transcendence_integrity =
            transcendence / count;

        let higher_order_stability =
            stability / count;

        let civilization_maturation_coherence =
            maturation / count;

        let sovereign_transcendence_stable =
            transcendence_integrity
                > 0.86
            &&
            higher_order_stability
                > 0.84
            &&
            civilization_maturation_coherence
                > 0.85;

        CivilizationTranscendenceState {

            transcendence_integrity,

            higher_order_stability,

            civilization_maturation_coherence,

            sovereign_transcendence_stable,

            directives,
        }
    }
}
