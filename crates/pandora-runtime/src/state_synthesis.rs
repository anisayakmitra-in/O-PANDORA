use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignSubsystemState {
    pub subsystem: String,

    pub operational_score: f64,

    pub survivability: f64,

    pub anomaly_risk: f64,

    pub continuity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizedRuntimeState {
    pub global_state: String,

    pub sovereign_stability: f64,

    pub recursion_safe: bool,

    pub distributed_ready: bool,

    pub operational_confidence: f64,
}

pub struct ExecutionStateSynthesisEngine;

impl ExecutionStateSynthesisEngine {
    pub fn synthesize(states: &[SovereignSubsystemState]) -> SynthesizedRuntimeState {
        let mut operational = 0.0;

        let mut survivability = 0.0;

        let mut continuity = 0.0;

        let mut anomaly = 0.0;

        for state in states {
            println!("[SYNTHESIS] subsystem={}", state.subsystem);

            operational += state.operational_score;

            survivability += state.survivability;

            continuity += state.continuity;

            anomaly += state.anomaly_risk;
        }

        let count = states.len() as f64;

        let operational_avg = operational / count;

        let survivability_avg = survivability / count;

        let continuity_avg = continuity / count;

        let anomaly_avg = anomaly / count;

        let sovereign_stability =
            (operational_avg * 0.35) + (survivability_avg * 0.30) + (continuity_avg * 0.25)
                - (anomaly_avg * 0.10);

        let recursion_safe = anomaly_avg < 0.65;

        let distributed_ready = survivability_avg > 0.75;

        let global_state = if sovereign_stability > 0.90 {
            "sovereign-stable"
        } else if sovereign_stability > 0.75 {
            "operationally-stable"
        } else {
            "degraded-cognition"
        };

        SynthesizedRuntimeState {
            global_state: global_state.into(),

            sovereign_stability,

            recursion_safe,

            distributed_ready,

            operational_confidence: operational_avg,
        }
    }
}
