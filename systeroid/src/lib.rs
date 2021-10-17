//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;

use crate::args::Args;
use rayon::prelude::*;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::sync::Mutex;
use systeroid_core::error::{Error, Result};
use systeroid_core::kernel::SysctlSection;
use systeroid_core::reader;
use systeroid_parser::parser::RstParser;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    if let Some(kernel_docs) = args.kernel_docs {
        let sysctl_docs = kernel_docs.join("admin-guide").join("sysctl");
        if !sysctl_docs.exists() {
            return Err(IoError::new(
                IoErrorKind::Other,
                format!("cannot find sysctl documentation: {:?}", sysctl_docs),
            )
            .into());
        }

        let param_docs = Mutex::new(Vec::new());
        SysctlSection::variants().par_iter().try_for_each(|s| {
            let mut param_docs = param_docs
                .lock()
                .map_err(|e| Error::ThreadLockError(e.to_string()))?;
            let mut parse = |section: SysctlSection| -> Result<()> {
                let docs = reader::read_to_string(&sysctl_docs.join(section.as_file()))?;
                param_docs.extend(RstParser::parse_docs(&docs, section)?);
                Ok(())
            };
            parse(*s)
        })?;

        for param in param_docs
            .lock()
            .map_err(|e| Error::ThreadLockError(e.to_string()))?
            .iter()
        {
            println!("## {}.{}\n", param.section, param.name);
            println!("{}\n", param.description);
        }
    }

    Ok(())
}
