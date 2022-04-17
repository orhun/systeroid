use crate::output::OutputType;
use std::env;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use systeroid_core::error::Result;
use systeroid_core::parseit::globwalk;
use systeroid_core::parseit::reader;
use systeroid_core::parseit::regex::Regex;
use systeroid_core::sysctl::controller::Sysctl;
use systeroid_core::sysctl::parameter::Parameter;
use systeroid_core::sysctl::{DEPRECATED_PARAMS, SYSTEM_PRELOAD};
use systeroid_core::tree::{Tree, TreeNode};

/// Application controller.
#[derive(Debug)]
pub struct App<'a, Output: Write> {
    /// Sysctl controller.
    sysctl: &'a mut Sysctl,
    /// Standard output.
    output: &'a mut Output,
    /// Output type.
    output_type: OutputType,
}

impl<'a, Output: Write> App<'a, Output> {
    /// Constructs a new instance.
    pub fn new(sysctl: &'a mut Sysctl, output: &'a mut Output, output_type: OutputType) -> Self {
        Self {
            sysctl,
            output,
            output_type,
        }
    }

    /// Prints the given parameters to stdout.
    fn print_parameters<'b, I>(&mut self, parameters: &mut I) -> Result<()>
    where
        I: Iterator<Item = &'b Parameter>,
    {
        match self.output_type {
            OutputType::Default => {
                parameters.try_for_each(|parameter| {
                    parameter.display_value(&self.sysctl.config, self.output)
                })?;
            }
            OutputType::Tree => {
                let mut root_node = TreeNode::default();
                parameters.for_each(|parameter| {
                    root_node.add(
                        &mut parameter
                            .get_tree_components(&self.sysctl.config)
                            .iter()
                            .map(|v| v.as_ref()),
                    );
                });
                Tree::new(root_node.childs).print(self.output, self.sysctl.config.default_color)?;
            }
            OutputType::Json => {
                Parameter::display_bulk_json(parameters.collect(), self.output)?;
            }
        }
        Ok(())
    }

    /// Displays all of the available kernel parameters.
    pub fn display_parameters(
        &mut self,
        pattern: Option<Regex>,
        display_deprecated: bool,
        explain: bool,
    ) -> Result<()> {
        let parameters = self.sysctl.parameters.clone();
        let mut parameters = parameters.iter().filter(|parameter| {
            if let Some(pattern) = &pattern {
                return pattern.is_match(&parameter.name);
            }
            if !display_deprecated {
                if let Some(param_name) = parameter.get_absolute_name() {
                    return !DEPRECATED_PARAMS.contains(&param_name);
                }
            }
            true
        });
        if explain {
            parameters.try_for_each(|parameter| self.display_documentation(&parameter.name))
        } else {
            self.print_parameters(&mut parameters)
        }
    }

    /// Displays the documentation of a parameter.
    pub fn display_documentation(&mut self, param_name: &str) -> Result<()> {
        let no_pager = self.sysctl.config.no_pager;
        for parameter in self.sysctl.get_parameters(param_name) {
            let mut fallback_to_default = false;
            if no_pager {
                parameter.display_documentation(self.output)?;
                continue;
            }
            let pager = env::var("PAGER").unwrap_or_else(|_| String::from("less"));
            match Command::new(&pager).stdin(Stdio::piped()).spawn() {
                Ok(mut process) => {
                    if let Some(stdin) = process.stdin.as_mut() {
                        parameter.display_documentation(stdin)?;
                        process.wait()?;
                    } else {
                        fallback_to_default = true;
                    }
                }
                Err(e) => {
                    if !pager.is_empty() {
                        eprintln!("pager error: `{}`", e);
                    }
                    fallback_to_default = true;
                }
            }
            if fallback_to_default {
                parameter.display_documentation(self.output)?;
            }
        }
        Ok(())
    }

