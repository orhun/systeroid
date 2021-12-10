use crate::config::Config;
use crate::error::Result;
use crate::sysctl::display::DisplayType;
use crate::sysctl::section::Section;
use colored::*;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;
use sysctl::{Ctl, Sysctl as SysctlImpl};

/// Representation of a kernel parameter.
#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    /// Name of the kernel parameter.
    pub name: String,
    /// Value of the kernel parameter.
    #[serde(skip)]
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

impl Parameter {
    /// Returns the parameter name with corresponding section colors.
    pub fn colored_name(&self, config: &Config) -> String {
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
    pub fn display_value<W: Write>(&self, config: &Config, output: &mut W) -> Result<()> {
        if !config.no_color {
            match config.display_type {
                DisplayType::Name => {
                    writeln!(output, "{}", self.colored_name(config))?;
                }
                DisplayType::Value => {
                    writeln!(output, "{}", self.value.bold())?;
                }
                DisplayType::Binary => {
                    write!(output, "{}", self.value.bold())?;
                }
                DisplayType::Default => {
                    writeln!(
                        output,
                        "{} {} {}",
                        self.colored_name(config),
                        "=".color(config.default_color),
                        self.value.bold(),
                    )?;
                }
            }
        } else {
            match config.display_type {
                DisplayType::Name => {
                    writeln!(output, "{}", self.name)?;
                }
                DisplayType::Value => {
                    writeln!(output, "{}", self.value)?;
                }
                DisplayType::Binary => {
                    write!(output, "{}", self.value)?;
                }
                DisplayType::Default => {
                    writeln!(output, "{} = {}", self.name, self.value)?;
                }
            }
        }
        Ok(())
    }

    /// Returns the parameter documentation if it exists.
    pub fn get_documentation(&self) -> Option<String> {
        self.description.as_ref().map(|description| {
            format!(
                "{}\n{}\n{}\n-\nParameter: {}\nReference: {}",
                self.docs_title,
                "=".repeat(self.docs_title.len()),
                description,
                self.name,
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
        config: &Config,
        output: &mut W,
    ) -> Result<()> {
        let ctl = Ctl::new(&self.name)?;
        let new_value = ctl.set_value_string(new_value)?;
        self.value = new_value;
        if !config.quiet {
            self.display_value(config, output)?;
        }
        Ok(())
    }
}
