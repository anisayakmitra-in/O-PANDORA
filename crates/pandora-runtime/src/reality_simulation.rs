use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureScenario {
    pub scenario_id: String,

    pub domain: String,

    pub governance_pressure: f64,

    pub topology_pressure: f64,

    pub ecosystem_instability: f64,

    pub survivability_projection: f64,

    pub replay_continuity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityBranch {
    pub scenario_id: String,

    pub civilization_survivable: bool,

    pub governance_collapse_risk: bool,

    pub topology_mutation_required: bool,

    pub ecosystem_expansion_safe: bool,

    pub future_branch_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationState {
    pub civilization_future_integrity: f64,

    pub survivability_forecast: f64,

    pub governance_future_stability: f64,

    pub sovereign_future_viable: bool,

    pub branches: Vec<RealityBranch>,
}

pub struct ConstitutionalRealitySimulationEngine;

impl ConstitutionalRealitySimulationEngine {
    pub fn simulate(scenarios: &[FutureScenario]) -> SimulationState {
        let mut branches = Vec::new();

        let mut integrity = 0.0;

        let mut survivability = 0.0;

        let mut governance = 0.0;

        for scenario in scenarios {
            println!("[SIMULATION] scenario={}", scenario.scenario_id);

            let future_branch_score = ((1.0 - scenario.governance_pressure) * 0.25)
                + ((1.0 - scenario.topology_pressure) * 0.20)
                + ((1.0 - scenario.ecosystem_instability) * 0.20)
                + (scenario.survivability_projection * 0.20)
                + (scenario.replay_continuity * 0.15);

            let civilization_survivable = future_branch_score > 0.82;

            let governance_collapse_risk = scenario.governance_pressure > 0.82;

            let topology_mutation_required = scenario.topology_pressure > 0.70;

            let ecosystem_expansion_safe = scenario.ecosystem_instability < 0.40;

            branches.push(RealityBranch {
                scenario_id: scenario.scenario_id.clone(),

                civilization_survivable,

                governance_collapse_risk,

                topology_mutation_required,

                ecosystem_expansion_safe,

                future_branch_score,
            });

            integrity += future_branch_score;

            survivability += scenario.survivability_projection;

            governance += 1.0 - scenario.governance_pressure;
        }

        let count = scenarios.len() as f64;

        let civilization_future_integrity = integrity / count;

        let survivability_forecast = survivability / count;

        let governance_future_stability = governance / count;

        let sovereign_future_viable = civilization_future_integrity > 0.83
            && survivability_forecast > 0.84
            && governance_future_stability > 0.80;

        SimulationState {
            civilization_future_integrity,

            survivability_forecast,

            governance_future_stability,

            sovereign_future_viable,

            branches,
        }
    }
}
