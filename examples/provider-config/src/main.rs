//! provider-config — configure and inspect LLM provider connections.
//!
//! Connections are stored in ~/.pandora/connections.toml.
//! This example lists all configured connections and their health.
//!
//! Run with: cargo run --example config

use pandora_types::connection_manager::ConnectionRegistry;

fn main() {
    let registry = ConnectionRegistry::load();

    if registry.connections.is_empty() {
        println!("No connections configured.");
        println!();
        println!("Add one with:");
        println!("  pandora connection add local ollama http://localhost:11434");
        println!("  pandora connection add deepseek openai-compatible https://api.deepseek.com --model deepseek-chat");
        return;
    }

    println!("{} connection(s):", registry.connections.len());
    for conn in &registry.connections {
        let healthy = registry.healthy().iter().any(|c| c.name == conn.name);
        println!();
        println!("  Name:     {}", conn.name);
        println!("  Kind:     {:?}", conn.kind);
        println!("  Endpoint: {}", conn.endpoint);
        println!("  Model:    {}", conn.default_model);
        println!("  Health:   {}", if healthy { "healthy" } else { conn.health_status.as_str() });
    }
}
