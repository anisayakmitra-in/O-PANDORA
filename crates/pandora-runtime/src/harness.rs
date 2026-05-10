use crate::capability::{
    CapabilityDecision,
    CapabilityRequest,
};

use crate::gene::GeneManifest;

pub trait MetaHarness {

    fn name(
        &self
    ) -> String;

    fn authorize(
       &self,
       gene: &GeneManifest,
       request: &CapabilityRequest,
    )
       -> CapabilityDecision;

    fn validate(
        &self,
        gene: &GeneManifest,
    ) -> bool;

    fn govern(
        &self,
        gene: &GeneManifest,
    );

    fn evolve(
        &self,
        gene: &GeneManifest,
    );
}
