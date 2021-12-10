/// Possible ways of displaying the kernel variables.
#[derive(Clone, Debug)]
pub enum DisplayType {
    /// Print the kernel variable name along with its value.
    Default,
    /// Print only the name of the variable.
    Name,
    /// Print only the value of the variable.
    Value,
    /// Print only the value of the variable without new line.
    Binary,
}

impl Default for DisplayType {
    fn default() -> Self {
        Self::Default
    }
}
