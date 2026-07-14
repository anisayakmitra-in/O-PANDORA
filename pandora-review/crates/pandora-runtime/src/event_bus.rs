use std::fs;

use crate::event::PandoraEvent;

pub fn emit_event(event: &PandoraEvent) {
    fs::create_dir_all("events").unwrap();

    let path = format!("events/{}.json", event.event_id);

    let serialized = serde_json::to_string_pretty(event).unwrap();

    fs::write(path, serialized).unwrap();

    println!("[EVENT BUS] emitted {}", event.event_type);
}
