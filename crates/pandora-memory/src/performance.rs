use std::collections::HashMap;
use std::fs;
use std::path::Path;

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

    pub fn record(
        &mut self,
        harness: &str,
        score: i32,
    ) {

        let entry = self
            .scores
            .entry(harness.to_string())
            .or_default();

        entry.push(score);

        if entry.len() > self.max_history {
            entry.remove(0);
        }
    }

    pub fn average(
        &self,
        harness: &str,
    ) -> f32 {

        if let Some(scores) =
            self.scores.get(harness)
        {

            if scores.is_empty() {
                return 0.0;
            }

            let sum: i32 =
                scores.iter().sum();

            sum as f32 / scores.len() as f32

        } else {

            0.0
        }
    }

    pub fn count(
        &self,
        harness: &str,
    ) -> usize {

        self.scores
            .get(harness)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    pub fn get_scores(
        &self,
        harness: &str,
    ) -> Vec<i32> {

        self.scores
            .get(harness)
            .cloned()
            .unwrap_or_default()
    }

    pub fn save(
        &self,
        path: &str,
    ) {

        let json =
            serde_json::to_string_pretty(
                &self.scores
            ).unwrap();

        fs::write(path, json).unwrap();
    }

    pub fn load(
        path: &str,
    ) -> Self {

        if !Path::new(path).exists() {

            return Self::new();
        }

        let content =
            fs::read_to_string(path)
                .unwrap();

        let scores =
            serde_json::from_str(&content)
                .unwrap_or_default();

        Self {
            scores,
            max_history: 20,
        }
    }
}


