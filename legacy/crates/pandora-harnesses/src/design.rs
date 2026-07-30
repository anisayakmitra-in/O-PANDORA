//! Design Domain Harness — UI/UX, animation, design review, and brand.

use pandora_types::gene::Gene;
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};

#[derive(Debug)]
pub struct DesignDomainHarness {
    manifest: HarnessManifest,
}
impl DesignDomainHarness {
    pub fn new() -> Self {
        Self { manifest: HarnessManifestBuilder::default().id("design-domain").name("Design").version(env!("CARGO_PKG_VERSION")).author("pandora").kind(HarnessKind::Domain).description("UI/UX design — brand identity, color theory, typography, motion, accessibility, design review, UI patterns").capability("design-review").capability("brand-identity").capability("color-theory").capability("typography").capability("motion-design").capability("ui-patterns").capability("accessibility").slash_command("/design.review", "Review a design: /design.review [url|file]").slash_command("/design.brand", "Suggest brand identity elements").slash_command("/design.color", "Suggest color palette from base color").slash_command("/design.typography", "Suggest typography pairings").slash_command("/design.motion", "Suggest animation patterns").slash_command("/design.ui", "Suggest UI component patterns").slash_command("/design.a11y", "Check accessibility compliance")
            .owned_gene("design-review")
            .owned_gene("brand-identity")
            .owned_gene("ui-patterns")
            .owned_gene("motion-design")
            .owned_gene("color-theory")
            .owned_gene("typography-expert")
            .owned_gene("accessibility-review")
            .build().unwrap() }
    }
}
impl Default for DesignDomainHarness {
    fn default() -> Self {
        Self::new()
    }
}
impl Harness for DesignDomainHarness {
    fn manifest(&self) -> &HarnessManifest {
        &self.manifest
    }
}

pub fn preloaded_genes() -> Vec<Box<dyn Gene>> {
    vec![
        Box::new(crate::design_genes::DesignReviewGene::new()),
        Box::new(crate::design_genes::BrandIdentityGene::new()),
        Box::new(crate::design_genes::UiPatternsGene::new()),
        Box::new(crate::design_genes::MotionDesignGene::new()),
        Box::new(crate::design_genes::ColorTheoryGene::new()),
        Box::new(crate::design_genes::TypographyExpertGene::new()),
        Box::new(crate::design_genes::AccessibilityReviewGene::new()),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;
    use pandora_types::gene::Gene;

    #[test]
    fn design_owns_declared_genes() {
        let manifest = DesignDomainHarness::new().manifest().clone();
        assert_eq!(manifest.owned_genes.len(), 7);
        assert_eq!(
            crate::design_genes::DesignReviewGene::new()
                .manifest()
                .owner_harness
                .as_deref(),
            Some("design-domain")
        );
    }
}
