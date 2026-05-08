use crate::{
    embeddings::cosine_similarity,
    storage::MemoryRecord,
};

pub fn retrieve_context(
    query_embedding:
        &[f32],

    memories:
        &[MemoryRecord],
) -> Vec<(f32, MemoryRecord)> {

    let mut scored =
        Vec::new();

    for memory
    in memories {

        let similarity =
            cosine_similarity(
                query_embedding,
                &memory.embedding,
            );

        let weight =
            similarity
            + memory.salience;

        scored.push(
            (
                weight,
                memory.clone()
            )
        );
    }

    scored.sort_by(
        |a, b| {
            b.0.partial_cmp(&a.0)
                .unwrap()
        }
    );

    scored
}

pub fn search_memories(
    memories:
        &[MemoryRecord],

    query:
        &str,
) -> Vec<(f32, MemoryRecord)> {

    let query_embedding =
        crate::generate_embedding(
            query
        );

    retrieve_context(
        &query_embedding,
        memories,
    )
}

pub fn restore_context(
    memories: &[MemoryRecord],
    limit: usize,
) -> Vec<MemoryRecord> {

    let mut sorted =
        memories.to_vec();

    sorted.sort_by(
        |a, b| {

            b.salience
                .partial_cmp(
                    &a.salience
                )
                .unwrap()
        }
    );

    sorted
        .into_iter()
        .take(limit)
        .collect()
}
