use crate::cache::{Cache, CacheData};
use crate::config::Config;
use crate::error::Result;
use crate::parsers::{parse_kernel_docs, KERNEL_DOCS_PATH};
use crate::sysctl::parameter::Parameter;
use crate::sysctl::section::Section;
use crate::sysctl::{DISABLE_CACHE_ENV, PARAMETERS_CACHE_LABEL, PROC_PATH};
use parseit::globwalk;
use rayon::prelude::*;
use std::convert::TryFrom;
use std::env;
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

    /// Returns the first found parameter in the available parameters.
    #[cfg(test)]
    fn get_parameter(&self, query: &str) -> Option<&Parameter> {
        self.get_parameters(query).first().map(|v| *v)
    }

    /// Returns the parameters that matches the given query.
    pub fn get_parameters(&self, query: &str) -> Vec<&Parameter> {
        let parameters = self
            .parameters
            .iter()
            .filter(|param| {
                param.name == query.replace('/', ".")
                    || param.section.to_string() == query
                    || param.get_absolute_name() == Some(&query.replace('/', "."))
            })
            .collect::<Vec<&Parameter>>();
        if parameters.is_empty() && !self.config.ignore_errors {
            eprintln!(
                "{}: cannot stat {}{}: No such file or directory",
                env!("CARGO_PKG_NAME").split('-').collect::<Vec<_>>()[0],
                PROC_PATH,
                query.replace('.', "/")
            )
        }
        parameters
    }

    /// Updates the descriptions of the kernel parameters using the given cached data.
    pub fn update_docs_from_cache(
        &mut self,
        kernel_docs: Option<&PathBuf>,
        cache: &Cache,
    ) -> Result<()> {
        let mut kernel_docs_path = if let Some(path) = kernel_docs {
            vec![path.to_path_buf()]
        } else {
            Vec::new()
        };
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
                let cache_data = cache.read(PARAMETERS_CACHE_LABEL)?;
                if cache_data.timestamp == CacheData::<()>::get_timestamp(path)? {
                    self.update_params(cache_data.data);
                    return Ok(());
                }
            }
            self.update_docs(path)?;
            if env::var(DISABLE_CACHE_ENV).is_err() {
                cache.write(
                    CacheData::new(&self.parameters, path)?,
                    PARAMETERS_CACHE_LABEL,
                )?;
            }
        } else {
            eprintln!("warning: `Linux kernel documentation cannot be found. Please specify a path via '-D' argument`");
        }
        Ok(())
    }

    /// Updates the parameters internally using the given list.
    ///
    /// Keeps the original values.
    fn update_params(&mut self, mut parameters: Vec<Parameter>) {
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
    fn update_docs(&mut self, kernel_docs: &Path) -> Result<()> {
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
            "Linux",
            sysctl.get_parameters("ostype").first().unwrap().value
        );
        assert!(sysctl.get_parameters("---").is_empty());

        sysctl.update_docs_from_cache(None, &Cache::init()?)?;

        let parameter = sysctl.get_parameter("kernel.hostname").unwrap().clone();
        let old_value = parameter.docs_title;
        let parameters = sysctl.parameters.clone();
        sysctl
            .parameters
            .iter_mut()
            .find(|param| param.name == parameter.name)
            .unwrap()
            .docs_title = String::from("-");
        sysctl.update_params(parameters);
        assert_eq!(
            old_value,
            sysctl
                .parameters
                .iter_mut()
                .find(|param| param.name == parameter.name)
                .unwrap()
                .docs_title
        );

        assert!(sysctl
            .get_parameter("vm.zone_reclaim_mode")
            .unwrap()
            .description
            .as_ref()
            .unwrap()
            .contains("zone_reclaim_mode is disabled by default."));

        assert!(sysctl
            .get_parameter("user.max_user_namespaces")
            .unwrap()
            .description
            .as_ref()
            .unwrap()
            .contains("The maximum number of user namespaces"));

        Ok(())
    }
}
