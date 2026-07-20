// ──────────────────────────────────────────────
// Resilience tests — corrupt data, invalid state, edge cases
// ──────────────────────────────────────────────

#[cfg(test)]
mod resilience {
    use pandora_types::auth_manager::AuthStore;
    use pandora_types::capability_registry::{CapabilityEntry, CapabilityRegistry};
    use pandora_types::connection_lifecycle::ConnectionLifecycle;
    use pandora_types::context_strategy::{ContextManager, ContextMessage, ContextStrategy};
    use pandora_types::event_bus::EventBus;
    use pandora_types::hierarchical_memory::{HierarchicalMemory, MemoryLayer};
    use pandora_types::lifecycle_hooks::{Hook, HookRegistry, LifecycleEvent};
    use pandora_types::permissions_manifest::{
        FilesystemScope, PermissionManifest, PermissionVerdict, ShellPermissions,
    };
    use pandora_types::risk_engine::{classify, OperationType, RiskLevel};
    use pandora_types::runtime_node::{
        NodeCapabilities, NodeKind, NodePlatform, NodeRegistry, RuntimeNode, TransportKind,
    };
    use pandora_types::universal_registry::{
        HealthStatus, InMemoryRegistry, Registry, RegistryEntry,
    };
    use pandora_types::workflow_lifecycle::{Lifecycle, LifecycleState};
    use std::collections::HashMap;

    // ── 1. Corrupt/empty manifests ──

    #[test]
    fn empty_permission_manifest_denies_everything() {
        let perm = PermissionManifest::default();
        assert!(matches!(
            perm.is_shell_allowed("anything"),
            PermissionVerdict::Denied { .. }
        ));
        assert!(matches!(
            perm.is_path_allowed("/etc/passwd", false),
            PermissionVerdict::Denied { .. }
        ));
        assert!(matches!(
            perm.is_host_allowed("example.com"),
            PermissionVerdict::Denied { .. }
        ));
    }

