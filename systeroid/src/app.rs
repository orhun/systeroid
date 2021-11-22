use std::env;
use std::io::{self, Stdout};
use std::path::Path;
use std::process::{Command, Stdio};
use systeroid_core::config::AppConfig;
use systeroid_core::error::Result;
use systeroid_core::sysctl::Sysctl;

/// Application controller.
#[derive(Debug)]
pub struct App<'a> {
    /// Sysctl manager.
    sysctl: &'a mut Sysctl,
    /// Configuration.
    config: &'a AppConfig,
    /// Standard output.
    stdout: Stdout,
}

impl<'a> App<'a> {
    /// Constructs a new instance.
    pub fn new(sysctl: &'a mut Sysctl, config: &'a AppConfig) -> Self {
        let stdout = io::stdout();
        Self {
            sysctl,
            config,
            stdout,
        }
    }

    /// Displays all of the available kernel modules.
    pub fn display_parameters(&mut self) -> Result<()> {
        self.sysctl
            .parameters
            .iter()
            .try_for_each(|parameter| parameter.display_value(self.config, &mut self.stdout))
    }

    /// Updates the documentation for kernel parameters.
    fn fetch_documentation(&mut self, kernel_docs: &Path) -> Result<()> {
        if !kernel_docs.exists() {
            eprintln!(
                "warning: Linux kernel documentation is not found in path: {:?}",
                kernel_docs.to_string_lossy()
            );
        }
        self.sysctl.update_docs(kernel_docs)
    }

    /// Displays the documentation of a parameter.
    pub fn display_documentation(&mut self, param_name: &str, kernel_docs: &Path) -> Result<()> {
        self.fetch_documentation(kernel_docs)?;
        if let Some(parameter) = self.sysctl.get_parameter(param_name) {
            let mut fallback_to_default = false;
            let pager = env::var("PAGER").unwrap_or_else(|_| String::from("less"));
            match Command::new(pager).stdin(Stdio::piped()).spawn() {
                Ok(mut process) => {
                    if let Some(stdin) = process.stdin.as_mut() {
                        parameter.display_documentation(stdin)?;
                        process.wait()?;
                    } else {
                        fallback_to_default = true;
                    }
                }
                Err(e) => {
                    eprintln!("error: `pager error: {}`", e);
                    fallback_to_default = true;
                }
            }
            if fallback_to_default {
                parameter.display_documentation(&mut self.stdout)?;
            }
        }
        Ok(())
    }

    /// Updates the parameter if it has the format `name=value`, displays it otherwise.
    pub fn process_parameter(&mut self, mut param_name: String) -> Result<()> {
        let new_value = if param_name.contains('=') {
            let fields = param_name
                .split('=')
                .take(2)
                .map(|v| v.trim().to_string())
                .collect::<Vec<String>>();
            param_name = fields[0].to_string();
            Some(fields[1].to_string())
        } else {
            None
        };
        if let Some(parameter) = self.sysctl.get_parameter(&param_name) {
            if let Some(new_value) = new_value {
                parameter.update_value(&new_value, self.config, &mut self.stdout)?;
            } else {
                parameter.display_value(self.config, &mut self.stdout)?;
            }
        }
        Ok(())
    }
}
