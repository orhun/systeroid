/// Documentation of the Linux kernel.
#[derive(Clone, Debug)]
pub struct Documentation<'a> {
    /// Kernel parameters.
    pub parameters: Vec<Parameter<'a>>,
}

impl<'a> Documentation<'a> {
    /// Constructs a new instance.
    pub fn new(parameters: Vec<Parameter<'a>>) -> Self {
        Self { parameters }
    }
}

/// Representation of a kernel parameter.
#[derive(Clone, Debug)]
pub struct Parameter<'a> {
    /// Name of the kernel parameter.
    pub name: &'a str,
    /// Description of the kernel parameter.
    pub description: &'a str,
}

impl<'a> Parameter<'a> {
    /// Constructs a new instance.
    pub fn new(name: &'a str, description: &'a str) -> Self {
        Self { name, description }
    }
}
