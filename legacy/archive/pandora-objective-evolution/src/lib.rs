//! Pandora Objective Evolution — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategicObjective {
    pub objective_id: String,

    pub priority: f64,

    pub survivability_alignment: f64,

    pub continuity_alignment: f64,

    pub recursion_pressure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveDirective {
    pub objective_id: String,

    pub status: String,

    pub evolve: bool,

    pub oversight_required: bool,

    pub recursion_authorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignObjectiveState {
    pub strategic_stability: f64,

    pub sovereign_alignment: f64,

    pub recursive_ready: bool,

    pub directives: Vec<ObjectiveDirective>,
}

pub struct SovereignObjectiveEvolution;

impl SovereignObjectiveEvolution {
    pub fn evolve(objectives: &[StrategicObjective]) -> SovereignObjectiveState {
        let mut directives = Vec::new();

        let mut stability = 0.0;

        let mut alignment = 0.0;

        for objective in objectives {
            println!("[OBJECTIVE] evaluating {}", objective.objective_id);

            let evolve = objective.priority > 0.75 && objective.survivability_alignment > 0.80;

            let oversight_required = objective.recursion_pressure > 0.78;

            let recursion_authorized = objective.continuity_alignment > 0.82;

            let status = if evolve && recursion_authorized {
                "sovereign-evolution"
            } else if evolve {
                "stable-objective"
            } else {
                "restricted-objective"
            };

            directives.push(ObjectiveDirective {
                objective_id: objective.objective_id.clone(),

                status: status.into(),

                evolve,

                oversight_required,

                recursion_authorized,
            });

            stability += objective.survivability_alignment;

            alignment += objective.continuity_alignment;
        }

        let count = objectives.len() as f64;

        let strategic_stability = stability / count;

        let sovereign_alignment = alignment / count;

        let recursive_ready = strategic_stability > 0.82 && sovereign_alignment > 0.80;

        SovereignObjectiveState {
            strategic_stability,

            sovereign_alignment,

            recursive_ready,

            directives,
        }
    }
}
