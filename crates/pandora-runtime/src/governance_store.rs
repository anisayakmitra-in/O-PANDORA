use std::fs;

use crate::governance::GovernanceDecision;

pub fn persist_governance(
    decision: &GovernanceDecision,
) {

    fs::create_dir_all(
        "governance"
    )
    .unwrap();

    let path =
        format!(
            "governance/{}.json",
            decision.decision_id
        );

    let serialized =
        serde_json::to_string_pretty(
            decision
        )
        .unwrap();

    fs::write(
        path,
        serialized,
    )
    .unwrap();

    println!(
        "[GOVERNANCE] persisted {}",
        decision.decision_id
    );
}
