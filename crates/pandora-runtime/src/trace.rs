use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct RuntimeTrace {

    pub session_id: String,

    pub gene: String,

    pub provider: String,

    pub prompt: String,

    pub approved_tools: Vec<String>,

    pub denied_tools: Vec<String>,

    pub memory_hits: usize,

    pub success: bool,
}
