//! Pandora Panoptes Store — extracted from pandora-runtime (Phase 1B).
//!
use std::fs;

use crate::panoptes::CognitionScore;

pub fn persist_score(score: &CognitionScore) {
    fs::create_dir_all("panoptes").unwrap();

    let path = format!("panoptes/{}.json", score.score_id);

    let serialized = serde_json::to_string_pretty(score).unwrap();

    fs::write(path, serialized).unwrap();

    println!("[PANOPTES] persisted {}", score.score_id);
}
