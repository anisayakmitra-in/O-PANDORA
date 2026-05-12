use serde::{
    Deserialize,
    Serialize,
};

use std::time::Duration;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum ExecutionTier {

    Tier1Isolated,

    Tier2Governed {

        network_access:
            bool,
    },

    Tier3Host,

    Tier4Autonomous {

        max_loop_duration:
            Option<Duration>,

        opt_in_receipt:
            String,
    },

    Tier5Unbounded {

        operator_session_id:
            String,
    },
}

impl ExecutionTier {

    pub fn is_host_execution(
        &self,
    ) -> bool {

        matches!(
            self,

            Self::Tier3Host
            |
            Self::Tier4Autonomous {
                ..
            }
            |
            Self::Tier5Unbounded {
                ..
            }
        )
    }

    pub fn requires_sync_consent(
        &self,
    ) -> bool {

        matches!(
            self,
            Self::Tier3Host
        )
    }

    pub fn privilege_level(
        &self,
    ) -> u8 {

        match self {

            Self::Tier1Isolated => 1,

            Self::Tier2Governed {
                ..
            } => 2,

            Self::Tier3Host => 3,

            Self::Tier4Autonomous {
                ..
            } => 4,

            Self::Tier5Unbounded {
                ..
            } => 5,
        }
    }
}
