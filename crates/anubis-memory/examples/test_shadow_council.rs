use anubis_memory::shadow_council::{CouncilVote, ShadowCouncil, VoteDecision};

fn main() {
    let votes = vec![
        CouncilVote {
            member_id: String::from("security-harness"),

            proposal_id: String::from("mutation-1"),

            decision: VoteDecision::Approve,

            confidence: 0.91,
        },
        CouncilVote {
            member_id: String::from("governance-harness"),

            proposal_id: String::from("mutation-1"),

            decision: VoteDecision::Reject,

            confidence: 0.84,
        },
        CouncilVote {
            member_id: String::from("reasoning-harness"),

            proposal_id: String::from("mutation-1"),

            decision: VoteDecision::Approve,

            confidence: 0.96,
        },
    ];

    let consensus = ShadowCouncil::consensus(&votes);

    println!("{:#?}", consensus);

    let confidence = ShadowCouncil::weighted_confidence(&votes);

    println!("Council confidence: {}", confidence);
}
