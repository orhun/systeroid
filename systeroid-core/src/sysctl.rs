use crate::config::{AppConfig, SysctlConfig};
use crate::error::Result;
use crate::parsers::parse_kernel_docs;
use colored::*;
use rayon::prelude::*;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use sysctl::{Ctl, CtlFlags, CtlIter, Sysctl as SysctlImpl};

/// Sections of the sysctl documentation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
    /// Documentation path.
    pub docs_path: PathBuf,
    /// Title of the kernel parameter taken from the documentation.
    pub docs_title: String,
}

impl Parameter {
    /// Returns the parameter name with corresponding section colors.
    pub fn colored_name(&self, config: &AppConfig) -> String {
        let fields = self.name.split('.').collect::<Vec<&str>>();
        fields
            .iter()
            .enumerate()
            .fold(String::new(), |mut result, (i, v)| {
                if i != fields.len() - 1 {
                    let section_color = *(config
                        .section_colors
                        .get(&self.section)
                        .unwrap_or(&config.default_color));
                    result += &format!(
                        "{}{}",
                        v.color(section_color),
                        ".".color(config.default_color)
                    );
                } else {
                    result += v;
                }
                result
            })
    }

    /// Prints the kernel parameter to given output.
    pub fn display_value<W: Write>(&self, config: &AppConfig, output: &mut W) -> Result<()> {
        if !config.no_color {
            writeln!(
                output,
                "{} {} {}",
                self.colored_name(config),
                "=".color(config.default_color),
                self.value.bold(),
            )?;
        } else {
            writeln!(output, "{} = {}", self.name, self.value)?;
        }
        Ok(())
    }

    /// Returns the parameter documentation if it exists.
    pub fn get_documentation(&self) -> Option<String> {
        self.description.as_ref().map(|description| {
            let title = if let Some(absolute_name) = self.name.split('.').last() {
                self.docs_title.replacen(absolute_name, &self.name, 1)
            } else {
                self.name.to_owned()
            };
            format!(
                "{}\n{}\n{}\n-\nReference: {}",
                title,
                "=".repeat(title.len()),
                description,
                self.docs_path.to_string_lossy()
            )
        })
    }

    /// Prints the description of the kernel parameter to the given output.
    pub fn display_documentation<W: Write>(&self, output: &mut W) -> Result<()> {
        if let Some(documentation) = self.get_documentation() {
            writeln!(output, "{}", documentation)?;
        } else {
            writeln!(output, "No documentation available")?;
        }
        Ok(())
    }

    /// Sets a new value for the kernel parameter.
    pub fn update_value<W: Write>(
        &mut self,
        new_value: &str,
        config: &AppConfig,
        output: &mut W,
    ) -> Result<()> {
        let ctl = Ctl::new(&self.name)?;
        let new_value = ctl.set_value_string(new_value)?;
        self.value = new_value;
        self.display_value(config, output)
    }
}

impl<'a> TryFrom<&'a Ctl> for Parameter {
    type Error = crate::error::Error;
    fn try_from(ctl: &'a Ctl) -> Result<Self> {
        Ok(Parameter {
            name: ctl.name()?,
            value: ctl.value_string()?,
            description: ctl
                .description()
                .ok()
                .and_then(|v| (v == "[N/A]").then(|| None)?),
            section: Section::from(ctl.name()?),
            docs_path: PathBuf::new(),
            docs_title: String::new(),
        })
    }
}

/// Sysctl wrapper for managing the kernel parameters.
#[derive(Debug)]
pub struct Sysctl {
    /// Available kernel parameters.
    pub parameters: Vec<Parameter>,
    /// Configuration.
    pub config: SysctlConfig,
}

impl Sysctl {
    /// Constructs a new instance by fetching the available kernel parameters.
    pub fn init(config: SysctlConfig) -> Result<Self> {
        let mut parameters = Vec::new();
        for ctl in CtlIter::root().filter_map(StdResult::ok).filter(|ctl| {
            ctl.flags()
                .map(|flags| !flags.contains(CtlFlags::SKIP))
                .unwrap_or(false)
        }) {
            match Parameter::try_from(&ctl) {
                Ok(parameter) => {
                    parameters.push(parameter);
                }
                Err(e) => {
                    eprintln!("{} ({})", e, ctl.name()?);
                }
            }
        }
        Ok(Self { parameters, config })
    }

    /// Searches and returns the parameter if it exists.
    pub fn get_parameter(&mut self, param_name: &str) -> Option<&mut Parameter> {
        let parameter = self
            .parameters
            .iter_mut()
            .find(|param| param.name == *param_name);
        if parameter.is_none() && !self.config.ignore_errors {
            eprintln!(
                "{}: cannot stat /proc/{}: No such file or directory",
                env!("CARGO_PKG_NAME").split('-').collect::<Vec<_>>()[0],
                param_name.replace(".", "/")
            )
        }
        parameter
    }

    /// Updates the descriptions of the kernel parameters.
    pub fn update_docs(&mut self, kernel_docs: &Path) -> Result<()> {
        let documents = parse_kernel_docs(kernel_docs)?;
        self.parameters
            .par_iter_mut()
            .filter(|p| p.description.is_none())
            .for_each(|param| {
                for document in documents
                    .iter()
                    .filter(|document| Section::from(document.path.as_path()) == param.section)
                {
                    if let Some(paragraph) =
                        document.paragraphs.par_iter().find_first(|paragraph| {
                            match param.name.split('.').collect::<Vec<&str>>().last() {
                                Some(absolute_name) => {
                                    absolute_name.len() > 2
                                        && paragraph.title.contains(absolute_name)
                                }
                                _ => false,
                            }
                        })
                    {
                        param.description = Some(paragraph.contents.to_owned());
                        param.docs_title = paragraph.title.to_owned();
                        param.docs_path = document.path.clone();
                        continue;
                    }
                }
            });
        Ok(())
    }
}
