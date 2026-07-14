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
pub struct CognitionMeshNode {

    pub node_id:
        String,

    pub swarm:
        String,

    pub cognition_integrity:
        f64,

    pub propagation_stability:
        f64,

    pub oversight_visibility:
        f64,

    pub continuity_sync:
        f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct MeshPropagationDirective {

    pub node_id:
        String,

    pub propagate:
        bool,

    pub oversight_required:
        bool,

    pub continuity_verified:
        bool,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CognitionMeshState {

    pub mesh_stability:
        f64,

    pub recursive_safe:
        bool,

    pub sovereign_mesh_ready:
        bool,

    pub directives:
        Vec<
            MeshPropagationDirective
        >,
}

pub struct RecursiveCognitionMesh;

impl RecursiveCognitionMesh {

    pub fn propagate(

        nodes:
            &[CognitionMeshNode],
    )
        -> CognitionMeshState
    {

        let mut directives =
            Vec::new();

        let mut integrity =
            0.0;

        let mut stability =
            0.0;

        let mut continuity =
            0.0;

        for node
            in nodes
        {

            println!(
                "[MESH] node={}",
                node.node_id
            );

            let propagate =
                node.cognition_integrity
                    > 0.75
                &&
                node.propagation_stability
                    > 0.70;

            let oversight_required =
                node.oversight_visibility
                    < 0.70;

            let continuity_verified =
                node.continuity_sync
                    > 0.80;

            directives.push(

                MeshPropagationDirective {

                    node_id:
                        node
                            .node_id
                            .clone(),

                    propagate,

                    oversight_required,

                    continuity_verified,
                }
            );

            integrity +=
                node.cognition_integrity;

            stability +=
                node.propagation_stability;

            continuity +=
                node.continuity_sync;
        }

        let count =
            nodes.len() as f64;

        let mesh_stability =
            (
                integrity / count
            )
            * 0.40
            + (
                stability / count
            )
            * 0.35
            + (
                continuity / count
            )
            * 0.25;

        let recursive_safe =
            mesh_stability
                > 0.78;

        let sovereign_mesh_ready =
            mesh_stability
                > 0.88;

        CognitionMeshState {

            mesh_stability,

            recursive_safe,

            sovereign_mesh_ready,

            directives,
        }
    }
}
