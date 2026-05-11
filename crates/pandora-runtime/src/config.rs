#[derive(
    Debug,
    Clone,
)]
pub struct RuntimeConfig {

    pub allow_shell: bool,
}

impl RuntimeConfig {

    pub fn load()
        -> Self
    {

        Self {

            allow_shell: true,
        }
    }
}
