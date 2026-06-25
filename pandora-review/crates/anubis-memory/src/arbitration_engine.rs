use crate::arbitration::ArbitrationScore;

pub fn rank_memories(memories: &mut Vec<ArbitrationScore>) {
    for memory in memories.iter_mut() {
        memory.final_score = (memory.semantic_score * 0.5)
            + (memory.temporal_score * 0.3)
            + (memory.graph_score * 0.2);
    }

    memories.sort_by(|a, b| b.final_score.partial_cmp(&a.final_score).unwrap());
}
