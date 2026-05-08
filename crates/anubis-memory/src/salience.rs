use crate::storage::MemoryRecord;

pub fn compute_salience(
    memory:
        &MemoryRecord,
) -> f32 {

    let mut salience =
        memory.score;

    salience +=
        memory.related.len()
        as f32;

    if memory.layer
        == "long_term"
    {
        salience += 5.0;
    }

    salience
}
