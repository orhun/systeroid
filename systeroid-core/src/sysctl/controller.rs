use crate::cache::{Cache, CacheData};
use crate::config::Config;
use crate::error::Result;
use crate::parsers::{parse_kernel_docs, KERNEL_DOCS_PATH};
use crate::sysctl::parameter::Parameter;
use crate::sysctl::section::Section;
use crate::sysctl::{
    DEFAULT_PRELOAD, DEPRECATED_PARAMS, DISABLE_CACHE_ENV, PARAMETERS_CACHE_LABEL, PROC_PATH,
};
use parseit::globwalk;
use parseit::reader;
use rayon::prelude::*;
use std::convert::TryFrom;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::result::Result as StdResult;
use sysctl::{CtlFlags, CtlIter, Sysctl as SysctlImpl};

/// Sysctl wrapper for managing the kernel parameters.
#[derive(Clone, Debug)]
pub struct Sysctl {
    /// Available kernel parameters.
    pub parameters: Vec<Parameter>,
    /// Configuration.
    pub config: Config,
}

impl Sysctl {
    /// Constructs a new instance by fetching the available kernel parameters.
    pub fn init(config: Config) -> Result<Self> {
        let parameters = CtlIter::root()
            .filter_map(StdResult::ok)
            .filter(|ctl| {
                ctl.flags()
                    .map(|flags| !flags.contains(CtlFlags::SKIP))
                    .unwrap_or(false)
            })
            .filter_map(|ctl| match Parameter::try_from(&ctl) {
                Ok(parameter) => {
                    if !config.display_deprecated {
                        let skip_param = parameter
                            .get_absolute_name()
                            .map(|pname| DEPRECATED_PARAMS.contains(&pname))
                            .unwrap_or(false);

                        if !skip_param {
                            Some(Ok(parameter))
                        } else {
                            None
                        }
                    } else {
                        Some(Ok(parameter))
                    }
                }
                Err(e) => match ctl.name() {
                    Ok(name) => {
                        log::trace!(target: "sysctl", "{} ({})", e, name);
                        None
                    }
                    Err(e) => Some(Err(crate::error::Error::from(e))),
                },
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { parameters, config })
    }

    /// Returns the first found parameter in the available parameters.
    #[cfg(test)]
    fn get_parameter(&self, query: &str) -> Option<&Parameter> {
        self.get_parameters(query).first().copied()
    }

    /// Returns the parameters that matches the given query.
    pub fn get_parameters(&self, query: &str) -> Vec<&Parameter> {
        log::trace!(target: "sysctl", "Querying parameters: {:?}", query);
        let query = query.replace('/', ".");
        let parameters = self
            .parameters
            .iter()
            .filter(|param| {
                param.name == query
                    || param.get_absolute_name() == Some(&query)
                    || param.is_in_section(&query)
            })
            .collect::<Vec<&Parameter>>();
        if parameters.is_empty() && !self.config.cli.ignore_errors {
            log::error!(
                target: "sysctl",
                "{}: cannot stat {}{}: No such file or directory",
                env!("CARGO_PKG_NAME").split('-').collect::<Vec<_>>()[0],
                PROC_PATH,
                query.replace('.', "/")
            )
        }
        parameters
    }

    /// Updates the descriptions of the kernel parameters using the given cached data.
    pub fn update_docs_from_cache(&mut self, cache: &Cache) -> Result<()> {
        log::trace!(target: "cache", "{:?}", cache);
        let mut kernel_docs_path = self
            .config
            .kernel_docs
            .as_ref()
            .map(|p| vec![p.to_path_buf()])
            .unwrap_or_default();

        for path in KERNEL_DOCS_PATH {
            if let Some(mut path) = globwalk::glob(path).ok().and_then(|glob| {
                glob.filter_map(StdResult::ok)
                    .filter(|entry| entry.file_type().is_dir())
                    .map(|entry| entry.into_path())
                    .next()
            }) {
                path.pop();
                kernel_docs_path.push(path);
            }
        }
        if let Some(path) = kernel_docs_path.iter().find(|path| path.exists()) {
            if cache.exists(PARAMETERS_CACHE_LABEL) {
                log::trace!(target: "cache", "Cache hit for {:?}", path);
                let cache_data = cache.read(PARAMETERS_CACHE_LABEL)?;
                if cache_data.timestamp == CacheData::<()>::get_timestamp(path)? {
                    self.update_params(cache_data.data);
                    return Ok(());
                }
            }
            self.update_docs(path)?;
            if env::var(DISABLE_CACHE_ENV).is_err() {
                log::trace!(target: "cache", "Writing cache to {:?}", cache);
                cache.write(
                    CacheData::new(&self.parameters, path)?,
                    PARAMETERS_CACHE_LABEL,
                )?;
            }
        } else {
            log::error!(target: "sysctl", "warning: `Linux kernel documentation cannot be found. Please specify a path via '-D' argument`");
        }
        Ok(())
    }

    /// Updates the parameters internally using the given list.
    ///
    /// Keeps the original values.
    fn update_params(&mut self, parameters: Vec<Parameter>) {
        self.parameters.par_iter_mut().for_each(|parameter| {
            if let Some(param) = parameters
                .par_iter()
                .find_any(|param| param.name == parameter.name)
            {
                parameter.description = param.description.clone();
                parameter.docs_path = param.docs_path.clone();
                parameter.docs_title = param.docs_title.clone();
            }
        });
    }

    /// Updates the descriptions of the kernel parameters.
    fn update_docs(&mut self, kernel_docs: &Path) -> Result<()> {
        log::trace!(target: "sysctl", "Parsing the kernel documentation from {:?}", kernel_docs);
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
                            match param.get_absolute_name() {
                                Some(absolute_name) => {
                                    absolute_name == paragraph.title
                                        || absolute_name.len() > 2
                                            && paragraph.title.contains(absolute_name)
                                }
                                None => false,
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

    /// Saves the parameter values to the given file.
    pub fn save_to_file(
        &self,
        param_name: String,
        new_value: String,
        save_path: &Option<PathBuf>,
    ) -> Result<PathBuf> {
        let save_path = save_path
            .clone()
            .unwrap_or_else(|| PathBuf::from(DEFAULT_PRELOAD));
        log::trace!(
            target: "param",
            "Writing the new value ({:?}) of {:?} to {:?}",
            new_value,
            param_name,
            save_path
        );
        let data = format!("{param_name} = {new_value}");
        if save_path.exists() {
            let contents = reader::read_to_string(&save_path)?;
            let mut lines = contents.split('\n').collect::<Vec<&str>>();
            if let Some(line) = lines.iter_mut().find(|v| v.starts_with(&param_name)) {
                *line = &data;
            } else {
                lines.push(&data);
            }
            let mut file = OpenOptions::new()
                .write(true)
                .create(false)
                .truncate(false)
                .open(&save_path)?;
            file.write_all(lines.join("\n").as_bytes())?;
        } else {
            let mut file = File::create(&save_path)?;
            file.write_all(data.as_bytes())?;
        }
        Ok(save_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysctl_controller() -> Result<()> {
        env::set_var(DISABLE_CACHE_ENV, "1");
        let config = Config::default();
        let mut sysctl = Sysctl::init(config)?;
        assert!(sysctl.get_parameter("kernel.hostname").is_some());
        assert!(sysctl.get_parameter("unexisting.param").is_none());
        assert_eq!(
            Some(String::from("Linux")),
            sysctl
                .get_parameters("ostype")
                .first()
                .map(|v| v.value.to_string())
        );
        assert!(sysctl.get_parameters("---").is_empty());

        sysctl.update_docs_from_cache(&Cache::init()?)?;

        let parameter = sysctl
            .get_parameter("kernel.hostname")
            .expect("failed to get parameter")
            .clone();
        let old_value = parameter.docs_title;
        let parameters = sysctl.parameters.clone();
        sysctl
            .parameters
            .iter_mut()
            .find(|param| param.name == parameter.name)
            .expect("parameter not found")
            .docs_title = String::from("-");
        sysctl.update_params(parameters);
        assert_eq!(
            old_value,
            sysctl
                .parameters
                .iter_mut()
                .find(|param| param.name == parameter.name)
                .expect("parameter not found")
                .docs_title
        );

        assert!(sysctl
            .get_parameter("vm.zone_reclaim_mode")
            .expect("failed to get parameter")
            .description
            .as_ref()
            .expect("parameter has no description")
            .contains("zone_reclaim_mode is disabled by default."));

        assert!(sysctl
            .get_parameter("user.max_user_namespaces")
            .expect("failed to get parameter")
            .description
            .as_ref()
            .expect("parameter has no description")
            .contains("The maximum number of user namespaces"));

        Ok(())
    }
}
