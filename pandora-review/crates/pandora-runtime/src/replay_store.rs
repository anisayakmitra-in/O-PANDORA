use std::fs;

use crate::replay::ReplaySession;

pub fn persist_replay(replay: &ReplaySession) {
    fs::create_dir_all("replays").unwrap();

    let path = format!("replays/{}.json", replay.replay_id);

    let serialized = serde_json::to_string_pretty(replay).unwrap();

    fs::write(path, serialized).unwrap();

    println!("[REPLAY] persisted {}", replay.replay_id);
}
