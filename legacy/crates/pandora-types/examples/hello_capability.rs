use pandora_types::capability_registry::{well_known, CapabilityEntry, CapabilityRegistry};
use std::collections::HashMap;

fn main() {
    let mut registry = CapabilityRegistry::new();
    registry.register(CapabilityEntry {
        capability: well_known::CODE_PARSE.into(),
        provider_id: "tree-sitter-gene".into(),
        provider_kind: "gene".into(),
        confidence: 1.0,
        metadata: HashMap::new(),
    });
    registry.register(CapabilityEntry {
        capability: well_known::NET_HTTP.into(),
        provider_id: "http-gene".into(),
        provider_kind: "gene".into(),
        confidence: 1.0,
        metadata: HashMap::new(),
    });
    println!("All capabilities:");
    for cap in registry.all_capabilities() {
        println!(
            "  {} -> {} providers",
            cap,
            registry.providers_for(cap).len()
        );
    }
    println!(
        "\nHas code_parse? {}",
        registry.providers_for(well_known::CODE_PARSE).len()
    );
}
