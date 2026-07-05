// ponytail: Research Domain Harness — bundles research workflow genes.

use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};

#[derive(Debug)]
pub struct ResearchDomainHarness {
    manifest: HarnessManifest,
}

impl ResearchDomainHarness {
    pub fn new() -> Self {
        Self {
            manifest: HarnessManifestBuilder::default()
                .id("research-domain")
                .name("Research")
                .version("0.1.0")
                .author("pandora")
                .kind(HarnessKind::Domain)
                .description("Research and analysis domain — search, browse, summarize, extract")
                .capability("research")
                .capability("web-scraping")
                .capability("data-extraction")
                .slash_command("/search", "Search web and local sources")
                .slash_command("/extract", "Extract structured data")
                .slash_command("/summarize", "Summarize content")
                .build().unwrap(),
        }
    }
}

impl Harness for ResearchDomainHarness {
    fn manifest(&self) -> &HarnessManifest { &self.manifest }
}
