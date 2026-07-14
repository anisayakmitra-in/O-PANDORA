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
pub struct MutationProposal {

    pub target:
        String,

    pub current_behavior:
        String,

    pub proposed_behavior:
        String,

    pub reasoning:
        String,

    pub confidence:
        f32,
}

pub mod engine;

pub mod reflective;

pub mod pareto;

pub mod loop;

pub mod llm;

pub mod validation;

pub mod testing;

pub mod promotion;

pub mod lineage;

pub mod branch;


