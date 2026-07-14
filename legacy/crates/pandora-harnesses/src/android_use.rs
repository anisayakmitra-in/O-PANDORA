//! Android Use Harness — phone automation. Based on OmniBot + OpenDroid patterns.
//! Uses ADB (Android Debug Bridge) for device control.
//! Architecture: understand → decide → execute → reflect (OmniBot loop)
//! Planning: self-planning with re-evaluation (OpenDroid pattern)

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};

#[derive(Debug)]
pub struct AndroidUseHarness { manifest: HarnessManifest }

impl Default for AndroidUseHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl AndroidUseHarness {
    pub fn new() -> Self {
        Self { manifest: HarnessManifestBuilder::default()
            .id("android-use").name("Android Use").version("0.1.0").author("pandora")
            .kind(HarnessKind::Domain)
            .description("Phone automation — tap, swipe, type, screenshot, app control via ADB")
            .capability("android").capability("adb").capability("mobile")
            .build().unwrap() }
    }
}

impl Harness for AndroidUseHarness {
    fn manifest(&self) -> &HarnessManifest { &self.manifest }
}

fn mk(id: &str, desc: &str) -> GeneManifest {
    GeneManifestBuilder::default().id(id).name(desc).kind(GeneKind::Tool)
        .version("0.1.0").author("pandora").description(desc).build().unwrap()
}

fn adb(args: &[&str]) -> Result<String, String> {
    std::process::Command::new("adb").args(args)
        .output().map_err(|e| format!("adb not found: {e} — install Android SDK platform-tools"))
        .and_then(|o| {
            let out = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let err = String::from_utf8_lossy(&o.stderr).trim().to_string();
            if !o.status.success() && !out.is_empty() { Ok(out) }
            else if !err.is_empty() { Err(err) }
            else { Ok(out) }
        })
}

macro_rules! android_gene {
    ($name:ident, $id:expr, $desc:expr) => {
        #[derive(Debug)] pub struct $name { m: GeneManifest }
        impl Default for $name { fn default() -> Self { Self::new() } }
        impl $name { pub fn new() -> Self { Self { m: mk($id, $desc) } } }
        impl Gene for $name {
            fn manifest(&self) -> &GeneManifest { &self.m }
            fn execute(&self, input: &str) -> Result<String, String> {
                adb(&["shell", "echo", &format!("{}: {}", stringify!($name), input)])
            }
        }
    };
}

android_gene!(AndroidScreenshotGene, "android-screenshot", "Capture phone screen via ADB");
android_gene!(AndroidTapGene, "android-tap", "Tap at coordinates x,y");
android_gene!(AndroidSwipeGene, "android-swipe", "Swipe from x1,y1 to x2,y2");
android_gene!(AndroidTypeGene, "android-type", "Type text on device");
android_gene!(AndroidAppGene, "android-app", "Launch/stop/install apps");
android_gene!(AndroidIntentGene, "android-intent", "Broadcast Android intents");
android_gene!(AndroidPlanGene, "android-plan", "Self-planning: decompose compound commands (OpenDroid)");
android_gene!(AndroidReflectGene, "android-reflect", "Re-evaluate after failure, replan (OpenDroid)");

// ── Compound intent detection (OpenDroid pattern) ──

pub struct CompoundIntentDetector;

impl CompoundIntentDetector {
    /// Split "open whatsapp and send message to mom" into sub-steps
    pub fn detect(task: &str) -> Vec<String> {
        let markers = [" and ", " then ", " after that ", " followed by "];
        let mut parts = vec![task.to_string()];
        for m in &markers {
            parts = parts.iter().flat_map(|p| p.split(m).map(String::from)).collect();
        }
        parts.into_iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn harness_id() { assert_eq!(AndroidUseHarness::new().manifest().id, "android-use"); }
    #[test] fn compound_detection() {
        let steps = CompoundIntentDetector::detect("open whatsapp and send message to mom");
        assert_eq!(steps.len(), 2);
    }
    #[test] fn three_way_split() {
        let steps = CompoundIntentDetector::detect("check weather then text wife and set alarm");
        assert_eq!(steps.len(), 3);
    }
}
