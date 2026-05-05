use pandora_model::OllamaClient;
use pandora_types::HarnessSpec;
use pandora_memory::HarnessPerformance;

use crate::harness_selector::select_harness;
use crate::feedback::score_response;

pub struct HarnessRunner {
    client: OllamaClient,
    performance: HarnessPerformance,
}

impl HarnessRunner {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: OllamaClient::new(endpoint),
            performance: HarnessPerformance::new(),
        }
    }

    pub async fn run_with_specs(
        &mut self,
        model: &str,
        input: &str,
        specs: &[HarnessSpec],
    ) -> Result<String, String> {

        let llm_choice = select_harness(&self.client, model, input, specs).await;

// performance-based override
let perf_choice = crate::harness_selector::select_best_by_performance(
    specs,
    &self.performance,
);

// simple arbitration
let harness_type = if self.performance.count(&llm_choice) > 3 {
    llm_choice
} else {
    perf_choice
};

        let planned_prompt = match harness_type.as_str() {
            "coding" => format!(
                "You are a senior Rust engineer.\nTask: {}\nAnswer precisely.",
                input
            ),
            "business" => format!(
                "You are a business strategist.\nTask: {}\nGive structured insight.",
                input
            ),
            _ => format!(
                "You are a helpful assistant.\nTask: {}\nAnswer clearly.",
                input
            ),
        };

        let response = self
            .client
            .chat(model, &planned_prompt)
            .await
            .map_err(|e| e.to_string())?;

        let output = response.message.content.clone();

        // scoring
        let score = score_response(&output);

        // record performance
        self.performance.record(&harness_type, score);

        // debug output
        println!("HARNESS: {}", harness_type);
        println!("SCORE: {}", score);
        println!("AVG: {}", self.performance.average(&harness_type));

        Ok(output)
    }
}
