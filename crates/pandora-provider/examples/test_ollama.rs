use pandora_provider::ollama::OllamaProvider;

use pandora_provider::provider::Provider;

use pandora_provider::types::GenerationRequest;

use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let provider = OllamaProvider::new();

    let request = GenerationRequest {
        prompt: String::from("Explain what Pandora Systems is in one sentence."),

        temperature: 0.7,

        max_tokens: 128,
    };

    let result = provider.generate(request, CancellationToken::new()).await;

    println!("\nRESULT:\n{:#?}", result);
}
