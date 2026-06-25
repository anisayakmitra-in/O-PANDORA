use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryChunk {
    pub id: String,

    pub content: String,

    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalResult {
    pub id: String,

    pub score: f32,

    pub content: String,
}

pub struct SemanticMemoryEngine;

impl SemanticMemoryEngine {
    pub fn similarity(a: &[f32], b: &[f32]) -> f32 {
        let mut score = 0.0;

        for (x, y) in a.iter().zip(b.iter()) {
            score += x * y;
        }

        score
    }

    pub fn retrieve(query: &[f32], memory: &[MemoryChunk]) -> Vec<RetrievalResult> {
        let mut results = Vec::new();

        for chunk in memory {
            let similarity = Self::similarity(query, &chunk.embedding);

            println!("[SEMANTIC] {} similarity={}", chunk.id, similarity);

            results.push(RetrievalResult {
                id: chunk.id.clone(),

                score: similarity,

                content: chunk.content.clone(),
            });
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        results
    }
}
