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
pub struct CouncilPersona {

    pub persona:
        String,

    pub domain:
        String,

    pub aggression:
        f64,

    pub caution:
        f64,

    pub survivability_bias:
        f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CouncilVerdict {

    pub persona:
        String,

    pub recommendation:
        String,

    pub confidence:
        f64,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct StrategicConsensus {

    pub objective:
        String,

    pub consensus:
        String,

    pub stability_score:
        f64,

    pub verdicts:
        Vec<
            CouncilVerdict
        >,
}

pub struct ShadowCouncilEngine;

impl ShadowCouncilEngine {

    pub fn deliberate(

        objective:
            &str,

        council:
            &[CouncilPersona],
    )
        -> StrategicConsensus
    {

        println!(
            "[SHADOW-COUNCIL] objective={}",
            objective
        );

        let mut verdicts =
            Vec::new();

        let mut cumulative =
            0.0;

        for member
            in council
        {

            println!(
                "[SHADOW-COUNCIL] deliberating {}",
                member.persona
            );

            let confidence =
                (
                    member.survivability_bias
                        * 0.45
                )
                + (
                    member.caution
                        * 0.35
                )
                + (
                    member.aggression
                        * 0.20
                );

            let recommendation =
                if confidence > 0.88 {

                    "approve strategic execution"

                } else if confidence > 0.72 {

                    "approve with oversight"

                } else {

                    "delay execution for review"
                };

            verdicts.push(

                CouncilVerdict {

                    persona:
                        member
                            .persona
                            .clone(),

                    recommendation:
                        recommendation
                            .into(),

                    confidence,
                }
            );

            cumulative +=
                confidence;
        }

        let stability =
            cumulative
                / council.len() as f64;

        let consensus =
            if stability > 0.88 {

                "stable-consensus"

            } else if stability > 0.74 {

                "conditional-consensus"

            } else {

                "unstable-consensus"
            };

        StrategicConsensus {

            objective:
                objective
                    .into(),

            consensus:
                consensus
                    .into(),

            stability_score:
                stability,

            verdicts,
        }
    }
}
