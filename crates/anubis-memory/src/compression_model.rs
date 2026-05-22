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
pub struct CompressionRecord {

    pub compression_id:
        String,

    pub source_memory:
        String,

    pub compressed_summary:
        String,

    pub compression_ratio:
        f32,

    pub retained_salience:
        f32,
}
