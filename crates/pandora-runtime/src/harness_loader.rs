use crate::harness_manifest::HarnessManifest;

pub struct HarnessLoader;

impl HarnessLoader {

    pub fn discover() -> Vec<HarnessManifest> {

        vec![
            HarnessManifest {

                name:
                    String::from(
                        "example-harness"
                    ),

                version:
                    String::from(
                        "0.1.0"
                    ),

                author:
                    String::from(
                        "Pandora"
                    ),

                description:
                    String::from(
                        "Example meta-harness"
                    ),
            }
        ]
    }
}
