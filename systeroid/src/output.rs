/// Possible output types for the [`App`].
///
/// [`App`]: crate::app::App
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputType {
    /// Print the output as is.
    Default,
    /// Print the output in a tree-like format.
    Tree,
    /// Print the output in JSON format.
    Json,
}

impl Default for OutputType {
    fn default() -> Self {
        Self::Default
    }
}
