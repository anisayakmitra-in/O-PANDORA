use crate::branch::CognitionBranch;

pub struct BranchRollback;

impl BranchRollback {
    pub fn prune(branches: &mut Vec<CognitionBranch>) {
        branches.retain(|branch| !branch.speculative);
    }
}
