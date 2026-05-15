use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum CognitionBranchType {

    General,

    Coding,

    Planning,

    Governance,

    Research,

    Multilingual,

    AutonomousExecution,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CognitionBranch {

    pub branch_id:
        String,

    pub branch_type:
        CognitionBranchType,

    pub active_candidates:
        Vec<String>,

    pub objectives:
        Vec<String>,
}

pub struct BranchManager;

impl BranchManager {

    pub fn create_branch(

        branch_id:
            impl Into<String>,

        branch_type:
            CognitionBranchType,

        objectives:
            Vec<String>,

    ) -> CognitionBranch {

        CognitionBranch {

            branch_id:
                branch_id.into(),

            branch_type,

            active_candidates:
                Vec::new(),

            objectives,
        }
    }

    pub fn assign_candidate(

        branch:
            &mut CognitionBranch,

        candidate_id:
            impl Into<String>,

    ) {

        branch
            .active_candidates
            .push(
                candidate_id.into()
            );
    }

    pub fn supports_multilingual(

        branch:
            &CognitionBranch,

    ) -> bool {

        matches!(

            branch.branch_type,

            CognitionBranchType
                ::Multilingual

            |

            CognitionBranchType
                ::General
        )
    }
}
