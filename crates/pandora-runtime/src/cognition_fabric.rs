use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricNode {
    pub node_id: String,

    pub harness: String,

    pub substrate: String,

    pub governance_score: f64,

    pub survivability: f64,

    pub replay_support: bool,

    pub distributed_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricTopology {
    pub topology_id: String,

    pub nodes: Vec<String>,

    pub heterogeneous: bool,

    pub replayable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricDirective {
    pub topology_id: String,

    pub orchestration_mode: String,

    pub governance_stable: bool,

    pub survivable: bool,

    pub replay_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionFabricState {
    pub fabric_integrity: f64,

    pub constitutional_stability: f64,

    pub replay_confidence: f64,

    pub heterogeneous_ready: bool,

    pub directives: Vec<FabricDirective>,
}

pub struct CognitionFabricOrchestrator;

impl CognitionFabricOrchestrator {
    pub fn orchestrate(
        topologies: &[FabricTopology],

        nodes: &[FabricNode],
    ) -> CognitionFabricState {
        let mut directives = Vec::new();

        let mut integrity = 0.0;

        let mut governance = 0.0;

        let mut replay = 0.0;

        for topology in topologies {
            println!("[FABRIC] topology={}", topology.topology_id);

            let matched_nodes = nodes
                .iter()
                .filter(|node| topology.nodes.contains(&node.node_id))
                .collect::<Vec<_>>();

            let avg_governance = matched_nodes
                .iter()
                .map(|node| node.governance_score)
                .sum::<f64>()
                / matched_nodes.len() as f64;

            let avg_survivability = matched_nodes
                .iter()
                .map(|node| node.survivability)
                .sum::<f64>()
                / matched_nodes.len() as f64;

            let replay_verified = matched_nodes.iter().all(|node| node.replay_support);

            let orchestration_mode = if topology.heterogeneous {
                "heterogeneous-fabric"
            } else {
                "stable-fabric"
            };

            let governance_stable = avg_governance > 0.82;

            let survivable = avg_survivability > 0.84;

            directives.push(FabricDirective {
                topology_id: topology.topology_id.clone(),

                orchestration_mode: orchestration_mode.into(),

                governance_stable,

                survivable,

                replay_verified,
            });

            integrity += avg_survivability;

            governance += avg_governance;

            replay += if replay_verified { 1.0 } else { 0.0 };
        }

        let count = topologies.len() as f64;

        let fabric_integrity = integrity / count;

        let constitutional_stability = governance / count;

        let replay_confidence = replay / count;

        let heterogeneous_ready = directives
            .iter()
            .any(|directive| directive.orchestration_mode == "heterogeneous-fabric");

        CognitionFabricState {
            fabric_integrity,

            constitutional_stability,

            replay_confidence,

            heterogeneous_ready,

            directives,
        }
    }
}
