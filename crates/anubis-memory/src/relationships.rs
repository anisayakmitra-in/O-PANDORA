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
