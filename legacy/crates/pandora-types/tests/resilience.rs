//! Resilience tests — corrupt data, invalid state, edge cases.

use pandora_types::auth_manager::AuthStore;
use pandora_types::capability_registry::{CapabilityEntry, CapabilityRegistry};
use pandora_types::connection_lifecycle::ConnectionLifecycle;
use pandora_types::context_strategy::{ContextManager, ContextMessage, ContextStrategy};
use pandora_types::event_bus::{BusEventKind, EventBus};
use pandora_types::hierarchical_memory::{HierarchicalMemory, MemoryLayer};
use pandora_types::lifecycle_hooks::{Hook, HookRegistry, LifecycleEvent};
use pandora_types::permissions_manifest::{
    PermissionManifest, PermissionVerdict, ShellPermissions,
};
use pandora_types::risk_engine::{classify, OperationType, RiskLevel};
use pandora_types::runtime_node::{NodeCapabilities, NodeRegistry, RuntimeNode};
use pandora_types::universal_registry::{
    InMemoryRegistry, Registry, RegistryEntry, RegistryHealth,
};

use pandora_types::workflow_lifecycle::{Lifecycle, LifecycleState};
use std::collections::HashMap;

// 1 — Empty/corrupt manifests

#[test]
fn empty_permission_denies_all() {
    let perm = PermissionManifest::default();
    assert!(matches!(
        perm.is_shell_allowed("x"),
        PermissionVerdict::Denied { .. }
    ));
    assert!(matches!(
        perm.is_path_allowed("/", false),
        PermissionVerdict::Denied { .. }
    ));
    assert!(matches!(
        perm.is_host_allowed("x.com"),
        PermissionVerdict::Denied { .. }
    ));
}