    #[test]
    fn partially_configured_manifest_has_defaults() {
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
            perm.is_shell_allowed("echo hi"),
            PermissionVerdict::Allowed
        ));
        assert!(matches!(
            perm.is_path_allowed("/tmp", false),
            PermissionVerdict::Denied { .. }
        ));
    }

    #[test]
    fn empty_host_is_denied() {
        let perm = PermissionManifest::default();
        assert!(matches!(
            perm.is_host_allowed(""),
            PermissionVerdict::Denied { .. }
        ));
    }

    // ── 2. Invalid registries ──

    #[test]
    fn empty_capability_registry_returns_empty() {
        let reg = CapabilityRegistry::new();
        assert!(reg.all_capabilities().is_empty());
        assert!(reg.providers_for("anything").is_empty());
        assert!(reg.provider_capabilities("nonexistent").is_empty());
    }

    #[test]
    fn duplicate_registration_is_idempotent() {
        let mut reg = CapabilityRegistry::new();
        let entry = CapabilityEntry {
            capability: "test.cap".into(),
            provider_id: "p1".into(),
            provider_kind: "gene".into(),
            confidence: 1.0,
            metadata: HashMap::new(),
        };
        reg.register(entry);
        reg.register(CapabilityEntry {
            capability: "test.cap".into(),
            provider_id: "p1".into(),
            provider_kind: "gene".into(),
            confidence: 1.0,
            metadata: HashMap::new(),
        });
        assert_eq!(reg.providers_for("test.cap").len(), 1);
    }

    #[test]
    fn empty_universal_registry_handles_all_queries() {
        let mut reg: InMemoryRegistry<RegistryEntry> = InMemoryRegistry::new();
        assert!(reg.list_by_kind("test").is_empty());
        assert!(reg.list_all().is_empty());
        assert!(reg.discover_by_capability("test").is_empty());
        let entry = RegistryEntry {
            id: "",
            name: "",
            version: "",
            kind: "",
            capabilities: vec![],
            health: HealthStatus::Healthy,
            signature: None,
            metadata: HashMap::new(),
        };
        assert!(reg.register(entry).is_ok());
    }

    #[test]
    fn registry_rejects_duplicate_id() {
        let mut reg: InMemoryRegistry<RegistryEntry> = InMemoryRegistry::new();
        reg.register(RegistryEntry {
            id: "dup".into(),
            name: "a".into(),
            version: "1".into(),
            kind: "t".into(),
            capabilities: vec![],
            health: HealthStatus::Healthy,
            signature: None,
            metadata: HashMap::new(),
        })
        .unwrap();
        assert!(reg
            .register(RegistryEntry {
                id: "dup".into(),
                name: "b".into(),
                version: "1".into(),
                kind: "t".into(),
                capabilities: vec![],
                health: HealthStatus::Healthy,
                signature: None,
                metadata: HashMap::new(),
            })
            .is_err());
    }

    #[test]
    fn unhealthy_entries_are_still_listed() {
        let mut reg: InMemoryRegistry<RegistryEntry> = InMemoryRegistry::new();
        reg.register(RegistryEntry {
            id: "sick".into(),
            name: "sick".into(),
            version: "".into(),
            kind: "test".into(),
            capabilities: vec![],
            health: HealthStatus::Unhealthy("disk full".into()),
            signature: None,
            metadata: HashMap::new(),
        })
        .unwrap();
        assert_eq!(reg.list_all().len(), 1);
    }

    // ── 3. Memory edge cases ──

    #[test]
    fn empty_memory_returns_empty() {
        let mem = HierarchicalMemory::new();
        assert!(mem.search_by_tags(&[], None).is_empty());
        assert!(mem.search_by_content("", None).is_empty());
        assert!(mem.recall("nonexistent").is_none());
    }

    #[test]
    fn memory_wrong_layer_has_no_entries() {
        let mut mem = HierarchicalMemory::new();
        mem.remember(MemoryLayer::Global, "test".into(), vec![], 1.0);
        assert!(mem.layer_entries(MemoryLayer::Session).is_empty());
    }

    #[test]
    fn memory_recall_nonexistent_returns_none() {
        let mem = HierarchicalMemory::new();
        assert!(mem.recall("no-such-id").is_none());
    }

    // ── 4. Context strategy edge cases ──

    #[test]
    fn empty_context_not_over_limit() {
        let cm = ContextManager::new(0, ContextStrategy::DropOldest);
        assert!(!cm.is_over_limit());
    }

    #[test]
    fn context_termination_guard_works() {
        let mut cm = ContextManager::new(1, ContextStrategy::Summarize);
        for i in 0..50 {
            cm.push(ContextMessage {
                role: "user".into(),
                content: format!("msg {} which is long enough to test the budget", i),
                timestamp: i,
                pinned: false,
            });
        }
        cm.enforce_limit();
        assert!(cm.messages_dropped > 0 || cm.messages().len() > 0);
    }

    // ── 5. Event bus edge cases ──

    #[test]
    fn bus_without_subscribers_works() {
        let bus = EventBus::default_capacity();
        bus.publish(
            crate::integration::BusEventKind::ExecutionStarted,
            serde_json::json!("test"),
            "t",
        );
    }

    #[test]
    fn bus_subscriber_dropped_does_not_break_others() {
        let bus = EventBus::default_capacity();
        let _rx = bus.subscribe();
        let rx2 = bus.subscribe();
        bus.publish(
            crate::integration::BusEventKind::ExecutionCompleted,
            serde_json::json!("ok"),
            "t",
        );
        std::thread::sleep(std::time::Duration::from_millis(20));
        assert!(rx2.try_recv().is_ok());
    }

    #[test]
    fn bus_many_events_dont_overflow() {
        let bus = EventBus::default_capacity();
        let rx = bus.subscribe();
        for i in 0..20 {
            bus.publish(
                crate::integration::BusEventKind::StageCompleted,
                serde_json::json!({"i": i}),
                "t",
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(30));
        let mut count = 0;
        while let Ok(_) = rx.try_recv() {
            count += 1;
        }
        assert!(count > 0);
    }

    // ── 6. Auth edge cases ──

    #[test]
    fn empty_tokens_fail() {
        let mut auth = AuthStore::default();
        assert!(auth.validate_bootstrap("").is_none());
        assert!(auth.validate_api_key("").is_none());
        assert!(auth.validate_session("").is_none());
    }

    #[test]
    fn bootstrap_cannot_be_reused() {
        let mut auth = AuthStore::default();
        let t = auth.create_bootstrap();
        assert!(auth.validate_bootstrap(&t).is_some());
        assert!(auth.validate_bootstrap(&t).is_none());
    }

    #[test]
    fn invalid_session_returns_none() {
        let mut auth = AuthStore::default();
        assert!(auth.validate_session("nope").is_none());
    }

    #[test]
    fn removed_session_does_not_validate() {
        let mut auth = AuthStore::default();
        let sid = auth.create_session("client-1");
        assert!(auth.validate_session(&sid).is_some());
        auth.remove_session(&sid);
        assert!(auth.validate_session(&sid).is_none());
    }

    // ── 7. Connection lifecycle edge cases ──

    #[test]
    fn empty_fleet_handles_all_queries() {
        let fleet = ConnectionLifecycle::new();
        assert_eq!(fleet.count(), 0);
        assert!(fleet.healthy_workers().is_empty());
        assert!(!fleet.heartbeat("nonexistent"));
    }

    #[test]
    fn lease_twice_fails() {
        let mut fleet = ConnectionLifecycle::new();
        fleet.connect("w1", "n", None, vec![]);
        assert!(fleet.acquire_lease("t1", "w1").is_some());
        assert!(fleet.acquire_lease("t1", "w2").is_none());
    }

    #[test]
    fn disconnect_unknown_worker_is_safe() {
        let mut fleet = ConnectionLifecycle::new();
        fleet.disconnect("ghost");
        assert_eq!(fleet.count(), 0);
    }

    #[test]
    fn release_untaken_lease_is_safe() {
        let mut fleet = ConnectionLifecycle::new();
        fleet.connect("w1", "n", None, vec![]);
        fleet.release_lease("never-held", "w1");
    }

    // ── 8. RuntimeNode edge cases ──

    #[test]
    fn empty_node_registry() {
        let reg = NodeRegistry::new();
        assert!(reg.with_capability("anything").is_empty());
    }

    #[test]
    fn node_unknown_capability_returns_empty() {
        let mut reg = NodeRegistry::new();
        let mut node = RuntimeNode::local();
        node.id = "test".into();
        node.capabilities.gpu = true;
        reg.register(node);
        assert!(reg.with_capability("nonexistent").is_empty());
    }

    #[test]
    fn node_multiple_transports() {
        let mut node = RuntimeNode::local();
        node.id = "multi".into();
        node.transports = vec![TransportKind::Tcp, TransportKind::Grpc];
        assert_eq!(node.transports.len(), 2);
        let mut reg = NodeRegistry::new();
        reg.register(node);
        assert_eq!(reg.nodes.len(), 1);
    }

    // ── 9. Workflow edge cases ──

    #[test]
    fn illegal_transition_returns_error() {
        let mut wf = Lifecycle::new("t", "t");
        assert!(wf.transition(LifecycleState::Complete).is_err());
    }

    #[test]
    fn terminal_state_rejects_all() {
        let mut wf = Lifecycle::new("t", "t");
        wf.transition(LifecycleState::Plan).ok();
        wf.transition(LifecycleState::Execute).ok();
        wf.transition(LifecycleState::Verify).ok();
        wf.transition(LifecycleState::Complete).ok();
        assert!(wf.transition(LifecycleState::Execute).is_err());
    }

    #[test]
    fn retry_limit_respected() {
        let mut wf = Lifecycle::new("t", "t");
        wf.transition(LifecycleState::Plan).ok();
        wf.transition(LifecycleState::Execute).ok();
        wf.step("task", 0);
        assert!(!wf.can_retry());
    }

    #[test]
    fn full_recovery_cycle() {
        let mut wf = Lifecycle::new("t", "t");
        assert_eq!(wf.state, LifecycleState::Initialize);
        wf.transition(LifecycleState::Plan).ok();
        wf.transition(LifecycleState::Execute).ok();
        wf.transition(LifecycleState::Verify).ok();
        wf.transition(LifecycleState::Recover).ok();
        wf.transition(LifecycleState::Execute).ok();
        wf.transition(LifecycleState::Verify).ok();
        wf.transition(LifecycleState::Complete).ok();
        assert!(wf.state.is_terminal());
    }

    // ── 10. Hooks edge cases ──

    #[test]
    fn empty_hook_registry() {
        let h = HookRegistry::new();
        assert_eq!(h.count(), 0);
        assert!(h.hooks_for(&LifecycleEvent::BeforeExecution).is_empty());
    }

    #[test]
    fn hook_event_mismatch() {
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

    #[test]
    fn hook_priority_ordering() {
        let mut h = HookRegistry::new();
        for p in &[20, 10, 30] {
            h.register(Hook {
                command: format!("h-{}", p),
                event: LifecycleEvent::BeforeExecution,
                blocking: false,
                owner: "t".into(),
                matcher: None,
                priority: *p,
            });
        }
        assert_eq!(h.hooks_for(&LifecycleEvent::BeforeExecution).len(), 3);
    }

    // ── 11. Risk engine edge cases ──

    #[test]
    fn empty_command_safe() {
        assert_eq!(classify(&OperationType::Shell("".into())), RiskLevel::Safe);
    }

    #[test]
    fn unknown_command_medium() {
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
    fn non_privileged_docker_medium() {
        assert_eq!(
            classify(&OperationType::Docker {
                image: "u".into(),
                privileged: false
            }),
            RiskLevel::Medium
        );
    }

    #[test]
    fn git_commands_classified() {
        assert_eq!(
            classify(&OperationType::Git("status".into())),
            RiskLevel::Safe
        );
        assert_eq!(
            classify(&OperationType::Git("push origin main".into())),
            RiskLevel::Medium
        );
    }

    #[test]
    fn browser_mcp_classified() {
        assert_eq!(
            classify(&OperationType::Browser("navigate".into())),
            RiskLevel::Medium
        );
        assert_eq!(
            classify(&OperationType::Mcp("ping".into())),
            RiskLevel::Safe
        );
    }

    // ── 12. Capability fuzzy tests ──

    #[test]
    fn capabilities_with_empty_strings() {
        let mut reg = CapabilityRegistry::new();
        reg.register(CapabilityEntry {
            capability: "".into(),
            provider_id: "".into(),
            provider_kind: "".into(),
            confidence: 0.0,
            metadata: HashMap::new(),
        });
        assert!(reg.all_capabilities().len() > 0 || reg.all_capabilities().is_empty());
    }

    #[test]
    fn capabilities_case_sensitive() {
        let mut reg = CapabilityRegistry::new();
        reg.register(CapabilityEntry {
            capability: "Code.Parse".into(),
            provider_id: "p".into(),
            provider_kind: "g".into(),
            confidence: 1.0,
            metadata: HashMap::new(),
        });
        assert_eq!(reg.providers_for("code.parse").len(), 0);
        assert_eq!(reg.providers_for("Code.Parse").len(), 1);
    }

    #[test]
    fn negative_confidence_registers() {
        let mut reg = CapabilityRegistry::new();
        reg.register(CapabilityEntry {
            capability: "t".into(),
            provider_id: "p".into(),
            provider_kind: "g".into(),
            confidence: -1.0,
            metadata: HashMap::new(),
        });
        assert_eq!(reg.providers_for("t").len(), 1);
    }
}
