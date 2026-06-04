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
pub struct BenchmarkSignal {

    pub benchmark_id:
        String,

    pub domain:
        String,

    pub governance_stability:
        f64,

    pub replay_integrity:
        f64,

    pub mutation_survivability:
        f64,

    pub autonomy_stability:
        f64,

    pub epistemic_coherence:
        f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct BenchmarkDirective {

    pub benchmark_id:
        String,

    pub constitutional_grade:
        String,

    pub governance_certified:
        bool,

    pub replay_certified:
        bool,

    pub mutation_promotion_allowed:
        bool,

    pub autonomy_expansion_allowed:
        bool,

    pub survivability_score:
        f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct BenchmarkState {

    pub constitutional_reliability:
        f64,

    pub replay_stability:
        f64,

    pub governance_survivability:
        f64,

    pub sovereign_benchmark_stable:
        bool,

    pub directives:
        Vec<
            BenchmarkDirective
        >,
}

pub struct ConstitutionalReliabilityBenchmarkEngine;

impl ConstitutionalReliabilityBenchmarkEngine {

    pub fn benchmark(

        signals:
            &[BenchmarkSignal],
    )
        -> BenchmarkState
    {

        let mut directives =
            Vec::new();

        let mut reliability =
            0.0;

        let mut replay =
            0.0;

        let mut governance =
            0.0;

        for signal
            in signals
        {

            println!(
                "[BENCHMARK] benchmark={}",
                signal.benchmark_id
            );

            let survivability_score =
                (
                    signal
                        .governance_stability
                        * 0.25
                )
                + (
                    signal
                        .replay_integrity
                        * 0.20
                )
                + (
                    signal
                        .mutation_survivability
                        * 0.20
                )
                + (
                    signal
                        .autonomy_stability
                        * 0.20
                )
                + (
                    signal
                        .epistemic_coherence
                        * 0.15
                );

            let constitutional_grade =
                if survivability_score
                    > 0.92
                {

                    "sovereign"

                } else if survivability_score
                    > 0.82
                {

                    "constitutional"

                } else if survivability_score
                    > 0.72
                {

                    "restricted"

                } else {

                    "quarantined"
                };

            let governance_certified =
                signal
                    .governance_stability
                        > 0.84;

            let replay_certified =
                signal
                    .replay_integrity
                        > 0.82;

            let mutation_promotion_allowed =
                signal
                    .mutation_survivability
                        > 0.86;

            let autonomy_expansion_allowed =
                signal
                    .autonomy_stability
                        > 0.88;

            directives.push(

                BenchmarkDirective {

                    benchmark_id:
                        signal
                            .benchmark_id
                            .clone(),

                    constitutional_grade:
                        constitutional_grade
                            .into(),

                    governance_certified,

                    replay_certified,

                    mutation_promotion_allowed,

                    autonomy_expansion_allowed,

                    survivability_score,
                }
            );

            reliability +=
                survivability_score;

            replay +=
                signal
                    .replay_integrity;

            governance +=
                signal
                    .governance_stability;
        }

        let count =
            signals.len() as f64;

        let constitutional_reliability =
            reliability / count;

        let replay_stability =
            replay / count;

        let governance_survivability =
            governance / count;

        let sovereign_benchmark_stable =
            constitutional_reliability
                > 0.84
            &&
            replay_stability
                > 0.83
            &&
            governance_survivability
                > 0.84;

        BenchmarkState {

            constitutional_reliability,

            replay_stability,

            governance_survivability,

            sovereign_benchmark_stable,

            directives,
        }
    }
}
