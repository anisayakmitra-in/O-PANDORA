use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTask {
    pub task_id: String,

    pub category: String,

    pub difficulty: f32,

    pub expected_output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub task_id: String,

    pub candidate_id: String,

    pub success: bool,

    pub score: f32,
}

pub struct BenchmarkHarness;

impl BenchmarkHarness {
    pub fn evaluate(candidate_id: &str, task: &BenchmarkTask) -> BenchmarkResult {
        let score = 1.0 - (task.difficulty * 0.2);

        BenchmarkResult {
            task_id: task.task_id.clone(),

            candidate_id: candidate_id.into(),

            success: score > 0.5,

            score,
        }
    }
}
