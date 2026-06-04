use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationEpoch {
    pub epoch_id: String,

    pub dominant_domain: String,

    pub governance_stability: f64,

    pub survivability_alignment: f64,

    pub topology_coherence: f64,

    pub replay_integrity: f64,

    pub ecosystem_expansion: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchaeologyInsight {
    pub epoch_id: String,

    pub civilization_stable: bool,

    pub governance_pressure: bool,

    pub topology_evolution_required: bool,

    pub replay_preservation_priority: String,

    pub strategic_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CivilizationState {
    pub civilization_continuity: f64,

    pub governance_coherence: f64,

    pub replay_civilization_integrity: f64,

    pub sovereign_memory_stable: bool,

    pub insights: Vec<ArchaeologyInsight>,
}

pub struct CivilizationMemoryEngine;

impl CivilizationMemoryEngine {
    pub fn preserve(epochs: &[CivilizationEpoch]) -> CivilizationState {
        let mut insights = Vec::new();

        let mut continuity = 0.0;

        let mut governance = 0.0;

        let mut replay = 0.0;

        for epoch in epochs {
            println!("[CIVILIZATION] epoch={}", epoch.epoch_id);

            let strategic_value = (epoch.governance_stability * 0.25)
                + (epoch.survivability_alignment * 0.25)
                + (epoch.topology_coherence * 0.20)
                + (epoch.replay_integrity * 0.15)
                + (epoch.ecosystem_expansion * 0.15);

            let civilization_stable = strategic_value > 0.84;

            let governance_pressure = epoch.governance_stability < 0.72;

            let topology_evolution_required = epoch.topology_coherence < 0.78;

            let replay_preservation_priority = if epoch.replay_integrity > 0.92 {
                "civilization-critical"
            } else if epoch.replay_integrity > 0.80 {
                "strategic"
            } else {
                "standard"
            };

            insights.push(ArchaeologyInsight {
                epoch_id: epoch.epoch_id.clone(),

                civilization_stable,

                governance_pressure,

                topology_evolution_required,

                replay_preservation_priority: replay_preservation_priority.into(),

                strategic_value,
            });

            continuity += strategic_value;

            governance += epoch.governance_stability;

            replay += epoch.replay_integrity;
        }

        let count = epochs.len() as f64;

        let civilization_continuity = continuity / count;

        let governance_coherence = governance / count;

        let replay_civilization_integrity = replay / count;

        let sovereign_memory_stable = civilization_continuity > 0.84
            && governance_coherence > 0.81
            && replay_civilization_integrity > 0.82;

        CivilizationState {
            civilization_continuity,

            governance_coherence,

            replay_civilization_integrity,

            sovereign_memory_stable,

            insights,
        }
    }
}
