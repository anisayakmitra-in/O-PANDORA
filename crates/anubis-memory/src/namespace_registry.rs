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
pub struct NamespaceRecord {

    pub namespace_id:
        String,

    pub owner:
        String,

    pub memory_count:
        usize,

    pub isolated:
        bool,
}
