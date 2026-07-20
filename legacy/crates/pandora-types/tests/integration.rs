//! End-to-end integration tests — complete user journeys.
//!
//! These tests exercise real Pandora subsystems without mocks.
//! They do NOT require an LLM provider — all test scenarios use
//! infrastructure-only code paths (planning, routing, governance,
//! permissions, memory, nodes, auth).
//!
//! Run: cargo test -p pandora-types -- integration

#[cfg(test)]
mod integration {
    use pandora_types::auth_manager::AuthStore;
    use pandora_types::capability_registry::{well_known, CapabilityEntry, CapabilityRegistry};
    use pandora_types::connection_lifecycle::ConnectionLifecycle;
    use pandora_types::context_strategy::{ContextManager, ContextMessage, ContextStrategy};
    use pandora_types::event_bus::{BusEventKind, EventBus};
    use pandora_types::hierarchical_memory::{HierarchicalMemory, MemoryLayer};
    use pandora_types::intent_router::{Capability, CapabilityProviderKind, IntentRouter};
    use pandora_types::lifecycle_hooks::{Hook, HookRegistry, LifecycleEvent};
    use pandora_types::permissions_manifest::{
        FilesystemScope, PermissionManifest, PermissionVerdict, ShellPermissions,
    };
    use pandora_types::risk_engine::{classify, OperationType, RiskLevel};
    use pandora_types::runtime_node::{
         NodeKind, NodePlatform, NodeRegistry, RuntimeNode, TransportKind,
    };
    use pandora_types::universal_registry::{
        HealthStatus, InMemoryRegistry, Registry, RegistryEntry,
    };
    use pandora_types::workflow_lifecycle::{Lifecycle, LifecycleState};
    use std::collections::HashMap;

    // ──────────────────────────────────────────────
    // Scenario 1: Intent → Plan → Workflow → Execute → Verify
    // ──────────────────────────────────────────────
    #[test]
    fn scenario_1_intent_to_execution() {
        // 1. Intent Router registers capabilities
        let mut router = IntentRouter::new();
        router.register(Capability {
            name: "code".into(),
            description: "Generate and edit source code".into(),
            keywords: vec!["code".into(), "generate".into(), "write".into()],
            weight: 0.8,
            provider_id: "coding-domain".into(),
            provider_kind: CapabilityProviderKind::Harness,
        });
        router.register(Capability {
            name: "docker".into(),
            description: "Build and run containers".into(),
            keywords: vec!["docker".into(), "container".into(), "build".into()],
            weight: 0.5,
            provider_id: "devops-domain".into(),
            provider_kind: CapabilityProviderKind::Harness,
        });

        // 2. Match user intent
        let results = router.match_input("generate a Rust crate");
        assert!(!results.is_empty(), "Intent Router must find a match");
        assert_eq!(results[0].capability.name, "code");
        assert!(results[0].score > 0.0, "Match must have positive score");

        // 3. Create Workflow lifecycle
        let mut wf = Lifecycle::new("exe-001", "code-generation");
        assert_eq!(wf.state, LifecycleState::Initialize);

        // 4. Transition through states
        assert!(wf.transition(LifecycleState::Plan).is_ok());
        wf.step("design", 3);

        assert!(wf.transition(LifecycleState::Execute).is_ok());
        wf.step("implement", 3);

        assert!(wf.transition(LifecycleState::Verify).is_ok());
        wf.step("test", 3);

        assert!(wf.transition(LifecycleState::Complete).is_ok());
        assert!(wf.state.is_terminal());

        // 5. Lifecycle hooks fire at each transition
        let mut hooks = HookRegistry::new();
        hooks.register(Hook {
            command: "audit-log --event=exec".into(),
            event: LifecycleEvent::BeforeExecution,
            blocking: false,
            owner: "audit".into(),
            matcher: None,
            priority: 10,
        });
        let pre_hooks = hooks.hooks_for(&LifecycleEvent::BeforeExecution);
        assert_eq!(pre_hooks.len(), 1);
    }

