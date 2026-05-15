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
pub enum CognitionCategory {

    Reasoning,

    Planning,

    Mutation,

    Deliberation,

    Telemetry,

    Execution,

    Governance,

    Memory,

    Lineage,

    Semantic,

    Capability,
}
