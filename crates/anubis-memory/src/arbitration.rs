use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct ArbitrationScore {

    pub memory_id:
        String,

    pub semantic_score:
        f32,

    pub temporal_score:
        f32,

    pub graph_score:
        f32,

    pub final_score:
        f32,
}
	
