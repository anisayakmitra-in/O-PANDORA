use std::collections::HashMap;

pub struct HarnessPerformance {
    scores: HashMap<String, Vec<i32>>,
    max_history: usize,
}

impl HarnessPerformance {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
            max_history: 20,
        }
    }

    pub fn record(&mut self, harness: &str, score: i32) {
        let entry = self
            .scores
            .entry(harness.to_string())
            .or_default();

        entry.push(score);

        // prevent unbounded growth
        if entry.len() > self.max_history {
            entry.remove(0);
        }
    }

    pub fn average(&self, harness: &str) -> f32 {
        if let Some(scores) = self.scores.get(harness) {
            let sum: i32 = scores.iter().sum();
            sum as f32 / scores.len() as f32
        } else {
            0.0
        }
    }

    pub fn latest(&self, harness: &str) -> Option<i32> {
        self.scores
            .get(harness)
            .and_then(|v| v.last().copied())
    }

    pub fn count(&self, harness: &str) -> usize {
        self.scores
            .get(harness)
            .map(|v| v.len())
            .unwrap_or(0)
    }
}

