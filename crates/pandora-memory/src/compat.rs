//! Backward-compatibility shim for the pre-refactor public API.
//!
//! ⚠️ TEMPORARY: The pre-refactor `pandora-memory` exposed a single
//! type, `HarnessPerformance`, used by `pandora-gene` and
//! `pandora-harness` to track per-harness score history. That
//! type is unrelated to the new generic persistence purpose of
//! this crate, but we keep a working re-export here so existing
//! consumers keep compiling while they migrate to
//! `pandora-harness-scoring` (or to a `pandora-harness` internal
//! module). The IO helpers are kept because the legacy API
//! bundled them with the struct.
//!
//! This module is **not** part of the persistence contract. It
//! will be removed once all consumers have migrated.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Per-harness integer score history with a rolling cap.
///
/// ⚠️ Legacy. Belongs in `pandora-harness-scoring` (or
/// `pandora-harness`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessPerformance {
    scores: HashMap<String, Vec<i32>>,
    max_history: usize,
}

impl Default for HarnessPerformance {
    fn default() -> Self {
        Self::new()
    }
}

impl HarnessPerformance {
    /// Create an empty `HarnessPerformance` with the default
    /// `max_history` of 20.
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
            max_history: 20,
        }
    }

    /// Record a score for `harness`. The history is trimmed to
    /// `max_history` entries.
    pub fn record(&mut self, harness: &str, score: i32) {
        let entry = self.scores.entry(harness.to_string()).or_default();
        entry.push(score);
        if entry.len() > self.max_history {
            entry.remove(0);
        }
    }

    /// Average score for `harness`, or `0.0` if there is no history.
    pub fn average(&self, harness: &str) -> f32 {
        if let Some(scores) = self.scores.get(harness) {
            if scores.is_empty() {
                return 0.0;
            }
            let sum: i32 = scores.iter().sum();
            sum as f32 / scores.len() as f32
        } else {
            0.0
        }
    }

    /// Number of recorded scores for `harness`.
    pub fn count(&self, harness: &str) -> usize {
        self.scores.get(harness).map(|v| v.len()).unwrap_or(0)
    }

    /// All recorded scores for `harness` (oldest first).
    pub fn get_scores(&self, harness: &str) -> Vec<i32> {
        self.scores.get(harness).cloned().unwrap_or_default()
    }

    /// Persist this `HarnessPerformance` to a JSON file at `path`.
    /// ⚠️ Legacy IO helper. Use `pandora-persistence` for new code.
    pub fn save(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self.scores).unwrap_or_default();
        let _ = fs::write(path, json);
    }

    /// Load a `HarnessPerformance` from a JSON file at `path`.
    /// Returns a fresh `HarnessPerformance` if the file does not
    /// exist. ⚠️ Legacy IO helper.
    pub fn load(path: &str) -> Self {
        if !Path::new(path).exists() {
            return Self::new();
        }
        let content = fs::read_to_string(path).unwrap_or_default();
        let scores = serde_json::from_str(&content).unwrap_or_default();
        Self {
            scores,
            max_history: 20,
        }
    }
}
