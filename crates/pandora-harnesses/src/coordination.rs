// ponytail: Coordination Meta Harness — mesh/harness-to-harness communication.

use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};

#[derive(Debug)]
pub struct CoordinationMetaHarness {
    manifest: HarnessManifest,
}

impl CoordinationMetaHarness {
    pub fn new() -> Self {
        Self {
            manifest: HarnessManifestBuilder::default()
                .id("coordination-meta")
                .name("Coordination")
                .version("0.1.0")
                .author("pandora")
                .kind(HarnessKind::Meta)
                .description(
                    "Inter-harness coordination — delegation, routing, workflow orchestration",
                )
                .capability("coordination")
                .capability("orchestration")
                .slash_command("/delegate", "Delegate a task to another harness")
                .slash_command("/route", "Route a request to the appropriate harness")
                .slash_command("/orchestrate", "Run a multi-step orchestration")
                .build()
                .unwrap(),
        }
    }
}

impl Harness for CoordinationMetaHarness {
    fn manifest(&self) -> &HarnessManifest {
        &self.manifest
    }
}
