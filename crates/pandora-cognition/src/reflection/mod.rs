use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct ReflectionResult {

    pub summary:
        String,

    pub strengths:
        Vec<String>,

    pub weaknesses:
        Vec<String>,

    pub improvements:
        Vec<String>,
}

pub mod engine;

pub mod basic;

pub mod llm;
