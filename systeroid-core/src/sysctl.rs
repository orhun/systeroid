use crate::docs::SysctlSection;
use crate::error::Result;
use std::result::Result as StdResult;
use sysctl::{CtlIter, Sysctl as SysctlImpl};

/// Representation of a kernel parameter.
pub struct Parameter {
    /// Name of the kernel parameter.
    pub name: String,
    /// Value of the kernel parameter.
    pub value: String,
    /// Description of the kernel parameter
    pub description: Option<String>,
    /// Section of the kernel parameter.
    pub section: SysctlSection,
}

/// Sysctl wrapper for managing the kernel parameters.
pub struct Sysctl {
    /// Available kernel parameters.
    pub parameters: Vec<Parameter>,
}

impl Sysctl {
    /// Constructs a new instance by fetching the available kernel parameters.
    pub fn init() -> Result<Self> {
        let mut parameters = Vec::new();
        for ctl in CtlIter::root().filter_map(StdResult::ok) {
            parameters.push(Parameter {
                name: ctl.name()?,
                value: ctl.value_string()?,
                description: ctl.description().ok(),
                section: SysctlSection::from(ctl.name()?),
            });
        }
        Ok(Self { parameters })
    }
}
