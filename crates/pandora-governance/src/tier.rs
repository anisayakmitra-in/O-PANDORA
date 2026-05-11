use std::time::Duration;

use pandora_sandbox::config::SandboxConfig;

#[derive(Debug, Clone)]
pub enum ExecutionTier {

    IsolatedSandbox,

    GovernedElevated(
        SandboxConfig
    ),

    HostUnrestricted,

    AutonomousOperator {

        max_loop_duration:
            Option<Duration>,

        opt_in_receipt:
            String,
    },

    UnboundedExecution {

        operator_session_id:
            String,
    },
}

impl ExecutionTier {

    pub fn is_host_execution(
        &self
    ) -> bool {

        matches!(
            self,

            Self::HostUnrestricted

            |

            Self::AutonomousOperator { .. }

            |

            Self::UnboundedExecution { .. }
        )
    }

    pub fn privilege_level(
        &self
    ) -> u8 {

        match self {

            Self::IsolatedSandbox => 1,

            Self::GovernedElevated(_) => 2,

            Self::HostUnrestricted => 3,

            Self::AutonomousOperator { .. } => 4,

            Self::UnboundedExecution { .. } => 5,
        }
    }
}
