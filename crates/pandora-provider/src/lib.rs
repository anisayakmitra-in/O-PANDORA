pub mod registry;

pub mod ollama;

pub mod provider;

pub mod types;

pub trait Provider {

    fn name(&self)
    -> &str;

    fn infer(
        &self,
        model: &str,
        prompt: &str,
    ) -> String;
}

pub struct OllamaProvider;

impl Provider
for OllamaProvider {

    fn name(&self)
    -> &str {

        "ollama"
    }

    fn infer(
        &self,
        model: &str,
        prompt: &str,
    ) -> String {

        format!(
            "\
OLLAMA PROVIDER

MODEL: {}

PROMPT:
{}

RESPONSE:
Simulated Ollama inference response.",
            model,
            prompt
        )
    }
}

pub struct OpenAIProvider;

impl Provider
for OpenAIProvider {

    fn name(&self)
    -> &str {

        "openai-compatible"
    }

    fn infer(
        &self,
        model: &str,
        prompt: &str,
    ) -> String {

        format!(
            "\
OPENAI-COMPATIBLE PROVIDER

MODEL: {}

PROMPT:
{}

RESPONSE:
Simulated cloud inference response.",
            model,
            prompt
        )
    }
}

pub fn model_for_harness(
    harness: &str,
) -> String {

    match harness {

        "coding" => {
            "qwen2.5-coder:7b"
                .to_string()
        }

        "research" => {
            "mistral"
                .to_string()
        }

        "writing" => {
            "llama3"
                .to_string()
        }

        _ => {
            "qwen2.5-coder:7b"
                .to_string()
        }
    }
}
