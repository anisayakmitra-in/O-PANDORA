use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyRequirement {
    pub domain: String,

    pub recursion_pressure: f64,

    pub distributed_pressure: f64,

    pub survivability_requirement: f64,

    pub heterogeneous_requirement: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyNode {
    pub node_id: String,

    pub harness: String,

    pub substrate: String,

    pub governance_score: f64,

    pub telemetry_visibility: f64,

    pub replay_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizedTopology {
    pub topology_id: String,

    pub execution_graph: Vec<String>,

    pub distributed: bool,

    pub heterogeneous: bool,

    pub replayable: bool,

    pub governance_stable: bool,
}

pub struct ExecutionTopologySynthesizer;

impl ExecutionTopologySynthesizer {
    pub fn synthesize(
        requirement: &TopologyRequirement,

        nodes: &[TopologyNode],
    ) -> SynthesizedTopology {
        println!("[TOPOLOGY] domain={}", requirement.domain);

        let mut graph = Vec::new();

        let mut governance = 0.0;

        let mut replay = true;

        for node in nodes {
            if node.governance_score < 0.75 {
                continue;
            }

            println!("[TOPOLOGY] node={}", node.node_id);

            graph.push(format!(
                "{}::{}::{}",
                node.node_id, node.harness, node.substrate
            ));

            governance += node.governance_score;

            replay = replay && node.replay_support;
        }

        let governance_avg = governance / nodes.len() as f64;

        let distributed = requirement.distributed_pressure > 0.65;

        let heterogeneous = requirement.heterogeneous_requirement;

        let governance_stable = governance_avg > 0.82;

        SynthesizedTopology {
            topology_id: format!("{}-topology", requirement.domain),

            execution_graph: graph,

            distributed,

            heterogeneous,

            replayable: replay,

            governance_stable,
        }
    }
}
