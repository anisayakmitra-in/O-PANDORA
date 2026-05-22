use crate::embedding::MemoryEmbedding;

pub fn cosine_similarity(

    a:
        &[f32],

    b:
        &[f32],
)
    -> f32
{

    let dot_product:
        f32 =
            a.iter()
                .zip(b.iter())
                .map(
                    |(x, y)| {

                        x * y
                    }
                )
                .sum();

    let magnitude_a:
        f32 =
            a.iter()
                .map(
                    |x| {

                        x * x
                    }
                )
                .sum::<f32>()
                .sqrt();

    let magnitude_b:
        f32 =
            b.iter()
                .map(
                    |x| {

                        x * x
                    }
                )
                .sum::<f32>()
                .sqrt();

    dot_product
        /
        (
            magnitude_a
            *
            magnitude_b
        )
}

pub fn nearest_embedding(

    query:
        &[f32],

    embeddings:
        &[MemoryEmbedding],
)
    -> Option<MemoryEmbedding>
{

    let mut best_score =
        -1.0;

    let mut best_embedding =
        None;

    for embedding
        in embeddings
    {

        let score =
            cosine_similarity(
                query,
                &embedding.vector,
            );

        if score > best_score {

            best_score =
                score;

            best_embedding =
                Some(
                    embedding.clone()
                );
        }
    }

    best_embedding
}
