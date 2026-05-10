use crate::capability::{
    CapabilityDecision,
    CapabilityRequest,
};

use crate::gene::GeneManifest;

use crate::harness::MetaHarness;

pub struct PanoptesHarness;

impl MetaHarness
    for PanoptesHarness
{

    fn name(
        &self
    ) -> String {

        String::from(
            "PANOPTES"
        )
    }

    fn authorize(
        &self,
        gene: &GeneManifest,
        request: &CapabilityRequest,
    )
        -> CapabilityDecision
    {

        println!(
            "[PANOPTES] evaluating capability: {}",
            request.capability
        );

        println!(
            "[PANOPTES] requester: {}",
            gene.name
        );

        if request.capability
            == "shell.execute"
        {

           return CapabilityDecision::Denied;
    }

    CapabilityDecision::Approved
}

    fn validate(
        &self,
        gene: &GeneManifest,
    ) -> bool {

        println!(
            "[PANOPTES] validating gene: {}",
            gene.name
        );

        true
    }

    fn govern(
        &self,
        gene: &GeneManifest,
    ) {

        println!(
            "[PANOPTES] governing {}",
            gene.name
        );
    }

    fn evolve(
        &self,
        gene: &GeneManifest,
    ) {

        println!(
            "[PANOPTES] supervising evolution for {}",
            gene.name
        );
    }
}
