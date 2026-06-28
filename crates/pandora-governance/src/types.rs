use crate::error::{GovernanceError, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GovernanceTier {
    Tier1Isolated,
    Tier2Governed,
    Tier3Host,
    Tier4Autonomous,
    Tier5Unbounded,
}

impl Default for GovernanceTier {
    fn default() -> Self {
        GovernanceTier::Tier1Isolated
    }
}

impl GovernanceTier {
    pub fn privilege_level(self) -> u8 {
        match self {
            GovernanceTier::Tier1Isolated => 1,
            GovernanceTier::Tier2Governed => 2,
            GovernanceTier::Tier3Host => 3,
            GovernanceTier::Tier4Autonomous => 4,
            GovernanceTier::Tier5Unbounded => 5,
        }
    }
    pub fn is_host_execution(self) -> bool {
        matches!(
            self,
            GovernanceTier::Tier3Host
                | GovernanceTier::Tier4Autonomous
                | GovernanceTier::Tier5Unbounded
        )
    }
    pub fn requires_sync_consent(self) -> bool {
        matches!(self, GovernanceTier::Tier3Host)
    }
}

pub type GovernanceContextMap = BTreeMap<String, String>;
pub type DecisionId = String;
pub type ApprovalId = String;
pub type PolicyId = String;
pub type GovernanceId = String;
pub type ApprovalFlowId = String;
pub type PrincipalId = String;
pub type AuditId = String;
pub type ProvenanceId = String;
pub type ProvenancePayload = BTreeMap<String, String>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TrustKind {
    Direct,
    Delegated,
    Reputation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRecord {
    pub principal: PrincipalId,
    pub kind: TrustKind,
    pub score: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default = "default_epoch")]
    pub refreshed_at: SystemTime,
}

fn default_epoch() -> SystemTime {
    SystemTime::UNIX_EPOCH
}

impl TrustRecord {
    pub fn new(principal: impl Into<PrincipalId>, kind: TrustKind, score: f32) -> Self {
        Self {
            principal: principal.into(),
            kind,
            score,
            source: None,
            refreshed_at: SystemTime::now(),
        }
    }
    pub fn clears(&self, threshold: f32) -> bool {
        self.score >= threshold
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceEntry {
    pub id: ProvenanceId,
    pub producer: String,
    pub action: String,
    pub timestamp: SystemTime,
    #[serde(default)]
    pub payload: ProvenancePayload,
}

impl ProvenanceEntry {
    pub fn new(
        id: impl Into<ProvenanceId>,
        producer: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            producer: producer.into(),
            action: action.into(),
            timestamp: SystemTime::now(),
            payload: BTreeMap::new(),
        }
    }
    pub fn with_payload(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.payload.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub id: PrincipalId,
    pub name: String,
    #[serde(default)]
    pub attributes: GovernanceContextMap,
}

impl Principal {
    pub fn new(id: impl Into<PrincipalId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            attributes: BTreeMap::new(),
        }
    }
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DecisionKind {
    Approved,
    Rejected,
    Quarantined,
    Escalated,
    RequiresReview,
}

impl DecisionKind {
    pub fn is_allowed(self) -> bool {
        matches!(self, DecisionKind::Approved)
    }
    pub fn is_blocked(self) -> bool {
        matches!(
            self,
            DecisionKind::Rejected | DecisionKind::Quarantined | DecisionKind::RequiresReview
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceDecision {
    pub id: DecisionId,
    pub kind: DecisionKind,
    pub reason: String,
    pub safety_score: f32,
    pub tier: GovernanceTier,
    pub principal: PrincipalId,
    pub timestamp: SystemTime,
}

impl GovernanceDecision {
    pub fn approved(
        principal: impl Into<PrincipalId>,
        tier: GovernanceTier,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: generate_decision_id(),
            kind: DecisionKind::Approved,
            reason: reason.into(),
            safety_score: 1.0,
            tier,
            principal: principal.into(),
            timestamp: SystemTime::now(),
        }
    }
    pub fn rejected(
        principal: impl Into<PrincipalId>,
        tier: GovernanceTier,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: generate_decision_id(),
            kind: DecisionKind::Rejected,
            reason: reason.into(),
            safety_score: 0.0,
            tier,
            principal: principal.into(),
            timestamp: SystemTime::now(),
        }
    }
    pub fn is_allowed(&self) -> bool {
        self.kind.is_allowed()
    }
}

pub fn generate_decision_id() -> DecisionId {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    thread_local! {
        static COUNTER: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    }
    let n = COUNTER.with(|c| {
        let v = c.get().wrapping_add(1);
        c.set(v);
        v
    });
    let mut s = String::from("dec_");
    s.push_str(&format!("{:x}", ms));
    s.push_str("_");
    s.push_str(&format!("{:x}", n));
    s
}

pub fn decision_to_result(decision: &GovernanceDecision) -> Result<()> {
    if decision.is_allowed() {
        return Ok(());
    }
    Err(match decision.kind {
        DecisionKind::Approved => unreachable!(),
        DecisionKind::Rejected => GovernanceError::PolicyViolation(decision.reason.clone()),
        DecisionKind::Quarantined => {
            let mut s = String::from("quarantined: ");
            s.push_str(&decision.reason);
            GovernanceError::ApprovalDenied(s)
        }
        DecisionKind::Escalated => {
            let mut s = String::from("escalated: ");
            s.push_str(&decision.reason);
            GovernanceError::ApprovalDenied(s)
        }
        DecisionKind::RequiresReview => {
            let mut s = String::from("requires review: ");
            s.push_str(&decision.reason);
            GovernanceError::ApprovalDenied(s)
        }
    })
}
