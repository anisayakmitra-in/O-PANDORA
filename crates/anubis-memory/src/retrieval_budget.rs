pub struct RetrievalBudget;

impl RetrievalBudget {
    pub fn enforce<T: Clone>(memories: &[T], max_items: usize) -> Vec<T> {
        memories.iter().take(max_items).cloned().collect()
    }
}