    /// Updates the parameter if it has the format `name=value`, displays it otherwise.
    pub fn process_parameter(
        &mut self,
        mut parameter: String,
        display_value: bool,
        write_mode: bool,
    ) -> Result<()> {
        let new_value = if parameter.contains('=') {
            let fields = parameter
                .split('=')
                .map(|v| v.trim().to_string())
                .collect::<Vec<String>>();
            parameter = fields[0].to_string();
            Some(fields[1..].join("="))
        } else {
            None
        };
        let sysctl = self.sysctl.clone();
        if let Some(new_value) = new_value {
            let parameters = sysctl.get_parameters(&parameter);
            if parameters.len() == 1 {
                let param = parameters[0];
                if DEPRECATED_PARAMS.contains(&param.get_absolute_name().unwrap_or_default()) {
                    eprintln!(
                        "{}: {} is deprecated, value not set",
                        env!("CARGO_PKG_NAME"),
                        parameter
                    );
                } else if let Some(param) = self
                    .sysctl
                    .parameters
                    .iter_mut()
                    .find(|p| p.name == param.name)
                {
                    let config = self.sysctl.config.clone();
                    param.update_value(&new_value, &config, self.output)?;
                }
            } else {
                eprintln!(
                    "{}: ambiguous parameter name: {}",
                    env!("CARGO_PKG_NAME"),
                    parameter
                );
            }
        } else if write_mode {
            eprintln!(
                "{}: {:?} must be in the format: name=value",
                env!("CARGO_PKG_NAME"),
                parameter
            );
        } else if display_value {
            let parameters = sysctl.get_parameters(&parameter);
            self.print_parameters(&mut parameters.into_iter())?;
        }
        Ok(())
    }

    /// Processes the parameters in the given file.
    pub fn preload_from_file(&mut self, path: PathBuf) -> Result<()> {
        if path == PathBuf::from("-") {
            for line in io::stdin().lock().lines() {
                if let Err(e) = self.process_parameter(line?, true, false) {
                    println!("{}: {}", env!("CARGO_PKG_NAME"), e);
                }
            }
            return Ok(());
        }
        if !path.exists() {
            eprintln!(
                "{}: cannot open {:?}: No such file or directory",
                env!("CARGO_PKG_NAME"),
                path
            );
            return Ok(());
        }
        let contents = reader::read_to_string(path)?;
        for parameter in contents
            .lines()
            .filter(|v| !(v.starts_with('#') || v.starts_with(';') || v.is_empty()))
        {
            let process_result =
                self.process_parameter(parameter.trim_start_matches('-').to_string(), false, false);
            if !parameter.starts_with('-') {
                process_result?;
            } else if let Err(e) = process_result {
                eprintln!("{}: {}", env!("CARGO_PKG_NAME"), e);
            }
        }
        Ok(())
    }

    /// Processes the parameters in files that are in predefined system directories.
    pub fn preload_from_system(&mut self) -> Result<()> {
        for preload_path in SYSTEM_PRELOAD
            .iter()
            .map(|v| PathBuf::from(v).join("*.conf"))
        {
            if let Ok(glob_walker) = globwalk::glob(preload_path.to_string_lossy()) {
                for file in glob_walker.filter_map(|v| v.ok()) {
                    println!("* Applying {} ...", file.path().display());
                    self.preload_from_file(file.path().to_path_buf())?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use systeroid_core::cache::Cache;
    use systeroid_core::config::Config;

    #[test]
    fn test_app() -> Result<()> {
        let mut output = Vec::new();
        let mut sysctl = Sysctl::init(Config {
            no_pager: true,
            ..Config::default()
        })?;
        sysctl.update_docs_from_cache(None, &Cache::init()?)?;

        let mut app = App::new(&mut sysctl, &mut output, OutputType::Default);

        app.display_parameters(Regex::new("kernel|vm").ok(), false, false)?;
        let result = String::from_utf8_lossy(app.output);
        assert!(result.contains("vm.zone_reclaim_mode ="));
        assert!(result.contains("kernel.version ="));
        app.output.clear();

        app.output_type = OutputType::Tree;
        app.display_parameters(None, true, false)?;
        assert!(String::from_utf8_lossy(app.output).contains("─ osrelease ="));
        app.output.clear();

        app.display_documentation("kernel.acct")?;
        assert!(String::from_utf8_lossy(app.output).contains("highwater lowwater frequency"));
        app.output.clear();

        let param_name = String::from("kernel.version");
        app.output_type = OutputType::Default;
        app.process_parameter(param_name.clone(), true, false)?;
        let result = String::from_utf8_lossy(app.output);
        assert_eq!(1, result.lines().count());
        assert!(result.contains(&param_name));
        app.output.clear();

        let param_name = String::from("kernel.version");
        app.output_type = OutputType::Json;
        app.process_parameter(param_name.clone(), true, false)?;
        let result = String::from_utf8_lossy(app.output);
        assert!(result.contains("\"section\":\"kernel\""));
        assert!(result.contains(&format!("\"name\":\"{}\"", param_name)));

        Ok(())
    }
}
