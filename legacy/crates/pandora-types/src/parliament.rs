//! Parliament — constitutional service registry.
//! Parliamentary services own the runtime. Every service implements
//! ParliamentService and runs during the governance cycle.

use std::time::Duration;

use std::collections::HashMap;

/// What every parliamentary service must implement.
pub trait ParliamentService: Send + Sync {
    fn name(&self) -> &str;
    /// Called before execution begins. Returns a verdict that can block or modify execution.
    fn pre_flight(&self, _session: &str, _task: &str) -> Result<ParliamentVerdict, crate::PandoraError> {
        Ok(ParliamentVerdict::Allow)
    }
    /// Called after execution completes. Returns a verdict that can record decisions or request follow-up.
    fn post_flight(&self, _session: &str, _outcome: &str) -> Result<ParliamentVerdict, crate::PandoraError> {
        Ok(ParliamentVerdict::Allow)
    }
}

/// The verdict of a parliamentary service.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ParliamentVerdict {
    /// Execution may proceed.
    Allow,
    /// Execution is blocked. Contains reason.
    Deny { reason: String },
    /// Execution requires approval before proceeding.
    RequireApproval {
        /// Who can approve: User, Parliament, or a specific role.
        who: ApprovalScope,
        /// How long the approval request is valid.
        expires: Option<Duration>,
    },
    /// The plan should be modified. Contains the amended execution plan.
    Modify {
        /// The amended execution plan (serialized as JSON for transport).
        amended_plan: serde_json::Value,
    },
    /// Escalate to a higher authority (e.g., human operator, external system).
    Escalate { to: Vec<String> },
}

/// Scope of who can grant approval.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ApprovalScope {
    /// Any user with access to the CLI.
    User,
    /// Requires parliamentary quorum.
    Parliament,
    /// Requires a specific role (encoded as string).
    Role(String),
}

/// The Parliament — a registry of constitutional services.
#[derive(Default)]
pub struct Parliament {
    services: HashMap<String, Box<dyn ParliamentService>>,
}

impl Parliament {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register(&mut self, service: Box<dyn ParliamentService>) {
        self.services.insert(service.name().to_string(), service);
    }

    /// Run all pre-flight checks and return the aggregate verdict.
    /// If any service returns Deny, the aggregate is Deny.
    /// If any service returns RequireApproval and none return Deny, the aggregate is RequireApproval.
    /// If any service returns Modify, the last Modify is kept (services should coordinate).
    /// Escalate is logged but doesn't change the aggregate unless combined with Deny.
    pub fn pre_flight(&self, session: &str, task: &str) -> ParliamentVerdict {
        let mut final_verdict = ParliamentVerdict::Allow;
        let mut saw_modify = false;
        let mut modify_verdict = ParliamentVerdict::Allow;

        for service in self.services.values() {
            match service.pre_flight(session, task) {
                Ok(ParliamentVerdict::Allow) => {}
                Ok(ParliamentVerdict::Deny { reason }) => {
                    return ParliamentVerdict::Deny { reason };
                }
                Ok(ParliamentVerdict::RequireApproval { who, expires }) => {
                    if matches!(final_verdict, ParliamentVerdict::Allow) {
                        final_verdict = ParliamentVerdict::RequireApproval { who, expires };
                    }
                }
                Ok(ParliamentVerdict::Modify { amended_plan }) => {
                    saw_modify = true;
                    modify_verdict = ParliamentVerdict::Modify { amended_plan };
                }
                Ok(ParliamentVerdict::Escalate { to }) => {
                    eprintln!("[PARLIAMENT] Service {} escalated to: {:?}", service.name(), to);
                    // Escalate doesn't change verdict unless combined with Deny
                }
                Err(e) => {
                    return ParliamentVerdict::Deny {
                        reason: format!("Service {} error: {}", service.name(), e),
                    };
                }
            }
        }

        if saw_modify {
            modify_verdict
        } else {
            final_verdict
        }
    }