#[test]
fn partial_manifest_allows_configured() {
    let perm = PermissionManifest {
        shell: ShellPermissions {
            enabled: true,
            blocked: vec![],
            auto_approved: vec![],
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(matches!(
        perm.is_shell_allowed("echo"),
        PermissionVerdict::Allowed
    ));
}

// 2 — Registry edge cases

#[test]
fn empty_cap_registry() {
    let r = CapabilityRegistry::new();
    assert!(r.all_capabilities().is_empty());
    assert!(r.providers_for("x").is_empty());
}

#[test]
fn cap_registry_accepts_duplicates() {
    let mut r = CapabilityRegistry::new();
    let e = || CapabilityEntry {
        capability: "a".into(),
        provider_id: "p".into(),
        provider_kind: "g".into(),
        confidence: 1.0,
        metadata: HashMap::new(),
    };
    r.register(e());
    r.register(e());
    assert!(!r.providers_for("a").is_empty());
}

#[test]
fn univ_registry_rejects_dup_id() {
    let mut r = InMemoryRegistry::new();
    let e = |id: &str| RegistryEntry {
        id: id.into(),
        name: "".into(),
        version: "1".into(),
        kind: "t".into(),
        capabilities: vec![],
        health: RegistryHealth::Healthy,
        signature: None,
        metadata: HashMap::new(),
    };
    assert!(r.register(e("x")).is_ok());
    assert!(r.register(e("x")).is_err());
}

// 3 — Memory edge cases

#[test]
fn empty_memory() {
    let mut m = HierarchicalMemory::new();
    assert!(m.search_by_tags(&[], None).is_empty());
    assert!(m.search_by_content("", None).is_empty());
    assert!(m.recall("x").is_none());
}

#[test]
fn memory_layer_isolation() {
    let mut m = HierarchicalMemory::new();
    m.remember(MemoryLayer::Global, "t".into(), vec![], 1.0);
    assert!(m.layer_entries(MemoryLayer::Session).is_empty());
}

// 4 — Context strategy

#[test]
fn empty_context_not_over_limit() {
    let cm = ContextManager::new(0, ContextStrategy::DropOldest);
    assert!(!cm.is_over_limit());
}

#[test]
fn context_push_does_not_hang() {
    let mut cm = ContextManager::new(2, ContextStrategy::DropOldest);
    cm.push(ContextMessage {
        role: "u".into(),
        content: "x".repeat(100),
        timestamp: 1,
        pinned: false,
    });
    cm.push(ContextMessage {
        role: "u".into(),
        content: "y".repeat(100),
        timestamp: 2,
        pinned: false,
    });
}

// 5 — Event bus

#[test]
fn bus_no_subscribers() {
    let bus = EventBus::default_capacity();
    bus.publish(BusEventKind::ExecutionStarted, serde_json::json!("t"), "t");
}

#[test]
fn bus_dropped_subscriber() {
    let bus = EventBus::default_capacity();
    let _rx = bus.subscribe();
    let mut rx2 = bus.subscribe();
    bus.publish(
        BusEventKind::ExecutionCompleted,
        serde_json::json!("ok"),
        "t",
    );
    std::thread::sleep(std::time::Duration::from_millis(20));
    assert!(rx2.try_recv().is_ok());
}

// 6 — Auth

#[test]
fn empty_tokens() {
    let mut a = AuthStore::default();
    assert!(a.validate_bootstrap("").is_none());
    assert!(a.validate_api_key("").is_none());
    assert!(a.validate_session("").is_none());
}

#[test]
fn bootstrap_one_time() {
    let mut a = AuthStore::default();
    let t = a.create_bootstrap();
    assert!(a.validate_bootstrap(&t).is_some());
    assert!(a.validate_bootstrap(&t).is_none());
}

// 7 — Connection lifecycle

#[test]
fn fleet_empty() {
    let mut f = ConnectionLifecycle::new();
    assert_eq!(f.count(), 0);
    assert!(!f.heartbeat("x"));
}

#[test]
fn double_lease_fails() {
    let mut f = ConnectionLifecycle::new();
    f.connect("w", "n", None, vec![]);
    assert!(f.acquire_lease("t", "w").is_some());
    assert!(f.acquire_lease("t", "w2").is_none());
}

#[test]
fn disconnect_ghost_safe() {
    let mut f = ConnectionLifecycle::new();
    f.disconnect("ghost");
    assert_eq!(f.count(), 0);
}

#[test]
fn release_unheld_lease_safe() {
    let mut f = ConnectionLifecycle::new();
    f.connect("w", "n", None, vec![]);
    f.release_lease("nonexistent", "w");
}

// 8 — RuntimeNode

#[test]
fn node_registry_empty() {
    let r = NodeRegistry::new();
    assert!(r.with_capability("x").is_empty());
}

#[test]
fn node_unknown_cap() {
    let mut reg = NodeRegistry::new();
    let mut node = RuntimeNode::local();
    node.id = "test".into();
    node.capabilities = NodeCapabilities {
        gpu: true,
        ..Default::default()
    };
    reg.register(node);
    assert!(reg.with_capability("nonexistent").is_empty());
}

// 9 — Workflow

#[test]
fn illegal_transition() {
    let mut wf = Lifecycle::new("t", "t");
    assert!(wf.transition(LifecycleState::Complete).is_err());
}

#[test]
fn terminal_stops_all() {
    let mut wf = Lifecycle::new("t", "t");
    let _ = wf.transition(LifecycleState::Plan);
    let _ = wf.transition(LifecycleState::Execute);
    let _ = wf.transition(LifecycleState::Verify);
    let _ = wf.transition(LifecycleState::Complete);
    assert!(wf.transition(LifecycleState::Execute).is_err());
}

#[test]
fn zero_retries() {
    let mut wf = Lifecycle::new("t", "t");
    let _ = wf.transition(LifecycleState::Plan);
    let _ = wf.transition(LifecycleState::Execute);
    wf.step("t", 0);
    assert!(!wf.can_retry());
}

#[test]
fn full_recovery() {
    let mut wf = Lifecycle::new("t", "t");
    let _ = wf.transition(LifecycleState::Plan);
    let _ = wf.transition(LifecycleState::Execute);
    let _ = wf.transition(LifecycleState::Verify);
    let _ = wf.transition(LifecycleState::Recover);
    let _ = wf.transition(LifecycleState::Execute);
    let _ = wf.transition(LifecycleState::Verify);
    let _ = wf.transition(LifecycleState::Complete);
    assert!(wf.state.is_terminal());
}

// 10 — Hooks

#[test]
fn hooks_empty() {
    let h = HookRegistry::new();
    assert_eq!(h.count(), 0);
    assert!(h.hooks_for(&LifecycleEvent::BeforeExecution).is_empty());
}

#[test]
fn hooks_wrong_event() {
    let mut h = HookRegistry::new();
    h.register(Hook {
        command: "x".into(),
        event: LifecycleEvent::BeforeExecution,
        blocking: false,
        owner: "t".into(),
        matcher: None,
        priority: 5,
    });
    assert!(h.hooks_for(&LifecycleEvent::AfterExecution).is_empty());
}

// 11 — Risk classification

#[test]
fn empty_shell_medium() {
    assert_eq!(
        classify(&OperationType::Shell("".into())),
        RiskLevel::Medium
    );
}

#[test]
fn unknown_shell_medium() {
    assert_eq!(
        classify(&OperationType::Shell("unknown".into())),
        RiskLevel::Medium
    );
}

#[test]
fn privileged_docker_high() {
    assert_eq!(
        classify(&OperationType::Docker {
            image: "u".into(),
            privileged: true
        }),
        RiskLevel::High
    );
}

#[test]
fn browser_navigate_safe() {
    assert_eq!(
        classify(&OperationType::Browser("navigate".into())),
        RiskLevel::Safe
    );
}

#[test]
fn mcp_tool_low() {
    assert_eq!(
        classify(&OperationType::Mcp {
            tool: "ping".into()
        }),
        RiskLevel::Low
    );
}

#[test]
fn git_status_safe() {
    assert_eq!(
        classify(&OperationType::Git("status".into())),
        RiskLevel::Safe
    );
}

// 12 — Capabilities

#[test]
fn cap_with_empty_strings() {
    let mut r = CapabilityRegistry::new();
    r.register(CapabilityEntry {
        capability: "".into(),
        provider_id: "".into(),
        provider_kind: "".into(),
        confidence: -1.0,
        metadata: HashMap::new(),
    });
}

#[test]
fn cap_case_sensitive() {
    let mut r = CapabilityRegistry::new();
    r.register(CapabilityEntry {
        capability: "Code.Parse".into(),
        provider_id: "p".into(),
        provider_kind: "g".into(),
        confidence: 1.0,
        metadata: HashMap::new(),
    });
    assert_eq!(r.providers_for("code.parse").len(), 0);
    assert_eq!(r.providers_for("Code.Parse").len(), 1);
}
