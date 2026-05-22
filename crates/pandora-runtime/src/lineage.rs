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
pub struct CognitionLineage {

    pub lineage_id:
        String,

    pub parent_lineage:
        Option<String>,

    pub originating_gene:
        String,

    pub mutation_reason:
        String,

    pub associated_graph:
        String,

    pub associated_event:
        String,

    pub timestamp:
        String,
}
