use crate::storage::MemoryRecord;

pub fn generate_relationships(
    memory:
        &MemoryRecord,

    memories:
        &[MemoryRecord],
) -> Vec<String> {

    let mut related =
        Vec::new();

    for other
    in memories {

        if other.id
            == memory.id
        {
            continue;
        }

        if other.gene
            == memory.gene
        {
            related.push(
                other.id.clone()
            );
        }

        if other.harness
            == memory.harness
        {
            related.push(
                other.id.clone()
            );
        }

        if other.prompt
            .contains(
                &memory.prompt
            )
        {
            related.push(
                other.id.clone()
            );
        }
    }

    related.sort();

    related.dedup();

    related
}

use std::collections::HashMap;

pub fn cluster_memories(
    memories: &[MemoryRecord]
) -> HashMap<String, Vec<String>> {

    let mut clusters =
        HashMap::new();

    for memory in memories {

        for tag in &memory.tags {

            clusters
                .entry(tag.clone())
                .or_insert(vec![])
                .push(memory.id.clone());
        }
    }

    clusters
}

pub fn related_by_tag(
    memories: &[MemoryRecord],
    tag: &str,
) -> Vec<MemoryRecord> {

    memories
        .iter()
        .filter(
            |m| {

                m.tags.contains(
                    &tag.to_string()
                )
            }
        )
        .cloned()
        .collect()
}
