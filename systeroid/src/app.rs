use std::env;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use systeroid_core::cache::{Cache, CacheData};
use systeroid_core::error::Result;
use systeroid_core::parsers::KERNEL_DOCS_PATH;
use systeroid_core::regex::Regex;
use systeroid_core::sysctl::controller::Sysctl;
use systeroid_core::sysctl::DEPRECATED_VARIABLES;
use systeroid_parser::reader;

/// Label for caching the kernel parameters.
const PARAMETERS_CACHE_LABEL: &str = "parameters";

/// Application controller.
#[derive(Debug)]
pub struct App<'a> {
    /// Sysctl controller.
    sysctl: &'a mut Sysctl,
    /// Application cache.
    cache: Cache,
    /// Standard output.
    stdout: Stdout,
}

impl<'a> App<'a> {
    /// Constructs a new instance.
    pub fn new(sysctl: &'a mut Sysctl) -> Result<Self> {
        Ok(Self {
            sysctl,
            cache: Cache::init()?,
            stdout: io::stdout(),
        })
    }

    /// Displays all of the available kernel parameters.
    pub fn display_parameters(
        &mut self,
        pattern: Option<Regex>,
        display_deprecated: bool,
    ) -> Result<()> {
        self.sysctl
            .parameters
            .iter()
            .filter(|parameter| {
                if let Some(pattern) = &pattern {
                    return pattern.is_match(&parameter.name);
                }
                if !display_deprecated {
                    if let Some(param_name) = parameter.absolute_name() {
                        return !DEPRECATED_VARIABLES.contains(&param_name);
                    }
                }
                true
            })
            .try_for_each(|parameter| {
                parameter.display_value(&self.sysctl.config, &mut self.stdout)
            })
    }

    /// Updates the documentation for kernel parameters.
    pub fn update_documentation(&mut self, kernel_docs: Option<&PathBuf>) -> Result<()> {
        let mut kernel_docs_path = KERNEL_DOCS_PATH.clone();
        if let Some(path) = kernel_docs {
            kernel_docs_path.insert(0, path);
        }
        if let Some(path) = kernel_docs_path.iter().find(|path| path.exists()) {
            if self.cache.exists(PARAMETERS_CACHE_LABEL) && kernel_docs.is_none() {
                let cache_data = self.cache.read(PARAMETERS_CACHE_LABEL)?;
                if cache_data.timestamp == CacheData::<()>::get_timestamp(path)? {
                    self.sysctl.update_params(cache_data.data);
                    return Ok(());
                }
            }
            self.sysctl.update_docs(path)?;
            self.cache.write(
                CacheData::new(&self.sysctl.parameters, path)?,
                PARAMETERS_CACHE_LABEL,
            )?;
        } else {
            eprintln!("warning: `Linux kernel documentation cannot be found. Please specify a path via '-d' argument`",);
        }
        Ok(())
    }

    /// Displays the documentation of a parameter.
    pub fn display_documentation(&mut self, param_name: &str) -> Result<()> {
        let no_pager = self.sysctl.config.no_pager;
        for parameter in self.sysctl.get_parameters(param_name) {
            let mut fallback_to_default = false;
            if no_pager {
                parameter.display_documentation(&mut self.stdout)?;
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
                parameter.display_documentation(&mut self.stdout)?;
            }
        }
        Ok(())
    }

    /// Updates the parameter if it has the format `name=value`, displays it otherwise.
    pub fn process_parameter(&mut self, mut parameter: String, display_value: bool) -> Result<()> {
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
        if let Some(new_value) = new_value {
            let config = self.sysctl.config.clone();
            if let Some(param) = self.sysctl.get_parameter(&parameter) {
                param.update_value(&new_value, &config, &mut self.stdout)?;
            }
        } else if display_value {
            self.sysctl
                .get_parameters(&parameter)
                .iter()
                .try_for_each(|parameter| {
                    parameter.display_value(&self.sysctl.config, &mut self.stdout)
                })?;
        }
        Ok(())
    }

    /// Processes the parameters in the given file.
    pub fn preload_values(&mut self, file: String) -> Result<()> {
        let path = PathBuf::from(file);
        if !path.exists() {
            eprintln!(
                "{}: cannot open {:?}: No such file or directory",
                env!("CARGO_PKG_NAME"),
                path
            );
            return Ok(());
        }
        let contents = reader::read_to_string(path)?;
        for parameter in contents.lines() {
            self.process_parameter(parameter.to_string(), false)?;
        }
        Ok(())
    }
}
