use crate::base::Harness;

pub struct CodingHarness;

impl Harness for CodingHarness {
    fn name(&self) -> &str {
        "coding"
    }
}
