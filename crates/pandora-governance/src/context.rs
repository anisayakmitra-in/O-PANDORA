use std::time::Duration;

use tokio_util::sync::CancellationToken;

use uuid::Uuid;

use crate::error::GovernanceError;

use crate::tier::ExecutionTier;

#[derive(
    Debug,
    Clone,
)]
pub struct ExecutionContext {

    pub trace_id:
        Uuid,

    pub parent_trace_id:
        Option<Uuid>,

    pub session_id:
        String,

    pub task_id:
        Option<Uuid>,

    pub gene_id:
        Option<String>,

    pub spawned_by:
        Option<Uuid>,

    pub tier:
        ExecutionTier,

    pub cancel_token:
        CancellationToken,

    pub timeout:
        Option<Duration>,
}

impl ExecutionContext {

    pub fn with_gene_id(

        mut self,

        gene_id: impl Into<String>,

    ) -> Self {

        self.gene_id =
            Some(
                gene_id.into()
            );

        self
    }

    pub fn with_timeout(

        mut self,

        timeout: Duration,

    ) -> Self {

        self.timeout =
            Some(timeout);

        self
    }

    pub fn is_cancelled(
        &self,
    ) -> bool {

        self.cancel_token
            .is_cancelled()
    }

    pub fn spawn_child(

        &self,

        child_tier:
            ExecutionTier,

    ) -> Result<
        Self,
        GovernanceError,
    > {

        if child_tier
            .privilege_level()
            >
            self
                .tier
                .privilege_level()
        {

            return Err(
                GovernanceError
                    ::PrivilegeEscalationAttempt
            );
        }

        Ok(
            Self {

                trace_id:
                    Uuid::new_v4(),

                parent_trace_id:
                    Some(
                        self.trace_id
                    ),

                session_id:
                    self
                        .session_id
                        .clone(),

                task_id:
                    None,

                gene_id:
                    self
                        .gene_id
                        .clone(),

                spawned_by:
                    Some(
                        self.trace_id
                    ),

                tier:
                    child_tier,

                cancel_token:
                    self
                        .cancel_token
                        .child_token(),

                timeout:
                    self.timeout,
            }
        )
    }
}
