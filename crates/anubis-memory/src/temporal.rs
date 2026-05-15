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
pub struct TemporalMetadata {

    pub timestamp:
        u64,

    pub sequence:
        u64,
}
