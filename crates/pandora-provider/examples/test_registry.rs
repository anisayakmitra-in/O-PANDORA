use std::sync::Arc;

use pandora_provider::ollama::OllamaProvider;

use pandora_provider::registry::ProviderRegistry;

#[tokio::main]
async fn main() {

    let registry =
        ProviderRegistry::new();

    registry
        .register(
            "ollama",
            Arc::new(
                OllamaProvider::new()
            ),
        )
        .await;

    let providers =
        registry
            .list()
            .await;

    println!(
        "REGISTERED:\n{:#?}",
        providers
    );

    let capabilities =
        registry
            .capabilities(
                "ollama"
            )
            .await;

    println!(
        "\nCAPABILITIES:\n{:#?}",
        capabilities
    );
}
