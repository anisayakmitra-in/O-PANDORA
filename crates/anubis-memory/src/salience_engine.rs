use crate::salience::SalienceScore;

pub fn calculate_salience(

    score:
        &mut SalienceScore,
)
{

    score.final_score =

        (
            score.replay_frequency
            * 0.4
        )

        +

        (
            score.governance_importance
            * 0.3
        )

        +

        (
            score.graph_centrality
            * 0.3
        );
}
