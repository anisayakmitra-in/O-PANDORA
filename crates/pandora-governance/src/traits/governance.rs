use std::sync::Arc;

use async_trait::async_trait;

use crate::error::Result;
use crate::types::{
    DecisionId, DecisionKind, GovernanceContextMap, GovernanceDecision, GovernanceId,
    GovernanceTier, PrincipalId,
};

/// The single, canonical governance trait.
#[async_trait]
pub trait Governance: Send + Sync {
    fn governance_id(&self) -> GovernanceId;
    fn name(&self) -> &str;
    fn tier(&self) -> GovernanceTier;

    async fn evaluate(
        &self,
        principal: PrincipalId,
        action: String,
        context: GovernanceContextMap,
    ) -> Result<GovernanceDecision>;

    async fn lookup(&self, decision_id: &DecisionId) -> Result<Option<GovernanceDecision>>;
}

impl dyn Governance {
    pub fn covers(&self, tier: GovernanceTier) -> bool {
        tier.privilege_level() <= self.tier().privilege_level()
    }

    pub async fn evaluate_kind(
        self: Arc<Self>,
        principal: PrincipalId,
        action: String,
        context: GovernanceContextMap,
    ) -> Result<DecisionKind> {
        Ok(self.evaluate(principal, action, context).await?.kind)
    }
}
