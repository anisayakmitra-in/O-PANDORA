use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageRecord {
    pub parent_id: String,

    pub child_id: String,

    pub generation: u32,

    pub mutation: String,
}

pub struct SwarmLineage;

impl SwarmLineage {
    pub fn trace(records: &[LineageRecord]) {
        for record in records {
            println!(
                "[LINEAGE] {} -> {} generation={} mutation={}",
                record.parent_id, record.child_id, record.generation, record.mutation
            );
        }
    }
}
