use serde::{Deserialize, Serialize};

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResult {
    pub text: String,

    pub embedding: Vec<f32>,
}

pub struct EmbeddingEngine;

impl EmbeddingEngine {
    pub fn generate(text: &str) -> EmbeddingResult {
        println!("[EMBEDDING] generating embedding");

        let mut embedding = Vec::new();

        for token in text.split_whitespace() {
            let mut hasher = DefaultHasher::new();

            token.hash(&mut hasher);

            let hash = hasher.finish();

            let normalized = (hash % 1000) as f32 / 1000.0;

            embedding.push(normalized);
        }

        while embedding.len() < 8 {
            embedding.push(0.0);
        }

        embedding.truncate(8);

        EmbeddingResult {
            text: text.into(),

            embedding,
        }
    }
}
