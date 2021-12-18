/// Possible ways of displaying the kernel parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DisplayType {
    /// Print the kernel parameter name along with its value.
    Default,
    /// Print only the name of the parameter.
    Name,
    /// Print only the value of the parameter.
    Value,
    /// Print only the value of the parameter without new line.
    Binary,
}

impl Default for DisplayType {
    fn default() -> Self {
        Self::Default
    }
}
