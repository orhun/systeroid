use crate::error::Result;
use std::fmt::{self, Display, Formatter};
use std::path::Path;
use std::result::Result as StdResult;
use sysctl::{CtlFlags, CtlIter, Sysctl as SysctlImpl};
use systeroid_parser::document::Document;

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
        Self::Net
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
    /// Parsed document about the kernel parameter.
    pub document: Option<Document>,
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
                document: None,
            });
        }
        Ok(Self { parameters })
    }

    /// Updates the description of the kernel parameters based on the [`parsed document`].
    ///
    /// [`parsed document`]: Document
    pub fn update_docs(&mut self, documents: Vec<Document>) {
        for param in self
            .parameters
            .iter_mut()
            .filter(|p| p.description.is_none() || p.description.as_deref() == Some("[N/A]"))
        {
            for document in documents
                .iter()
                .filter(|document| Section::from(document.path.as_path()) == param.section)
            {
                if let Some(paragraph) = document.paragraphs.iter().find(|paragraph| {
                    match param.name.split('.').collect::<Vec<&str>>().last() {
                        Some(absolute_name) => {
                            absolute_name.len() > 2 && paragraph.title.contains(absolute_name)
                        }
                        _ => false,
                    }
                }) {
                    param.description = Some(paragraph.contents.to_owned());
                    param.document = Some(document.clone());
                    continue;
                }
            }
        }
    }
}
