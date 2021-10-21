//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;

use crate::args::Args;
use rayon::prelude::*;
use std::sync::Mutex;
use systeroid_core::docs::{Documentation, SysctlSection};
use systeroid_core::error::{Error, Result};
use systeroid_core::reader;
use systeroid_core::sysctl::Sysctl;
use systeroid_parser::parser::RstParser;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let mut sysctl = Sysctl::init()?;

    let param_docs = if let Some(kernel_docs) = args.kernel_docs {
        let param_docs = Mutex::new(Vec::new());
        SysctlSection::variants().par_iter().try_for_each(|s| {
            let mut param_docs = param_docs
                .lock()
                .map_err(|e| Error::ThreadLockError(e.to_string()))?;
            let mut parse = |section: SysctlSection| -> Result<()> {
                let docs = reader::read_to_string(&section.as_path(&kernel_docs))?;
                param_docs.extend(RstParser::parse_docs(&docs, section)?);
                Ok(())
            };
            parse(*s)
        })?;
        let param_docs = param_docs
            .lock()
            .map_err(|e| Error::ThreadLockError(e.to_string()))?
            .clone()
            .into_iter()
            .collect::<Vec<Documentation>>();
        Some(param_docs)
    } else {
        None
    };

    if let Some(param_docs) = param_docs {
        sysctl.update_docs(param_docs);
    }

    for param in sysctl.parameters {
        println!(
            "{} ({})\n===\n{}\n",
            param.name,
            param.documentation.map(|d| d.name).unwrap_or_default(),
            param
                .description
                .unwrap_or_else(|| String::from("no documentation"))
        );
    }

    Ok(())
}