    /// Run all post-flight checks and return the aggregate verdict.
    pub fn post_flight(&self, session: &str, outcome: &str) -> ParliamentVerdict {
        let mut final_verdict = ParliamentVerdict::Allow;

        for service in self.services.values() {
            match service.post_flight(session, outcome) {
                Ok(ParliamentVerdict::Allow) => {}
                Ok(ParliamentVerdict::Deny { reason }) => {
                    return ParliamentVerdict::Deny { reason };
                }
                Ok(ParliamentVerdict::RequireApproval { who, expires }) => {
                    if matches!(final_verdict, ParliamentVerdict::Allow) {
                        final_verdict = ParliamentVerdict::RequireApproval { who, expires };
                    }
                }
                Ok(ParliamentVerdict::Modify { .. }) => {
                    // Post-flight modifications are advisory only
                }
                Ok(ParliamentVerdict::Escalate { to }) => {
                    eprintln!("[PARLIAMENT] Service {} escalated to: {:?}", service.name(), to);
                }
                Err(e) => {
                    return ParliamentVerdict::Deny {
                        reason: format!("Service {} error: {}", service.name(), e),
                    };
                }
            }
        }

        final_verdict
    }

    pub fn service_count(&self) -> usize {
        self.services.len()
    }
}

/// Built-in parliamentary service: monitors governance policies.
pub struct GovernanceService;

