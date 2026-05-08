use crate::MemoryRecord;

pub fn compute_salience(
    memory: &MemoryRecord
) -> f32 {

    let mut score = 1.0;

    score +=
        memory
            .related_memories
            .len() as f32;

    score +=
        memory
            .tags
            .len() as f32 * 0.5;

    score
}

pub fn decay_salience(
    memory: &mut MemoryRecord
) {

    memory.salience *= 0.98;
}
