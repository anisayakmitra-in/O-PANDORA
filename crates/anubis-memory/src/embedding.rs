use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEmbedding {
    pub embedding_id: String,

    pub memory_id: String,

    pub vector: Vec<f32>,

    pub model: String,
}