impl ParliamentService for GovernanceService {
    fn name(&self) -> &str {
        "governance"
    }
    fn pre_flight(&self, _session: &str, task: &str) -> Result<ParliamentVerdict, crate::PandoraError> {
        if task.is_empty() {
            return Ok(ParliamentVerdict::Deny {
                reason: "empty task".to_string(),
            });
        }
        Ok(ParliamentVerdict::Allow)
    }
    fn post_flight(&self, _session: &str, outcome: &str) -> Result<ParliamentVerdict, crate::PandoraError> {
        if outcome.is_empty() {
            return Ok(ParliamentVerdict::Deny {
                reason: "empty outcome — possible pipeline failure".to_string(),
            });
        }
        Ok(ParliamentVerdict::Allow)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::PandoraError;
    use serde_json::json;

    // ── Helper: create a service that returns a fixed verdict ──

    struct StaticService {
        name: String,
        pre_reply: ParliamentVerdict,
        post_reply: ParliamentVerdict,
    }

    impl StaticService {
        fn new(name: &str, pre: ParliamentVerdict, post: ParliamentVerdict) -> Self {
            Self { name: name.into(), pre_reply: pre, post_reply: post }
        }
    }

    impl ParliamentService for StaticService {
        fn name(&self) -> &str { &self.name }
        fn pre_flight(&self, _: &str, _: &str) -> Result<ParliamentVerdict, PandoraError> {
            Ok(self.pre_reply.clone())
        }
        fn post_flight(&self, _: &str, _: &str) -> Result<ParliamentVerdict, PandoraError> {
            Ok(self.post_reply.clone())
        }
    }

    // ── Helper: service that returns Err ──

    struct ErrorService(String);

    impl ParliamentService for ErrorService {
        fn name(&self) -> &str { &self.0 }
        fn pre_flight(&self, _: &str, _: &str) -> Result<ParliamentVerdict, PandoraError> {
            Err(PandoraError::Governance(format!("{} failed", self.0)))
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  GovernanceService
    // ═══════════════════════════════════════════════════════════

    #[test]
    fn governance_denies_empty_task() {
        let g = GovernanceService;
        let v = g.pre_flight("s1", "").unwrap();
        assert_eq!(v, ParliamentVerdict::Deny { reason: "empty task".into() });
    }

    #[test]
    fn governance_allows_nonempty_task() {
        let g = GovernanceService;
        let v = g.pre_flight("s1", "build a thing").unwrap();
        assert_eq!(v, ParliamentVerdict::Allow);
    }

    #[test]
    fn governance_denies_empty_outcome_postflight() {
        let g = GovernanceService;
        let v = g.post_flight("s1", "").unwrap();
        assert_eq!(v, ParliamentVerdict::Deny { reason: "empty outcome — possible pipeline failure".into() });
    }

    #[test]
    fn governance_allows_nonempty_outcome_postflight() {
        let g = GovernanceService;
        let v = g.post_flight("s1", "task completed").unwrap();
        assert_eq!(v, ParliamentVerdict::Allow);
    }

    // ═══════════════════════════════════════════════════════════
    //  Parliament verdict aggregation — pre_flight
    // ═══════════════════════════════════════════════════════════

    #[test]
    fn empty_parliament_returns_allow() {
        let p = Parliament::new();
        assert_eq!(p.pre_flight("s1", "task"), ParliamentVerdict::Allow);
        assert_eq!(p.service_count(), 0);
    }

    #[test]
    fn single_allow_returns_allow() {
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("a", ParliamentVerdict::Allow, ParliamentVerdict::Allow)));
        assert_eq!(p.pre_flight("s1", "task"), ParliamentVerdict::Allow);
    }

    #[test]
    fn deny_blocks_immediately() {
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("a", ParliamentVerdict::Deny { reason: "no".into() }, ParliamentVerdict::Allow)));
        assert_eq!(p.pre_flight("s1", "task"), ParliamentVerdict::Deny { reason: "no".into() });
    }

    #[test]
    fn deny_overrides_approval() {
        let mut p = Parliament::new();
        // First service requests approval, second denies — deny wins
        p.register(Box::new(StaticService::new("approver",
            ParliamentVerdict::RequireApproval { who: ApprovalScope::User, expires: None },
            ParliamentVerdict::Allow)));
        p.register(Box::new(StaticService::new("denier",
            ParliamentVerdict::Deny { reason: "overridden".into() },
            ParliamentVerdict::Allow)));
        assert_eq!(p.pre_flight("s1", "task"), ParliamentVerdict::Deny { reason: "overridden".into() });
    }

    #[test]
    fn deny_overrides_modify() {
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("modifier",
            ParliamentVerdict::Modify { amended_plan: json!({"x": 1}) },
            ParliamentVerdict::Allow)));
        p.register(Box::new(StaticService::new("denier",
            ParliamentVerdict::Deny { reason: "no mods".into() },
            ParliamentVerdict::Allow)));
        assert_eq!(p.pre_flight("s1", "task"), ParliamentVerdict::Deny { reason: "no mods".into() });
    }

    #[test]
    fn require_approval_surfaces() {
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("gate",
            ParliamentVerdict::RequireApproval {
                who: ApprovalScope::Role("auditor".into()),
                expires: Some(Duration::from_secs(300)),
            },
            ParliamentVerdict::Allow)));
        assert_eq!(
            p.pre_flight("s1", "task"),
            ParliamentVerdict::RequireApproval {
                who: ApprovalScope::Role("auditor".into()),
                expires: Some(Duration::from_secs(300)),
            }
        );
    }

    #[test]
    fn modify_replaces_plan() {
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("planner",
            ParliamentVerdict::Modify { amended_plan: json!({"steps": ["a", "b"]}) },
            ParliamentVerdict::Allow)));
        assert_eq!(
            p.pre_flight("s1", "task"),
            ParliamentVerdict::Modify { amended_plan: json!({"steps": ["a", "b"]}) }
        );
    }

    #[test]
    fn escalate_is_logged_does_not_block() {
        // Escalate alone should not change the aggregate — still Allow
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("alerter",
            ParliamentVerdict::Escalate { to: vec!["human-ops".into()] },
            ParliamentVerdict::Allow)));
        assert_eq!(p.pre_flight("s1", "task"), ParliamentVerdict::Allow);
    }

    #[test]
    fn modify_is_returned_when_service_requests_it() {
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("planner",
            ParliamentVerdict::Modify { amended_plan: json!({"steps": 3}) },
            ParliamentVerdict::Allow)));
        let result = p.pre_flight("s1", "task");
        // A Modify verdict is returned (not Allow, not Deny)
        assert!(matches!(result, ParliamentVerdict::Modify { .. }));
    }

    #[test]
    fn modify_with_escalate_returns_modify() {
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("planner",
            ParliamentVerdict::Modify { amended_plan: json!({"x": 1}) },
            ParliamentVerdict::Allow)));
        p.register(Box::new(StaticService::new("alerter",
            ParliamentVerdict::Escalate { to: vec!["ops".into()] },
            ParliamentVerdict::Allow)));
        // Modify takes precedence over Escalate
        let result = p.pre_flight("s1", "task");
        assert!(matches!(result, ParliamentVerdict::Modify { .. }));
    }

    #[test]
    fn service_error_returns_deny() {
        let mut p = Parliament::new();
        p.register(Box::new(ErrorService("broken".into())));
        match p.pre_flight("s1", "task") {
            ParliamentVerdict::Deny { reason } => {
                assert!(reason.contains("broken"));
                assert!(reason.contains("error"));
            }
            other => panic!("expected Deny, got {:?}", other),
        }
    }

    #[test]
    fn error_overrides_approval() {
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("gate",
            ParliamentVerdict::RequireApproval { who: ApprovalScope::User, expires: None },
            ParliamentVerdict::Allow)));
        p.register(Box::new(ErrorService("broken".into())));
        match p.pre_flight("s1", "task") {
            ParliamentVerdict::Deny { .. } => {} // expected
            other => panic!("expected Deny, got {:?}", other),
        }
    }

    // ═══════════════════════════════════════════════════════════
    //  Parliament verdict aggregation — post_flight
    // ═══════════════════════════════════════════════════════════

    #[test]
    fn post_flight_empty_returns_allow() {
        let p = Parliament::new();
        assert_eq!(p.post_flight("s1", "outcome"), ParliamentVerdict::Allow);
    }

    #[test]
    fn post_flight_deny_blocks() {
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("auditor",
            ParliamentVerdict::Allow,
            ParliamentVerdict::Deny { reason: "bad outcome".into() })));
        assert_eq!(p.post_flight("s1", "outcome"), ParliamentVerdict::Deny { reason: "bad outcome".into() });
    }

    #[test]
    fn post_flight_modify_is_advisory_only() {
        // Post-flight Modify should not change the aggregate — it's advisory
        let mut p = Parliament::new();
        p.register(Box::new(StaticService::new("advisor",
            ParliamentVerdict::Allow,
            ParliamentVerdict::Modify { amended_plan: json!({"fix": true}) })));
        assert_eq!(p.post_flight("s1", "outcome"), ParliamentVerdict::Allow);
    }

    // ═══════════════════════════════════════════════════════════
    //  ApprovalScope
    // ═══════════════════════════════════════════════════════════

    #[test]
    fn approval_scope_variants() {
        let u = ApprovalScope::User;
        let p = ApprovalScope::Parliament;
        let r = ApprovalScope::Role("admin".into());
        assert_eq!(format!("{:?}", u), "User");
        assert_eq!(format!("{:?}", p), "Parliament");
        assert_eq!(format!("{:?}", r), "Role(\"admin\")");
    }

    // ═══════════════════════════════════════════════════════════
    //  ParliamentVerdict serialization roundtrip
    // ═══════════════════════════════════════════════════════════

    #[test]
    fn verdict_roundtrip_allow() {
        let v = ParliamentVerdict::Allow;
        let s = serde_json::to_string(&v).unwrap();
        let back: ParliamentVerdict = serde_json::from_str(&s).unwrap();
        assert_eq!(back, ParliamentVerdict::Allow);
    }

    #[test]
    fn verdict_roundtrip_deny() {
        let v = ParliamentVerdict::Deny { reason: "test".into() };
        let s = serde_json::to_string(&v).unwrap();
        let back: ParliamentVerdict = serde_json::from_str(&s).unwrap();
        assert_eq!(back, ParliamentVerdict::Deny { reason: "test".into() });
    }

    #[test]
    fn verdict_roundtrip_require_approval() {
        let v = ParliamentVerdict::RequireApproval {
            who: ApprovalScope::Role("auditor".into()),
            expires: None,
        };
        let s = serde_json::to_string(&v).unwrap();
        let back: ParliamentVerdict = serde_json::from_str(&s).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn verdict_roundtrip_modify() {
        let v = ParliamentVerdict::Modify { amended_plan: json!({"x": [1, 2, 3]}) };
        let s = serde_json::to_string(&v).unwrap();
        let back: ParliamentVerdict = serde_json::from_str(&s).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn verdict_roundtrip_escalate() {
        let v = ParliamentVerdict::Escalate { to: vec!["ops".into(), "legal".into()] };
        let s = serde_json::to_string(&v).unwrap();
        let back: ParliamentVerdict = serde_json::from_str(&s).unwrap();
        assert_eq!(back, v);
    }
}
