/// Demonstrates the Event Bus — pub/sub for runtime events.
///
/// Run: cargo run --example hello_event_bus -p pandora-types
use pandora_types::event_bus::{BusEventKind, EventBus};
use std::thread;
use std::time::Duration;

fn main() {
    let bus = EventBus::default_capacity();
    let mut rx = bus.subscribe();

    println!("Publishing events...");

    bus.publish(
        BusEventKind::ExecutionStarted,
        serde_json::json!({"task": "hello-world"}),
        "example",
    );
    thread::sleep(Duration::from_millis(10));

    bus.publish(
        BusEventKind::StageCompleted,
        serde_json::json!({"stage": "plan", "duration_ms": 42}),
        "pipeline",
    );
    thread::sleep(Duration::from_millis(10));

    bus.publish(
        BusEventKind::ExecutionCompleted,
        serde_json::json!({"success": true}),
        "runner",
    );
    thread::sleep(Duration::from_millis(20));

    println!("\nReceiving events:");
    let mut count = 0;
    while let Ok(event) = rx.try_recv() {
        count += 1;
        println!("  [{}] {}", event.kind.label(), event.payload);
    }
    println!(
        "\nReceived {} events. The EventBus connects runtime -> TUI -> API -> Fleet.",
        count
    );
}
