use pandora_types::workflow_lifecycle::{Lifecycle, LifecycleState};

fn main() {
    let mut wf = Lifecycle::new("demo-001", "Example Workflow");
    println!("Workflow: {} (state: {})", wf.name, wf.state.label());

    assert!(wf.transition(LifecycleState::Plan).is_ok());
    println!("  Plan");
    wf.step("design", 3);

    assert!(wf.transition(LifecycleState::Execute).is_ok());
    wf.step("implement", 3);

    assert!(wf.transition(LifecycleState::Verify).is_ok());
    wf.step("test", 3);

    assert!(wf.transition(LifecycleState::Recover).is_ok());
    println!("  Recovery");
    assert!(wf.transition(LifecycleState::Execute).is_ok());

    assert!(wf.transition(LifecycleState::Verify).is_ok());
    assert!(wf.transition(LifecycleState::Complete).is_ok());

    println!("Workflow completed: {} steps", wf.steps.len());
    println!("Canonical lifecycle: Initialize -> Plan -> Execute -> Verify -> Recover -> Complete");
}
