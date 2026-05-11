use uuid::Uuid;

use tokio_util::sync::CancellationToken;

use std::time::{
    Instant,
    SystemTime,
};

use tracing::{
    info_span,
    Span,
};

use crate::tier::ExecutionTier;

use crate::error::GovernanceError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextOrigin {

    Operator(String),

    SystemTimer,

    Agent(String),

    Subagent(String),
}

impl std::fmt::Display for ContextOrigin {

    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {

        match self {

            Self::Operator(id) => {
                write!(
                    f,
                    "operator:{}",
                    id
                )
            }

            Self::SystemTimer => {
                write!(
                    f,
                    "system:scheduler"
                )
            }

            Self::Agent(id) => {
                write!(
                    f,
                    "agent:{}",
                    id
                )
            }

            Self::Subagent(id) => {
                write!(
                    f,
                    "subagent:{}",
                    id
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {

    pub trace_id:
        Uuid,

    pub parent_trace_id:
        Option<Uuid>,

    pub session_id:
        String,

    pub task_id:
        String,

    pub gene_id:
        Option<String>,

    pub tier:
        ExecutionTier,

    pub spawned_by:
        ContextOrigin,

    pub created_at:
        SystemTime,

    pub deadline:
        Option<Instant>,

    pub cancel_token:
        CancellationToken,
}

impl ExecutionContext {

    pub fn new_root(

        session_id:
            String,

        task_id:
            String,

        tier:
            ExecutionTier,

        spawned_by:
            ContextOrigin,

        global_kill_switch:
            CancellationToken,

    ) -> Self {

        Self {

            trace_id:
                Uuid::new_v4(),

            parent_trace_id:
                None,

            session_id,

            task_id,

            gene_id:
                None,

            tier,

            spawned_by,

            created_at:
                SystemTime::now(),

            deadline:
                None,

            cancel_token:
                global_kill_switch
                    .child_token(),
        }
    }

    pub fn with_gene_id(
        mut self,
        gene_id: String,
    ) -> Self {

        self.gene_id =
            Some(gene_id);

        self
    }

    pub fn with_timeout(
        mut self,
        duration: std::time::Duration,
    ) -> Self {

        self.deadline =
            Some(
                Instant::now() + duration
            );

        self
    }

    pub fn spawn_child(

        &self,

        sub_task_id:
            String,

        requested_tier:
            ExecutionTier,

        subagent_id:
            String,

    ) -> Result<Self, GovernanceError> {

        if requested_tier
            .privilege_level()

            >

            self.tier
                .privilege_level()
        {

            return Err(
                GovernanceError::PrivilegeEscalationAttempt
            );
        }

        let child_token =
            self.cancel_token
                .child_token();

        Ok(

            Self {

                trace_id:
                    Uuid::new_v4(),

                parent_trace_id:
                    Some(self.trace_id),

                session_id:
                    self.session_id.clone(),

                task_id:
                    sub_task_id,

                gene_id:
                    self.gene_id.clone(),

                tier:
                    requested_tier,

                spawned_by:
                    ContextOrigin::Subagent(
                        subagent_id
                    ),

                created_at:
                    SystemTime::now(),

                deadline:
                    self.deadline,

                cancel_token:
                    child_token,
            }
        )
    }

    pub fn create_span(
        &self
    ) -> Span {

        info_span!(
            "pandora_execution",

            trace_id =
                %self.trace_id,

            parent_trace_id =
                ?self.parent_trace_id,

            session_id =
                %self.session_id,

            task_id =
                %self.task_id,

            gene_id =
                ?self.gene_id,

            tier =
                ?self.tier,
        )
    }

    pub fn is_cancelled(
        &self
    ) -> bool {

        self.cancel_token
            .is_cancelled()
    }
}
