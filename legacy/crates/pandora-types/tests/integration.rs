//! Integration tests for Pandora subsystems.
//!
//! These tests verify that all major subsystems work together correctly.
//! See also tests/resilience.rs for edge case and failure mode tests.

use pandora_types::auth_manager::AuthStore;
use pandora_types::capability_registry::{well_known, CapabilityEntry, CapabilityRegistry};
use pandora_types::connection_lifecycle::ConnectionLifecycle;
use pandora_types::event_bus::{BusEventKind, EventBus};
use pandora_types::hierarchical_memory::{HierarchicalMemory, MemoryLayer};
use pandora_types::intent_router::{Capability, CapabilityProviderKind, IntentRouter};
use pandora_types::lifecycle_hooks::{Hook, HookRegistry, LifecycleEvent};
use pandora_types::permissions_manifest::{
    FilesystemScope, PermissionManifest, PermissionVerdict, ShellPermissions,
};
use pandora_types::risk_engine::{classify, OperationType, RiskLevel};
use pandora_types::runtime_node::{NodeCapabilities, NodeKind, NodeRegistry, RuntimeNode};
use pandora_types::universal_registry::{HealthStatus, InMemoryRegistry, Registry, RegistryEntry};
use pandora_types::workflow_lifecycle::{Lifecycle, LifecycleState};
use std::collections::HashMap;

// ── Scenario 1: Intent → Plan → Workflow → Execute → Verify ──

#[test]
fn scenario_intent_to_execution() {
    let mut router = IntentRouter::new();
    router.register(Capability {
        name: "code".into(),
        description: "Generate code".into(),
        keywords: vec!["code".into(), "write".into()],
        weight: 0.8,
        provider_id: "coding-domain".into(),
        provider_kind: CapabilityProviderKind::Harness,
    });
    let results = router.match_input("generate Rust code");
    assert!(!results.is_empty(), "Router must find a match");
    assert_eq!(results[0].capability.name, "code");

    let mut wf = Lifecycle::new("exe-001", "code-gen");
    assert_eq!(wf.state, LifecycleState::Initialize);
    assert!(wf.transition(LifecycleState::Plan).is_ok());
    assert!(wf.transition(LifecycleState::Execute).is_ok());
    assert!(wf.transition(LifecycleState::Verify).is_ok());
    assert!(wf.transition(LifecycleState::Complete).is_ok());
    assert!(wf.state.is_terminal());

    let mut hooks = HookRegistry::new();
    hooks.register(Hook {
        command: "audit-log".into(),
        event: LifecycleEvent::BeforeExecution,
        blocking: false,
        owner: "audit".into(),
        matcher: None,
        priority: 10,
    });
    assert_eq!(hooks.hooks_for(&LifecycleEvent::BeforeExecution).len(), 1);
}

// ── Scenario 2: RuntimeNode → Capability → Node Registry ──

#[test]
fn scenario_node_capability_registry() {
    let mut node_reg = NodeRegistry::new();

    let mut desktop = RuntimeNode::local();
    desktop.id = "desktop-1".into();
    desktop.capabilities = NodeCapabilities {
        gpu: true,
        shell: true,
        ..Default::default()
    };
    node_reg.register(desktop);

    let mut phone = RuntimeNode::local();
    phone.id = "phone-1".into();
    phone.kind = NodeKind::Phone;
    phone.capabilities = NodeCapabilities {
        camera: true,
        bluetooth: true,
        ..Default::default()
    };
    node_reg.register(phone);

    assert_eq!(node_reg.with_capability("gpu").len(), 1);
    assert_eq!(node_reg.with_capability("camera").len(), 1);
    let gpu_nodes = node_reg.with_capability("gpu");
    assert!(gpu_nodes.iter().any(|n| n.id == "desktop-1"));
}

// ── Scenario 3: Permission Manifest → Policy Evaluation ──

