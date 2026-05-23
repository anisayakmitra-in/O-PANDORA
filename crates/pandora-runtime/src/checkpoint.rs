use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionCheckpoint {
    pub checkpoint_id: String,

    pub execution_graph: String,

    pub entropy: f32,

    pub stable: bool,
}
