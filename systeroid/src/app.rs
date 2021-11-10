use std::io::{self, Stdout};
use systeroid_core::config::Config;
use systeroid_core::error::Result;
use systeroid_core::sysctl::Sysctl;

/// Application controller.
#[derive(Debug)]
pub struct App<'a> {
    /// Sysctl manager.
    sysctl: &'a mut Sysctl,
    /// Configuration.
    config: &'a Config,
    /// Standard output.
    stdout: Stdout,
}

impl<'a> App<'a> {
    /// Constructs a new instance.
    pub fn new(sysctl: &'a mut Sysctl, config: &'a Config) -> Self {
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
            .try_for_each(|parameter| parameter.display(&self.config.color, &mut self.stdout))
    }

    /// Updates the parameter if it has the format `name=value`, displays it otherwise.
    pub fn process_parameter(&mut self, mut param_name: String) -> Result<()> {
        let new_value = if param_name.contains('=') {
            let fields = param_name
                .split('=')
                .take(2)
                .map(String::from)
                .collect::<Vec<String>>();
            param_name = fields[0].to_string();
            Some(fields[1].to_string())
        } else {
            None
        };
        match self
            .sysctl
            .parameters
            .iter_mut()
            .find(|param| param.name == *param_name)
        {
            Some(parameter) => {
                if let Some(new_value) = new_value {
                    parameter.update(&new_value, &self.config.color, &mut self.stdout)?;
                } else {
                    parameter.display(&self.config.color, &mut self.stdout)?;
                }
            }
            None => eprintln!(
                "{}: cannot stat /proc/{}: No such file or directory",
                env!("CARGO_PKG_NAME"),
                param_name.replace(".", "/")
            ),
        }
        Ok(())
    }
}