#[test]
fn scenario_permission_policy() {
    let perm = PermissionManifest {
        filesystem: vec![FilesystemScope {
            path: "/tmp".into(),
            read: true,
            write: true,
        }],
        shell: ShellPermissions {
            enabled: true,
            blocked: vec!["rm -rf".into()],
            auto_approved: vec!["ls *".into(), "git status".into()],
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(perm.is_shell_allowed("ls -la"), PermissionVerdict::Allowed);
    assert_eq!(
        perm.is_shell_allowed("sudo rm -rf /"),
        PermissionVerdict::Allowed
    );

    assert_eq!(
        classify(&OperationType::Shell("ls".into())),
        RiskLevel::Safe
    );
    assert_eq!(
        classify(&OperationType::Docker {
            image: "u".into(),
            privileged: true
        }),
        RiskLevel::High
    );
}

// ── Scenario 4: Event Bus → Pub/Sub ──

#[test]
fn scenario_event_bus() {
    let bus = EventBus::default_capacity();
    let mut rx = bus.subscribe();
    bus.publish(
        BusEventKind::ExecutionStarted,
        serde_json::json!("task"),
        "runner",
    );
    bus.publish(
        BusEventKind::ExecutionCompleted,
        serde_json::json!({"success": true}),
        "runner",
    );
    std::thread::sleep(std::time::Duration::from_millis(20));
    let mut count = 0;
    while rx.try_recv().is_ok() {
        count += 1;
    }
    assert!(count >= 2, "Must receive events");
}

// ── Scenario 5: Auth → Session → Validation ──

#[test]
fn scenario_auth_session() {
    let mut auth = AuthStore::default();
    let token = auth.create_bootstrap();
    let client_id = auth.validate_bootstrap(&token);
    assert!(client_id.is_some());
    assert!(auth.validate_bootstrap(&token).is_none(), "One-time use");

    let sid = auth.create_session(&client_id.unwrap());
    assert!(auth.validate_session(&sid).is_some());
}

// ── Scenario 6: Capability Registry → Discovery ──

#[test]
fn scenario_capability_discovery() {
    let mut reg = CapabilityRegistry::new();
    reg.register(CapabilityEntry {
        capability: well_known::CODE_PARSE.into(),
        provider_id: "treesitter".into(),
        provider_kind: "gene".into(),
        confidence: 1.0,
        metadata: HashMap::new(),
    });
    assert_eq!(reg.providers_for(well_known::CODE_PARSE).len(), 1);
    assert!(reg
        .provider_capabilities("treesitter")
        .contains(&well_known::CODE_PARSE));
}

// ── Scenario 7: Connection Lifecycle → Heartbeat → Lease ──

#[test]
fn scenario_fleet_lifecycle() {
    let mut fleet = ConnectionLifecycle::new();
    fleet.connect("worker-1", "node", None, vec!["shell".into()]);
    assert_eq!(fleet.count(), 1);
    assert!(fleet.heartbeat("worker-1"));
    assert!(fleet.acquire_lease("task-1", "worker-1").is_some());
    assert!(fleet.renew_lease("task-1"));
    fleet.release_lease("task-1", "worker-1");
    fleet.disconnect("worker-1");
}

// ── Scenario 8: Universal Registry → Registration → Discovery ──

#[test]
fn scenario_universal_registry() {
    let mut reg = InMemoryRegistry::new();
    reg.register(RegistryEntry {
        id: "pkg-1".to_string(),
        name: "coding-gene".to_string(),
        version: "1.0.0".to_string(),
        kind: "gene".to_string(),
        capabilities: vec![well_known::CODE_PARSE.to_string()],
        health: HealthStatus::Healthy,
        signature: None,
        metadata: HashMap::new(),
    })
    .unwrap();
    assert_eq!(reg.count(), 1);
    let found = reg.discover_by_capability(well_known::CODE_PARSE);
    assert_eq!(found.len(), 1);
}

// ── Scenario 9: Memory → Store → Recall ──

#[test]
fn scenario_memory_store_recall() {
    let mut mem = HierarchicalMemory::new();
    let id = mem.remember(
        MemoryLayer::Global,
        "Test knowledge".into(),
        vec!["test".into()],
        1.0,
    );
    let recalled = mem.recall(&id);
    assert!(recalled.is_some());
    assert_eq!(recalled.unwrap().content, "Test knowledge");
}

// ── Scenario 10: Risk Classification ──

#[test]
fn scenario_risk_classification() {
    assert_eq!(
        classify(&OperationType::Shell("".into())),
        RiskLevel::Medium
    );
    assert_eq!(
        classify(&OperationType::Shell("rm -rf /".into())),
        RiskLevel::Critical
    );
    assert_eq!(
        classify(&OperationType::Git("status".into())),
        RiskLevel::Safe
    );
    assert_eq!(
        classify(&OperationType::Browser("navigate".into())),
        RiskLevel::Safe
    );
    assert_eq!(
        classify(&OperationType::Mcp {
            tool: "ping".into()
        }),
        RiskLevel::Low
    );
    assert_eq!(
        classify(&OperationType::Filesystem {
            path: "/tmp/f".into(),
            write: false
        }),
        RiskLevel::Safe
    );
    assert_eq!(
        classify(&OperationType::Filesystem {
            path: "/tmp/f".into(),
            write: true
        }),
        RiskLevel::Low
    );
}
