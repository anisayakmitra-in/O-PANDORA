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