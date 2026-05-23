use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayScore {
    pub replay_id: String,

    pub entropy: f32,

    pub loop_detected: bool,

    pub rollback_triggered: bool,

    pub quality_score: f32,
}

pub struct ReplayScorer;

impl ReplayScorer {
    pub fn evaluate(replay: &mut ReplayScore) {
        let mut score = 1.0;

        score -= replay.entropy * 0.2;

        if replay.loop_detected {
            score -= 0.3;
        }

        if replay.rollback_triggered {
            score -= 0.2;
        }

        if score < 0.0 {
            score = 0.0;
        }

        replay.quality_score = score;
    }
}
