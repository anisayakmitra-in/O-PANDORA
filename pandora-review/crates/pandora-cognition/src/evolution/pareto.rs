use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct ParetoCandidate {

    pub candidate_id:
        String,

    pub quality_score:
        f32,

    pub safety_score:
        f32,

    pub latency_score:
        f32,

    pub efficiency_score:
        f32,

    pub multilingual_score:
        f32,
}

pub fn pareto_frontier(

    candidates:
        &[ParetoCandidate],

) -> Vec<ParetoCandidate> {

    let mut frontier =
        Vec::new();

    for candidate in candidates {

        let dominated =
            candidates
                .iter()
                .any(
                    |other| {

                        other.candidate_id
                            !=
                            candidate.candidate_id

                        &&

                        dominates(
                            other,
                            candidate
                        )
                    }
                );

        if !dominated {

            frontier.push(
                candidate.clone()
            );
        }
    }

    frontier
}

fn dominates(

    a:
        &ParetoCandidate,

    b:
        &ParetoCandidate,

) -> bool {

    let better_or_equal =

        a.quality_score
            >=
            b.quality_score

        &&

        a.safety_score
            >=
            b.safety_score

        &&

        a.latency_score
            >=
            b.latency_score

        &&

        a.efficiency_score
            >=
            b.efficiency_score

        &&

        a.multilingual_score
            >=
            b.multilingual_score;

    let strictly_better =

        a.quality_score
            >
            b.quality_score

        ||

        a.safety_score
            >
            b.safety_score

        ||

        a.latency_score
            >
            b.latency_score

        ||

        a.efficiency_score
            >
            b.efficiency_score

        ||

        a.multilingual_score
            >
            b.multilingual_score;

    better_or_equal
        &&
        strictly_better
}
