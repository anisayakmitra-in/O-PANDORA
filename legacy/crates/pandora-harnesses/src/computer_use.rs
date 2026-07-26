//! Computer Use Harness — desktop automation. Based on GhostOS architecture.
use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};

#[derive(Debug)]
pub struct ComputerUseHarness {
    manifest: HarnessManifest,
}

impl Default for ComputerUseHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputerUseHarness {
    pub fn new() -> Self {
        Self {
            manifest: HarnessManifestBuilder::default()
                .id("computer-use")
                .name("Computer Use")
                .version("0.2.0")
                .author("pandora")
                .kind(HarnessKind::Domain)
                .description("Desktop automation — click, type, screenshot, find elements")
                .capability("screenshot")
                .capability("click")
                .capability("typing")
                .capability("context")
                .build()
                .unwrap(),
        }
    }
}

impl Harness for ComputerUseHarness {
    fn manifest(&self) -> &HarnessManifest {
        &self.manifest
    }
}

fn platform() -> &'static str {
    std::env::consts::OS
}

fn mk(id: &str, desc: &str) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id)
        .name(desc)
        .kind(GeneKind::Tool)
        .version("0.2.0")
        .author("pandora")
        .description(desc)
        .build()
        .unwrap()
}

macro_rules! ghost_gene {
    ($name:ident, $id:expr, $desc:expr) => {
        #[derive(Debug)]
        pub struct $name {
            m: GeneManifest,
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $name {
            pub fn new() -> Self {
                Self { m: mk($id, $desc) }
            }
        }
        impl Gene for $name {
            fn manifest(&self) -> &GeneManifest {
                &self.m
            }
            fn execute(&self, _input: &str) -> Result<String, String> {
                let os = platform();
                let t = match os {
                    "linux" => "xdotool/imagemagick",
                    "macos" => "osascript/screencapture",
                    _ => "not available",
                };
                Err(format!("{} on {}: install {}", stringify!($name), os, t))
            }
        }
    };
}

ghost_gene!(
    GhostContextGene,
    "ghost-context",
    "Focused app, window, URL, interactive elements"
);
ghost_gene!(
    GhostClickGene,
    "ghost-click",
    "Click coordinates or element. AX-native first, synthetic fallback"
);
ghost_gene!(
    GhostTypeGene,
    "ghost-type",
    "Type text. Focus the target field first with ghost_click"
);
ghost_gene!(
    GhostScreenshotGene,
    "ghost-screenshot",
    "Capture screen to PNG file"
);
ghost_gene!(
    GhostFindGene,
    "ghost-find",
    "Find elements by query, role, or ID"
);
ghost_gene!(GhostReadGene, "ghost-read", "Read text from screen region");
ghost_gene!(
    GhostScrollGene,
    "ghost-scroll",
    "Scroll in a window or element"
);
ghost_gene!(GhostHotkeyGene, "ghost-hotkey", "Press keyboard shortcut");
ghost_gene!(
    GhostInspectGene,
    "ghost-inspect",
    "Full metadata about one element"
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn harness_id() {
        assert_eq!(ComputerUseHarness::new().manifest().id, "computer-use");
    }
    #[test]
    fn gene_id() {
        assert_eq!(GhostClickGene::new().manifest().id, "ghost-click");
    }
}
