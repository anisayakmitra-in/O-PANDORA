use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationOperator {
    pub operator_id: String,

    pub mutation_type: String,

    pub intensity: f32,
}

impl MutationOperator {
    pub fn apply(&self, source: &str) -> String {
        match self.mutation_type.as_str() {
            "prompt.expand" => {
                format!("{} :: expanded reasoning", source)
            }

            "planner.recursive" => {
                format!("{} :: recursive planning", source)
            }

            "retrieval.optimize" => {
                format!("{} :: optimized retrieval", source)
            }

            _ => source.to_string(),
        }
    }
}
