use crate::MemoryRecord;

pub fn synthesize_reflection(memories: &[MemoryRecord]) -> String {
    let mut synthesis = String::new();

    synthesis.push_str("ANUBIS SYNTHESIS\n\n");

    synthesis.push_str(&format!("Observed {} memories.\n\n", memories.len()));

    let tags: Vec<String> = memories.iter().flat_map(|m| m.tags.clone()).collect();

    synthesis.push_str(&format!("Dominant tags: {:?}\n\n", tags));

    if let Some(memory) = memories.last() {
        synthesis.push_str(&format!("Latest cognition:\n{}\n", memory.prompt));
    }

    synthesis
}
