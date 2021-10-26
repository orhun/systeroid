use crate::sysctl::Section;

/// Documentation of a kernel parameter.
#[derive(Clone, Debug)]
pub struct Documentation {
    /// Name of the kernel parameter.
    pub name: String,
    /// Description of the kernel parameter.
    pub description: String,
    /// Section of the kernel parameter.
    pub section: Section,
}

impl Documentation {
    /// Constructs a new instance.
    pub fn new(name: String, description: String, section: Section) -> Self {
        Self {
            name,
            description,
            section,
        }
    }
}
