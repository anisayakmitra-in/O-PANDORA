use crate::temporal::TemporalMemory;

pub fn sort_by_recency(memories: &mut [TemporalMemory]) {
    memories.sort_by_key(|b| std::cmp::Reverse(b.sequence));
}
