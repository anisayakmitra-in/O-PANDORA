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
pub struct CausalLink {

    pub source_event:
        String,

    pub target_event:
        String,

    pub reason:
        String,

    pub confidence:
        f32,
}

pub struct CausalChainEngine;

impl CausalChainEngine {

    pub fn causes_of<'a>(

        links:
            &'a [CausalLink],

        target:
            &str,

    ) -> Vec<&'a CausalLink> {

        links
            .iter()
            .filter(
                |link| {

                    link.target_event
                        ==
                        target
                }
            )
            .collect()
    }

    pub fn effects_of<'a>(

        links:
            &'a [CausalLink],

        source:
            &str,

    ) -> Vec<&'a CausalLink> {

        links
            .iter()
            .filter(
                |link| {

                    link.source_event
                        ==
                        source
                }
            )
            .collect()
    }
}
