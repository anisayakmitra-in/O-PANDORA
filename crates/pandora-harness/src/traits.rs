use crate::roles::HarnessRole;

/// Common interface implemented by every harness in Pandora.
///
/// Constitutional Harnesses and Meta Harnesses both implement this trait.
pub trait Harness {
    /// Unique identifier.
    fn id(&self) -> &str;

    /// Human-readable name.
    fn name(&self) -> &str;

    /// Version.
    fn version(&self) -> &str;

    /// Role fulfilled by this harness.
    fn role(&self) -> HarnessRole;

    /// Initialize the harness.
    fn initialize(&mut self) -> Result<(), String>;

    /// Shutdown gracefully.
    fn shutdown(&mut self) -> Result<(), String>;
}
