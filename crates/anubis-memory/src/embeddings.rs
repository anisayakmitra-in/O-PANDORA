pub fn generate_embedding(
    text: &str,
) -> Vec<f32> {

    text.bytes()

        .map(
            |b| {
                b as f32 / 255.0
            }
        )

        .take(128)

        .collect()
}

pub fn cosine_similarity(
    a: &[f32],
    b: &[f32],
) -> f32 {

    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (x, y)
    in a.iter().zip(b.iter()) {

        dot += x * y;

        norm_a += x * x;

        norm_b += y * y;
    }

    dot / (
        norm_a.sqrt()
        * norm_b.sqrt()
        + 0.0001
    )
}
