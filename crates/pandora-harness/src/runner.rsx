use pandora_model::OllamaClient;
use crate::harness_selector::select_harness;

pub struct HarnessRunner {
    client: OllamaClient,
}

impl HarnessRunner {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: OllamaClient::new(endpoint),
        }
    }

    pub async fn run(&self, model: &str, input: &str) -> Result<String, String> {
        // 1. SELECT HARNESS
        let harness_type = select_harness(input);

        // 2. PLAN PROMPT BASED ON HARNESS
        let planned_prompt = match harness_type {
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

        // 3. CALL MODEL
        let response = self
            .client
            .chat(model, &planned_prompt)
            .await
            .map_err(|e| e.to_string())?;

        // 4. RETURN
        Ok(response.message.content)
    }
}

