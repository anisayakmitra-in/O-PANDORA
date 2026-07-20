use pandora_types::hierarchical_memory::{HierarchicalMemory, MemoryLayer};

fn main() {
    let mut mem = HierarchicalMemory::new();
    mem.remember(
        MemoryLayer::Global,
        "Pandora is a governed execution runtime".into(),
        vec!["pandora".into(), "architecture".into()],
        1.0,
    );
    mem.remember(
        MemoryLayer::Session,
        "Working on SDK example".into(),
        vec!["session".into(), "example".into()],
        0.5,
    );
    println!("Searching for pandora:");
    for e in mem.search_by_tags(&["pandora"], None) {
        println!(
            "  [{}] {} (imp: {})",
            e.layer.label(),
            e.content,
            e.importance
        );
    }
}