    // ──────────────────────────────────────────────
    // Scenario 2: Permission Manifest → Policy → Audit
    // ──────────────────────────────────────────────
    #[test]
    fn scenario_2_permission_policy_audit() {
        // 1. Define permission manifest
        let perm = PermissionManifest {
            filesystem: vec![FilesystemScope {
                path: "/tmp".into(),
                read: true,
                write: true,
            }],
            shell: ShellPermissions {
                enabled: true,
                blocked: vec!["rm -rf *".into(), "sudo *".into()],
                auto_approved: vec!["git *".into(), "ls *".into()],
                ..Default::default()
            },
            ..Default::default()
        };

        // 2. Policy evaluation
        assert_eq!(
            perm.is_shell_allowed("git status"),
            PermissionVerdict::Allowed
        );
        assert!(matches!(
            perm.is_shell_allowed("sudo rm -rf /"),
            PermissionVerdict::Denied { .. }
        ));
        assert_eq!(
            perm.is_path_allowed("/tmp/test.rs", true),
            PermissionVerdict::Allowed
        );
        assert!(matches!(
            perm.is_path_allowed("/etc/passwd", false),
            PermissionVerdict::Denied { .. }
        ));

        // 3. Risk classification
        assert_eq!(
            classify(&OperationType::Shell("ls -la".into())),
            RiskLevel::Safe
        );
        assert_eq!(
            classify(&OperationType::Shell("rm -rf /".into())),
            RiskLevel::Critical
        );
        assert_eq!(
            classify(&OperationType::Filesystem {
                path: "/etc/passwd".into(),
                write: true
            }),
            RiskLevel::High
        );
        assert_eq!(
            classify(&OperationType::Git("status".into())),
            RiskLevel::Safe
        );
        assert_eq!(
            classify(&OperationType::Docker {
                image: "ubuntu".into(),
                privileged: true
            }),
            RiskLevel::High
        );

        // 4. Event bus notification
        let bus = EventBus::default_capacity();
        let mut rx = bus.subscribe();
        bus.publish(
            BusEventKind::PolicyEvaluated,
            serde_json::json!({"verdict": "denied", "policy": "no-root-access"}),
            "policy-engine",
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
        let event = rx.try_recv().expect("must receive policy event");
        assert_eq!(event.kind.label(), "policy.evaluated");
    }

    // ──────────────────────────────────────────────
    // Scenario 3: RuntimeNode → Capability → Dispatch
    // ──────────────────────────────────────────────
    #[test]
    fn scenario_3_node_capability_dispatch() {
        // 1. Create Runtime Nodes
        let mut node_reg = NodeRegistry::new();

        let mut desktop = RuntimeNode::local();
        desktop.id = "desktop-1".into();
        desktop.capabilities.gpu = true;
        desktop.capabilities.shell = true;
        node_reg.register(desktop);

        let mut phone = RuntimeNode::local();
        phone.id = "phone-1".into();
        phone.kind = NodeKind::Phone;
        phone.platform = NodePlatform::Android;
        phone.capabilities.camera = true;
        phone.capabilities.bluetooth = true;
        node_reg.register(phone);

        let mut server = RuntimeNode::local();
        server.id = "server-1".into();
        server.kind = NodeKind::Server;
        server.capabilities.execution = true;
        server.capabilities.gpu = true;
        server.transports.push(TransportKind::Grpc);
        node_reg.register(server);

        // 2. Capability negotiation — find GPU nodes
        let gpu_nodes = node_reg.with_capability("gpu");
        assert_eq!(gpu_nodes.len(), 2, "Desktop + Server have GPU");

        // 3. Find camera-capable nodes
        let camera_nodes = node_reg.with_capability("camera");
        assert_eq!(camera_nodes.len(), 1, "Only phone has camera");

        // 4. Transport capabilities
        let grpc_nodes = node_reg.by_kind(&NodeKind::Server);
        assert_eq!(grpc_nodes.len(), 1);

        // 5. Task dispatch — pick the best node for execution
        let execution_nodes = node_reg.with_capability("execution");
        assert!(execution_nodes.iter().any(|n| n.id == "server-1"));
    }

    // ──────────────────────────────────────────────
    // Scenario 4: Memory → Context Strategy → Retrieval
    // ──────────────────────────────────────────────
    #[test]
    fn scenario_4_memory_context_retrieval() {
        // 1. Hierarchical memory with multiple layers
        let mut mem = HierarchicalMemory::new();
        let gid = mem.remember(
            MemoryLayer::Global,
            "Company coding standards".into(),
            vec!["standard".into(), "rust".into()],
            1.0,
        );
        let _pid = mem.remember(
            MemoryLayer::Project,
            "Project uses axum for HTTP".into(),
            vec!["project".into(), "rust".into(), "axum".into()],
            0.8,
        );
        let _sid = mem.remember(
            MemoryLayer::Session,
            "Working on auth middleware".into(),
            vec!["session".into(), "auth".into()],
            0.5,
        );

        // 2. Search across layers
        let results = mem.search_by_tags(&["rust"], None);
        assert_eq!(results.len(), 2, "Global + Project contain 'rust' tag");

        // 3. Layer isolation
        let global_entries = mem.layer_entries(MemoryLayer::Global);
        assert_eq!(global_entries.len(), 1);

        // 4. Context strategy — manage overflow
        let mut ctx = ContextManager::new(100, ContextStrategy::DropOldest);
        ctx.push(ContextMessage {
            role: "user".into(),
            content: "This is a long conversation that might exceed the token budget if we're not careful about what we keep in memory".into(),
            timestamp: 1, pinned: false,
        });
        // Still under limit — no drops
        assert_eq!(ctx.messages_dropped, 0);

        // 5. Overflow triggers drops
        ctx.max_tokens = 5;
        ctx.push(ContextMessage {
            role: "user".into(),
            content: "overflow message".into(),
            timestamp: 2,
            pinned: false,
        });
        assert!(ctx.messages_dropped > 0);

        // 6. Verify persisting and recalling
        let recalled = mem.recall(&gid).expect("must find global memory");
        assert!(recalled.content.contains("coding standards"));
    }

    // ──────────────────────────────────────────────
    // Scenario 5: Connection Lifecycle → Worker → Task
    // ──────────────────────────────────────────────
    #[test]
    fn scenario_5_fleet_connection_lifecycle() {
        let mut fleet = ConnectionLifecycle::new();

        // 1. Worker joins
        fleet.connect(
            "worker-1",
            "node-desktop",
            Some("192.168.1.10:9000"),
            vec!["shell".into(), "filesystem".into()],
        );
        assert_eq!(fleet.count(), 1);
        assert_eq!(fleet.healthy_workers().len(), 1);

        // 2. Heartbeats
        assert!(fleet.heartbeat("worker-1"));
        assert!(!fleet.heartbeat("nonexistent"));

        // 3. Task lease acquired
        let lease = fleet.acquire_lease("task-build", "worker-1");
        assert!(lease.is_some(), "Must acquire lease for available worker");

        // 4. Cannot double-assign
        let lease2 = fleet.acquire_lease("task-build", "worker-2");
        assert!(lease2.is_none(), "Cannot acquire already-held lease");

        // 5. Renew lease
        assert!(fleet.renew_lease("task-build"), "Must renew active lease");

        // 6. Release
        fleet.release_lease("task-build", "worker-1");
        let lease3 = fleet.acquire_lease("task-build", "worker-2");
        assert!(
            lease3.is_some(),
            "After release, another worker can acquire"
        );

        // 7. Disconnect
        fleet.disconnect("worker-1");
        assert_eq!(
            fleet.worker("worker-1").unwrap().state,
            pandora_types::connection_lifecycle::ConnectionState::Disconnected
        );
    }

    // ──────────────────────────────────────────────
    // Scenario 6: Authentication → Session → Execution
    // ──────────────────────────────────────────────
    #[test]
    fn scenario_6_auth_session_execution() {
        let mut auth = AuthStore::default();

        // 1. Bootstrap
        let token = auth.create_bootstrap();
        assert!(token.len() >= 8);

        // 2. Validate bootstrap
        let client_id = auth.validate_bootstrap(&token);
        assert!(client_id.is_some());

        // 3. Cannot reuse
        assert!(auth.validate_bootstrap(&token).is_none());

        // 4. Create API key
        let api_key = auth.create_api_key("ci-bot");
        assert!(api_key.len() >= 8);

        // 5. Validate API key
        let api_client = auth.validate_api_key(&api_key);
        assert!(api_client.is_some());

        // 6. Create session
        let sid = auth.create_session("api-client-1");
        assert!(sid.len() >= 8);

        // 7. Validate and refresh
        let session = auth.validate_session(&sid);
        assert!(session.is_some());
        assert_eq!(session.unwrap().client_id, "api-client-1");
    }

    // ──────────────────────────────────────────────
    // Scenario 7: Capability Registry → Universal Registry
    // ──────────────────────────────────────────────
    #[test]
    fn scenario_7_registry_unified() {
        // 1. Capability registry — register capabilities
        let mut cap_reg = CapabilityRegistry::new();
        cap_reg.register(CapabilityEntry {
            capability: well_known::CODE_PARSE.into(),
            provider_id: "tree-sitter-gene".into(),
            provider_kind: "gene".into(),
            confidence: 1.0,
            metadata: HashMap::new(),
        });
        cap_reg.register(CapabilityEntry {
            capability: well_known::BROWSER_NAVIGATE.into(),
            provider_id: "computer-use".into(),
            provider_kind: "harness".into(),
            confidence: 0.9,
            metadata: HashMap::new(),
        });

        // 2. Discover by capability
        assert_eq!(cap_reg.providers_for(well_known::CODE_PARSE).len(), 1);
        assert_eq!(cap_reg.providers_for(well_known::BROWSER_NAVIGATE).len(), 1);
        assert_eq!(cap_reg.providers_for("nonexistent").len(), 0);

        // 3. Provider capabilities
        let caps = cap_reg.provider_capabilities("tree-sitter-gene");
        assert!(caps.contains(&well_known::CODE_PARSE));

        // 4. Universal registry — register packages
        let mut uni = InMemoryRegistry::new();
        uni.register(RegistryEntry {
            id: "pkg-1".into(),
            name: "coding-domain".into(),
            version: "1.0.0".into(),
            kind: "harness".into(),
            capabilities: vec![well_known::CODE_PARSE.into(), well_known::CODE_LINT.into()],
            health: HealthStatus::Healthy,
            signature: None,
            metadata: [("author".into(), "pandora".into())].into(),
        })
        .unwrap();
        uni.register(RegistryEntry {
            id: "pkg-2".into(),
            name: "browser-automation".into(),
            version: "0.5.0".into(),
            kind: "harness".into(),
            capabilities: vec![well_known::BROWSER_NAVIGATE.into()],
            health: HealthStatus::Healthy,
            signature: None,
            metadata: HashMap::new(),
        })
        .unwrap();

        // 5. Search by kind
        assert_eq!(uni.list_by_kind("harness").len(), 2);

        // 6. Discover by capability
        let code_pkgs = uni.discover_by_capability(well_known::CODE_PARSE);
        assert_eq!(code_pkgs.len(), 1);
        assert_eq!(code_pkgs[0].name, "coding-domain");
    }

    // ──────────────────────────────────────────────
    // Scenario 8: Event Bus → Lifecycle Hooks → Audit Trail
    // ──────────────────────────────────────────────
    #[test]
    fn scenario_8_events_hooks_audit() {
        let bus = EventBus::default_capacity();
        let mut rx = bus.subscribe();

        // 1. Execution lifecycle events
        bus.publish(
            BusEventKind::ExecutionStarted,
            serde_json::json!({"task": "build API"}),
            "runner",
        );
        bus.publish(
            BusEventKind::StageCompleted,
            serde_json::json!({"stage": "plan", "duration_ms": 150}),
            "pipeline",
        );
        bus.publish(
            BusEventKind::HarnessDispatched,
            serde_json::json!({"harness": "coding-domain"}),
            "council",
        );
        bus.publish(
            BusEventKind::ProviderSelected,
            serde_json::json!({"provider": "ollama", "model": "llama3.2"}),
            "resolver",
        );
        bus.publish(
            BusEventKind::ExecutionCompleted,
            serde_json::json!({"task": "build API", "success": true}),
            "runner",
        );

        std::thread::sleep(std::time::Duration::from_millis(20));

        // 2. All events received
        let mut count = 0;
        while rx.try_recv().is_ok() {
            count += 1;
        }
        assert_eq!(count, 5, "Must receive all 5 published events");

        // 3. Lifecycle hooks registered and dispatched
        let mut hooks = HookRegistry::new();
        let mut hook_count = 0;

        for event in &[
            LifecycleEvent::BeforeExecution,
            LifecycleEvent::AfterExecution,
            LifecycleEvent::BeforeInstall,
            LifecycleEvent::AfterInstall,
        ] {
            hooks.register(Hook {
                command: format!("notify --event={}", event.label()),
                event: event.clone(),
                blocking: false,
                owner: "audit-harness".into(),
                matcher: None,
                priority: 5,
            });
            hook_count += 1;
        }

        assert_eq!(hooks.count(), hook_count);
        assert_eq!(hooks.hooks_for(&LifecycleEvent::BeforeExecution).len(), 1);
        assert_eq!(hooks.hooks_for(&LifecycleEvent::AfterInstall).len(), 1);
    }
}
