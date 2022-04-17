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
#[derive(Clone, Debug, Serialize, Deserialize)]
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
            section: Section::from_name(ctl.name()?),
            docs_path: PathBuf::new(),
            docs_title: String::new(),
        })
    }
}

impl Parameter {
    /// Returns the absolute name of the parameter, without the sections.
    pub fn get_absolute_name(&self) -> Option<&str> {
        self.name.split('.').collect::<Vec<&str>>().last().copied()
    }

    /// Returns the parameter name with corresponding section colors.
    pub fn get_colored_name(&self, config: &Config) -> String {
        let section_color = *(config
            .section_colors
            .get(&self.section)
            .unwrap_or(&config.default_color));
        let fields = self.name.split('.').collect::<Vec<&str>>();
        fields
            .iter()
            .enumerate()
            .fold(String::new(), |mut result, (i, v)| {
                if i != fields.len() - 1 {
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

    /// Returns the components of the parameter to contruct a [`Tree`].
    ///
    /// [`Tree`]: crate::tree::Tree
    pub fn get_tree_components(&self, config: &Config) -> Vec<String> {
        let section_color = *(config
            .section_colors
            .get(&self.section)
            .unwrap_or(&config.default_color));
        let mut components = self
            .name
            .split('.')
            .map(String::from)
            .collect::<Vec<String>>();
        let total_components = components.len();
        components
            .iter_mut()
            .enumerate()
            .for_each(|(i, component)| {
                if i != total_components - 1 {
                    *component = component.color(section_color).to_string();
                } else if config.display_type != DisplayType::Name {
                    *component = format!(
                        "{} {} {}",
                        component,
                        "=".color(config.default_color),
                        self.value.replace('\n', " ").bold()
                    );
                }
            });
        components
    }

    /// Prints the kernel parameter to given output.
    pub fn display_value<Output: Write>(&self, config: &Config, output: &mut Output) -> Result<()> {
        match config.display_type {
            DisplayType::Name => {
                writeln!(output, "{}", self.get_colored_name(config))?;
            }
            DisplayType::Value => {
                writeln!(output, "{}", self.value.bold())?;
            }
            DisplayType::Binary => {
                write!(output, "{}", self.value.bold())?;
            }
            DisplayType::Default => {
                for value in self.value.lines() {
                    writeln!(
                        output,
                        "{} {} {}",
                        self.get_colored_name(config),
                        "=".color(config.default_color),
                        value.bold(),
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Prints the given parameters in JSON format.
    pub fn display_bulk_json<Output: Write>(
        parameters: Vec<&Self>,
        output: &mut Output,
    ) -> Result<()> {
        let parameters = parameters
            .iter()
            .map(|p| {
                serde_json::json!({
                    "name": p.name,
                    "value": p.value,
                    "section": p.section.to_string(),
                })
            })
            .collect::<Vec<_>>();
        writeln!(output, "{}", serde_json::to_string(&parameters)?)?;
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
    pub fn display_documentation<Output: Write>(&self, output: &mut Output) -> Result<()> {
        if let Some(documentation) = self.get_documentation() {
            writeln!(output, "{}\n", documentation)?;
        } else {
            writeln!(output, "No documentation available for {}", self.name)?;
        }
        Ok(())
    }

    /// Sets a new value for the kernel parameter.
    pub fn update_value<Output: Write>(
        &mut self,
        new_value: &str,
        config: &Config,
        output: &mut Output,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysctl_parameter() -> Result<()> {
        let mut parameter = Parameter {
            name: String::from("kernel.fictional.test_param"),
            value: String::from("1"),
            description: Some(String::from("This is a fictional parameter for testing")),
            section: Section::Kernel,
            docs_path: PathBuf::from("/etc/cosmos"),
            docs_title: String::from("Test Parameter"),
        };
        assert_eq!(Some("test_param"), parameter.get_absolute_name());

        let mut config = Config::default();
        config.default_color = Color::White;
        *(config.section_colors.get_mut(&Section::Kernel).unwrap()) = Color::Yellow;
        assert_eq!(parameter.name, parameter.get_colored_name(&config));

        assert_eq!(
            vec![
                String::from("kernel"),
                String::from("fictional"),
                String::from("test_param = 1")
            ],
            parameter.get_tree_components(&config)
        );

        let mut output = Vec::new();
        parameter.display_value(&config, &mut output)?;
        assert_eq!(
            "kernel.fictional.test_param = 1\n",
            String::from_utf8_lossy(&output)
        );

        output.clear();
        config.display_type = DisplayType::Name;
        parameter.display_value(&config, &mut output)?;
        assert_eq!(
            "kernel.fictional.test_param\n",
            String::from_utf8_lossy(&output)
        );

        output.clear();
        config.display_type = DisplayType::Value;
        parameter.display_value(&config, &mut output)?;
        assert_eq!("1\n", String::from_utf8_lossy(&output));

        output.clear();
        config.display_type = DisplayType::Binary;
        parameter.display_value(&config, &mut output)?;
        assert_eq!("1", String::from_utf8_lossy(&output));

        let mut output = Vec::new();
        parameter.display_documentation(&mut output)?;
        assert_eq!(
            "Test Parameter
            ==============
            This is a fictional parameter for testing
            -
            Parameter: kernel.fictional.test_param
            Reference: /etc/cosmos\n\n\n"
                .lines()
                .map(|line| line.trim_start())
                .collect::<Vec<&str>>()
                .join("\n"),
            String::from_utf8_lossy(&output)
        );

        parameter.description = None;
        let mut output = Vec::new();
        parameter.display_documentation(&mut output)?;
        assert_eq!(
            format!("No documentation available for {}\n", parameter.name),
            String::from_utf8_lossy(&output)
        );

        assert!(parameter
            .update_value("0", &config, &mut Vec::new())
            .is_err());

        Ok(())
    }
}
