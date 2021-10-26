use crate::docs::Documentation;
use crate::error::Result;
use std::fmt::{self, Display, Formatter};
use std::path::Path;
use std::result::Result as StdResult;
use sysctl::{CtlFlags, CtlIter, Sysctl as SysctlImpl};

/// Sections of the sysctl documentation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Section {
    /// Documentation for `/proc/sys/abi/*`
    Abi,
    /// Documentation for `/proc/sys/fs/*`
    Fs,
    /// Documentation for `/proc/sys/kernel/*`
    Kernel,
    /// Documentation for `/proc/sys/net/*`
    Net,
    /// Documentation for `/proc/sys/sunrpc/*`
    Sunrpc,
    /// Documentation for `/proc/sys/user/*`
    User,
    /// Documentation for `/proc/sys/vm/*`
    Vm,
    /// Unknown.
    Unknown,
}

impl From<String> for Section {
    fn from(value: String) -> Self {
        for section in Self::variants() {
            if value.starts_with(&format!("{}.", section)) {
                return *section;
            }
        }
        Self::Unknown
    }
}

impl<'a> From<&'a Path> for Section {
    fn from(value: &'a Path) -> Self {
        for section in Self::variants() {
            if value.file_stem().map(|v| v.to_str()).flatten() == Some(&section.to_string()) {
                return *section;
            }
        }
        Self::Unknown
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Section {
    /// Returns the variants.
    pub fn variants() -> &'static [Section] {
        &[
            Self::Abi,
            Self::Fs,
            Self::Kernel,
            Self::Net,
            Self::Sunrpc,
            Self::User,
            Self::Vm,
        ]
    }
}

/// Representation of a kernel parameter.
#[derive(Debug)]
pub struct Parameter {
    /// Name of the kernel parameter.
    pub name: String,
    /// Value of the kernel parameter.
    pub value: String,
    /// Description of the kernel parameter
    pub description: Option<String>,
    /// Section of the kernel parameter.
    pub section: Section,
    /// Documentation of the kernel parameter.
    pub documentation: Option<Documentation>,
}

/// Sysctl wrapper for managing the kernel parameters.
#[derive(Debug)]
pub struct Sysctl {
    /// Available kernel parameters.
    pub parameters: Vec<Parameter>,
}

impl Sysctl {
    /// Constructs a new instance by fetching the available kernel parameters.
    pub fn init() -> Result<Self> {
        let mut parameters = Vec::new();
        for ctl in CtlIter::root().filter_map(StdResult::ok).filter(|ctl| {
            ctl.flags()
                .map(|flags| !flags.contains(CtlFlags::SKIP))
                .unwrap_or(false)
        }) {
            parameters.push(Parameter {
                name: ctl.name()?,
                value: ctl.value_string()?,
                description: ctl.description().ok(),
                section: Section::from(ctl.name()?),
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
