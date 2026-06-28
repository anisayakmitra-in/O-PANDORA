use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use pandora_provider::legacy::ollama::OllamaProvider;
use pandora_provider::traits::Provider;
use pandora_provider::types::{GenerationRequest, TokenChunk};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let provider = OllamaProvider::new();

    let request = GenerationRequest {
        prompt: String::from("Explain Pandora Systems in three sentences."),
        model: String::from("qwen2.5-coder:7b"),
        temperature: 0.7,
        max_tokens: 128,
    };

    let (tx, mut rx) = mpsc::channel::<TokenChunk>(128);

    let cancel = CancellationToken::new();

    let provider_task = tokio::spawn({
        let cancel = cancel.clone();
        async move { provider.stream_generate(request, cancel, tx).await }
    });

    while let Some(chunk) = rx.recv().await {
        print!("{}", chunk.text);
    }

    let result = provider_task.await.unwrap();

    println!("\n\nFINAL:\n{:#?}", result);
}
