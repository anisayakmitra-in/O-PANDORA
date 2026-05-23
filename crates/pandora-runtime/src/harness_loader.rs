use libloading::{Library, Symbol};

use serde::{Deserialize, Serialize};

use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessManifest {
    pub name: String,

    pub version: String,

    pub author: String,

    pub description: String,

    pub library_path: String,
}

pub struct LoadedHarness {
    pub library: Library,
}

pub struct HarnessLoader;

impl HarnessLoader {
    pub fn discover() -> Vec<HarnessManifest> {
        println!("[HARNESS LOADER] discovering harnesses");

        vec![HarnessManifest {
            name: "dynamic-harness".into(),

            version: "0.1.0".into(),

            author: "Pandora".into(),

            description: "Dynamic cognition harness".into(),

            library_path: "target/release/libdynamic_harness.so".into(),
        }]
    }

    pub unsafe fn load(path: impl AsRef<Path>) -> Option<LoadedHarness> {
        let library = Library::new(path.as_ref()).ok()?;

        println!("[HARNESS LOADER] loaded dynamic harness");

        Some(LoadedHarness { library })
    }

    pub unsafe fn execute(harness: &LoadedHarness) {
        type ExecuteFn = unsafe fn();

        let function: Symbol<ExecuteFn> = harness.library.get(b"execute").unwrap();

        function();

        println!("[HARNESS LOADER] executed harness");
    }
}
