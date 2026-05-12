use chrono::{
    DateTime,
    Utc,
};

use serde::{
    Deserialize,
    Serialize,
};

use uuid::Uuid;

use crate::budget::{
    ExecutionBudget,
    RetryPolicy,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum ExecutionTier {

    Tier1Isolated,

    Tier2Governed,

    Tier3Host,

    Tier4Autonomous,

    Tier5Unbounded,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
)]
pub enum TaskStatus {

    Pending,

    Running,

    Completed,

    Failed,

    Cancelled,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum Recurrence {

    OneShot,

    IntervalSeconds(
        u64
    ),

    Cron(
        String
    ),
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum TaskPayload {

    ExecuteCommand {

        command: Vec<String>,
    },

    SpawnAgent {

        gene_id: String,
    },

    EvaluateGene {

        gene: String,
    },

    MemoryConsolidation,

    ProviderInference,

    GovernanceAudit,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct Task {

    pub id: Uuid,

    pub created_at:
        DateTime<Utc>,

    pub next_run:
        DateTime<Utc>,

    pub status:
        TaskStatus,

    pub tier:
        ExecutionTier,

    pub recurrence:
        Recurrence,

    pub retry_policy:
        RetryPolicy,

    pub budget:
        ExecutionBudget,

    pub attempts: u32,

    pub invocations: u64,

    pub payload:
        TaskPayload,
}

impl Task {

    pub fn new(
        tier: ExecutionTier,
        payload: TaskPayload,
    ) -> Self {

        Self {

            id:
                Uuid::new_v4(),

            created_at:
                Utc::now(),

            next_run:
                Utc::now(),

            status:
                TaskStatus::Pending,

            tier,

            recurrence:
                Recurrence::OneShot,

            retry_policy:
                RetryPolicy::default(),

            budget:
                ExecutionBudget::default(),

            attempts:
                0,

            invocations:
                0,

            payload,
        }
    }

    pub fn with_delay(
        mut self,
        seconds: u64,
    ) -> Self {

        self.next_run =
            Utc::now()
            + chrono::Duration::seconds(
                seconds as i64
            );

        self
    }
}
