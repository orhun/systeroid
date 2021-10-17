use sysctl::{CtlIter, Sysctl as SysctlImpl};

/// Representation of a kernel parameter.
pub struct Parameter {
    /// Name of the kernel parameter.
    pub name: String,
    /// Value of the kernel parameter.
    pub value: String,
    /// Description of the kernel parameter
    pub description: Option<String>,
}

/// Sysctl wrapper for managing the kernel parameters.
pub struct Sysctl {
    /// Available kernel parameters.
    pub parameters: Vec<Parameter>,
}

impl Sysctl {
    /// Constructs a new instance by fetching the available kernel parameters.
    pub fn init() -> Self {
        Self {
            parameters: CtlIter::root()
                .filter_map(Result::ok)
                .filter_map(|ctl| {
                    Some(Parameter {
                        name: ctl.name().ok()?,
                        value: ctl.value_string().ok()?,
                        description: ctl.description().ok(),
                    })
                })
                .collect(),
        }
    }
}
