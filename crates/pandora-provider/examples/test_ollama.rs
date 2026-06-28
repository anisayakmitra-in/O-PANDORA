use pandora_provider::legacy::ollama::OllamaProvider;
use pandora_provider::traits::Provider;
use pandora_provider::types::GenerationRequest;

use tokio_util::sync::CancellationToken;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let provider = OllamaProvider::new();

    let request = GenerationRequest {
        prompt: String::from("Explain what Pandora Systems is in one sentence."),
        model: String::from("qwen2.5-coder:7b"),
        temperature: 0.7,
        max_tokens: 128,
    };

    let result = provider.generate(request, CancellationToken::new()).await;

    println!("\nRESULT:\n{:#?}", result);
}
