//! Design Domain Harness — UI/UX, animation, design review, and brand.

use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};

#[derive(Debug)]
pub struct DesignDomainHarness {
    manifest: HarnessManifest,
}
impl DesignDomainHarness {
    pub fn new() -> Self {
        Self { manifest: HarnessManifestBuilder::default().id("design-domain").name("Design").version("0.2.0").author("pandora").kind(HarnessKind::Domain).description("UI/UX design — brand identity, color theory, typography, motion, accessibility, design review, UI patterns").capability("design-review").capability("brand-identity").capability("color-theory").capability("typography").capability("motion-design").capability("ui-patterns").capability("accessibility").slash_command("/design.review", "Review a design: /design.review [url|file]").slash_command("/design.brand", "Suggest brand identity elements").slash_command("/design.color", "Suggest color palette from base color").slash_command("/design.typography", "Suggest typography pairings").slash_command("/design.motion", "Suggest animation patterns").slash_command("/design.ui", "Suggest UI component patterns").slash_command("/design.a11y", "Check accessibility compliance").build().unwrap() }
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
