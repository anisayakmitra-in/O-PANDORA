pub mod ollama;

pub trait Provider {

    fn name(
        &self
    )
        -> &str;

    fn infer(
        &self,
        model: &str,
        prompt: &str,
    )
        -> String;
}

