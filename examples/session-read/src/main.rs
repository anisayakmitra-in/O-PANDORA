//! session-read — read and inspect a Pandora session.
//!
//! Sessions are stored as JSON files in ~/.pandora/sessions/.
//! This example loads one and prints its timeline.
//!
//! Run with: cargo run --example read <session-id>

use pandora_types::session::SessionStore;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example read <session-id>");
        eprintln!("Sessions are in ~/.pandora/sessions/");
        return;
    }

    let id = &args[1];
    let store = SessionStore::new();

    match store.get(id) {
        Some(session) => {
            println!("Session: {}", session.id);
            println!("  Prompt:  {}", session.prompt);
            println!("  Status:  {:?}", session.status);
            println!("  Timeline: {} frames", session.timeline.len());
            for (i, frame) in session.timeline.iter().enumerate() {
                println!("    {}. {} — {}/{} ({})",
                    i + 1, frame.step_label, frame.provider, frame.model,
                    if frame.success { "ok" } else { "fail" }
                );
            }
        }
        None => eprintln!("Session not found: {id}"),
    }
}
