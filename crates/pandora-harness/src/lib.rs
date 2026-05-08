pub trait Harness {

    fn name(&self)
    -> &str;

    fn system_prompt(&self)
    -> String;

    fn execute(
        &self,
        input: &str,
    ) -> String;
}

pub struct CodingHarness;

impl Harness
for CodingHarness {

    fn name(&self)
    -> &str {

        "coding"
    }

    fn system_prompt(
        &self,
    ) -> String {

        "You are a Rust coding harness."
            .to_string()
    }

    fn execute(
        &self,
        input: &str,
    ) -> String {

        format!(
            "CODING HARNESS EXECUTED:\n{}",
            input
        )
    }
}

pub struct ResearchHarness;

impl Harness
for ResearchHarness {

    fn name(&self)
    -> &str {

        "research"
    }

    fn system_prompt(
        &self,
    ) -> String {

        "You are a research harness."
            .to_string()
    }

    fn execute(
        &self,
        input: &str,
    ) -> String {

        format!(
            "RESEARCH HARNESS EXECUTED:\n{}",
            input
        )
    }
}
