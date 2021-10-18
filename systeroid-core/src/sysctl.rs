use crate::docs::{Documentation, SysctlSection};
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
    /// Documentation of the kernel parameter.
    pub documentation: Option<Documentation>,
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
                documentation: None,
            });
        }
        Ok(Self { parameters })
    }

    /// Updates the description of the kernel parameters based on the parsed documentation.
    ///
    /// [`parsed documentation`]: Documentation
    pub fn update_docs(&mut self, docs: Vec<Documentation>) {
        for param in self
            .parameters
            .iter_mut()
            .filter(|p| p.description.is_none() || p.description.as_deref() == Some("[N/A]"))
        {
            if let Some(documentation) =
                docs.iter().find(
                    |doc| match param.name.split('.').collect::<Vec<&str>>().last() {
                        Some(absolute_name) => {
                            absolute_name.len() > 2
                                && doc.name.contains(absolute_name)
                                && doc.section == param.section
                        }
                        _ => false,
                    },
                )
            {
                param.description = Some(documentation.description.to_owned());
                param.documentation = Some(documentation.clone());
            }
        }
    }
}
