use crate::memory_index::MemoryIndex;

use crate::retrieval::{
    RetrievalQuery,
    RetrievalResult,
};

pub fn retrieve_memories(

    index:
        &MemoryIndex,

    query:
        &RetrievalQuery,

)
    -> Vec<RetrievalResult>
{

    let mut results =
        Vec::new();

    for entry
        in &index.entries
    {

        let mut score =
            0.0;

        if entry.content
            .to_lowercase()
            .contains(
                &query
                    .semantic_query
                    .to_lowercase()
            )
        {

            score += 0.7;
        }

        for tag
            in &query.tags
        {

            if entry.tags
                .contains(tag)
            {

                score += 0.15;
            }
        }

        if let Some(namespace)
            = &query.namespace
        {

            if &entry.namespace
                == namespace
            {

                score += 0.15;
            }
        }

        if score > 0.0 {

            results.push(
                RetrievalResult {

                    memory_id:
                        entry.memory_id.clone(),

                    score,

                    matched_content:
                        entry.content.clone(),
                }
            );
        }
    }

    results.sort_by(
        |a, b| {

            b.score
                .partial_cmp(
                    &a.score
                )
                .unwrap()
        }
    );

    results.truncate(
        query.limit
    );

    results
}
