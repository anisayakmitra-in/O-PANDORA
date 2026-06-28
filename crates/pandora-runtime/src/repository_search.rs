use serde::{Deserialize, Serialize};

use crate::embedding_engine::EmbeddingEngine;

use crate::semantic_memory::SemanticMemoryEngine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryDocument {
    pub id: String,

    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,

    pub score: f32,

    pub content: String,
}

pub struct RepositorySearchEngine;

impl RepositorySearchEngine {
    pub fn search(query: &str, documents: &[RepositoryDocument]) -> Vec<SearchResult> {
        println!("[SEARCH] semantic query={}", query);

        let query_embedding = EmbeddingEngine::generate(query);

        let mut results = Vec::new();

        for document in documents {
            let embedding = EmbeddingEngine::generate(&document.content);

            let similarity =
                SemanticMemoryEngine::similarity(&query_embedding.embedding, &embedding.embedding);

            println!("[SEARCH] {} similarity={}", document.id, similarity);

            results.push(SearchResult {
                id: document.id.clone(),

                score: similarity,

                content: document.content.clone(),
            });
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        results
    }
}

// Compatibility aliases for old import names used in main.rs.
pub type Result = SearchResult;
pub type Search = SearchResult;
