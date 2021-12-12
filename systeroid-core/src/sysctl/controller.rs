use crate::config::Config;
use crate::error::Result;
use crate::parsers::parse_kernel_docs;
use crate::sysctl::parameter::Parameter;
use crate::sysctl::section::Section;
use rayon::prelude::*;
use std::convert::TryFrom;
use std::path::Path;
use std::result::Result as StdResult;
use sysctl::{CtlFlags, CtlIter, Sysctl as SysctlImpl};

/// Sysctl wrapper for managing the kernel parameters.
#[derive(Debug)]
pub struct Sysctl {
    /// Available kernel parameters.
    pub parameters: Vec<Parameter>,
    /// Configuration.
    pub config: Config,
}

impl Sysctl {
    /// Constructs a new instance by fetching the available kernel parameters.
    pub fn init(config: Config) -> Result<Self> {
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
                    if config.verbose {
                        eprintln!("{} ({})", e, ctl.name()?);
                    }
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
            .find(|param| param.name == *param_name.replace("/", "."));
        if parameter.is_none() && !self.config.ignore_errors {
            eprintln!(
                "{}: cannot stat /proc/{}: No such file or directory",
                env!("CARGO_PKG_NAME").split('-').collect::<Vec<_>>()[0],
                param_name.replace(".", "/")
            )
        }
        parameter
    }

    /// Returns the parameters that matches the given query.
    pub fn get_parameters(&self, query: &str) -> Vec<&Parameter> {
        let parameters = self
            .parameters
            .iter()
            .filter(|param| {
                param.name == query.replace("/", ".")
                    || param.section.to_string() == query
                    || param.absolute_name() == Some(&query.replace("/", "."))
            })
            .collect::<Vec<&Parameter>>();
        if parameters.is_empty() {
            eprintln!(
                "{}: cannot stat /proc/{}: No such file or directory",
                env!("CARGO_PKG_NAME").split('-').collect::<Vec<_>>()[0],
                query.replace(".", "/")
            )
        }
        parameters
    }

    /// Updates the parameters using the given list.
    ///
    /// Keeps the original values.
    pub fn update_params(&mut self, mut parameters: Vec<Parameter>) {
        parameters.par_iter_mut().for_each(|parameter| {
            if let Some(param) = self
                .parameters
                .par_iter()
                .find_any(|param| param.name == parameter.name)
            {
                parameter.value = param.value.to_string();
            }
        });
        self.parameters = parameters;
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
                            match param.absolute_name() {
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
