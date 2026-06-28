use std::sync::Arc;

use pandora_provider::legacy::ollama::OllamaProvider;
use pandora_provider::registry::ProviderRegistry;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let registry = ProviderRegistry::new();

    registry
        .register_with_name("ollama", Arc::new(OllamaProvider::new()))
        .await;

    let providers = registry.list().await;

    println!("REGISTERED:\n{:#?}", providers);

    let capabilities = registry.capabilities("ollama").await;

    println!("\nCAPABILITIES:\n{:#?}", capabilities);
}
