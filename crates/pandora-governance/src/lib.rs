#[derive(Debug)]
pub enum PermissionTier {

    T1,
    T2,
    T3,
}

#[derive(Debug)]
pub struct ActionRequest {

    pub source:
        String,

    pub action:
        String,

    pub tier:
        PermissionTier,
}

#[derive(Debug)]
pub enum GovernanceDecision {

    Approved,

    Denied(String),
}

pub struct Rahu;

impl Rahu {

    pub fn propose(
        source: &str,
        action: &str,
        tier: PermissionTier,
    ) -> ActionRequest {

        ActionRequest {

            source:
                source.to_string(),

            action:
                action.to_string(),

            tier,
        }
    }
}

pub struct Ketu;

impl Ketu {

    pub fn validate(
        request: &ActionRequest,
    ) -> GovernanceDecision {

        match request.tier {

            PermissionTier::T1 => {

                GovernanceDecision::Approved
            }

            PermissionTier::T2 => {

                if request.action
                    .contains("shell")
                {

                    GovernanceDecision::Denied(
                        "Shell access denied under T2 policy"
                            .to_string()
                    )

                } else {

                    GovernanceDecision::Approved
                }
            }

            PermissionTier::T3 => {

                GovernanceDecision::Denied(
                    "T3 actions require sovereign approval"
                        .to_string()
                )
            }
        }
    }
}
