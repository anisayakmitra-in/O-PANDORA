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
pub struct MutationRecord {

    pub mutation_id:
        String,

    pub parent_id:
        Option<String>,

    pub actor:
        String,

    pub timestamp:
        u64,

    pub reason:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct MemoryLineage {

    pub memory_id:
        String,

    pub mutations:
        Vec<MutationRecord>,
}

pub struct LineageEngine;

impl LineageEngine {

    pub fn append_mutation(

        lineage:
            &mut MemoryLineage,

        mutation:
            MutationRecord,

    ) {

        lineage
            .mutations
            .push(
                mutation
            );
    }

    pub fn latest_mutation(

        lineage:
            &MemoryLineage,

    ) -> Option<
        &MutationRecord
    > {

        lineage
            .mutations
            .last()
    }
}
