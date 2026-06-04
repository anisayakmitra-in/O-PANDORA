use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategicSignal {
    pub signal_id: String,

    pub domain: String,

    pub survivability_pressure: f64,

    pub ecosystem_pressure: f64,

    pub infrastructure_value: f64,

    pub governance_alignment: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategicDirective {
    pub directive_id: String,

    pub target_domain: String,

    pub priority: String,

    pub expansion_required: bool,

    pub governance_priority: bool,

    pub topology_evolution: bool,

    pub survivability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategicState {
    pub sovereign_alignment: f64,

    pub infrastructure_coherence: f64,

    pub survivability_continuity: f64,

    pub strategic_stability: bool,

    pub directives: Vec<StrategicDirective>,
}

pub struct SovereignStrategicDirectiveEngine;

impl SovereignStrategicDirectiveEngine {
    pub fn synthesize(signals: &[StrategicSignal]) -> StrategicState {
        let mut directives = Vec::new();

        let mut alignment = 0.0;

        let mut coherence = 0.0;

        let mut survivability = 0.0;

        for signal in signals {
            println!("[STRATEGIC] signal={}", signal.signal_id);

            let survivability_score = (signal.survivability_pressure * 0.35)
                + (signal.ecosystem_pressure * 0.20)
                + (signal.infrastructure_value * 0.30)
                + (signal.governance_alignment * 0.15);

            let priority = if survivability_score > 0.90 {
                "apex"
            } else if survivability_score > 0.78 {
                "high"
            } else {
                "restricted"
            };

            let expansion_required = signal.ecosystem_pressure > 0.75;

            let governance_priority = signal.governance_alignment < 0.70;

            let topology_evolution = signal.infrastructure_value > 0.82;

            directives.push(StrategicDirective {
                directive_id: format!("directive-{}", signal.signal_id),

                target_domain: signal.domain.clone(),

                priority: priority.into(),

                expansion_required,

                governance_priority,

                topology_evolution,

                survivability_score,
            });

            alignment += signal.governance_alignment;

            coherence += signal.infrastructure_value;

            survivability += survivability_score;
        }

        let count = signals.len() as f64;

        let sovereign_alignment = alignment / count;

        let infrastructure_coherence = coherence / count;

        let survivability_continuity = survivability / count;

        let strategic_stability = sovereign_alignment > 0.80
            && infrastructure_coherence > 0.82
            && survivability_continuity > 0.84;

        StrategicState {
            sovereign_alignment,

            infrastructure_coherence,

            survivability_continuity,

            strategic_stability,

            directives,
        }
    }
}
