//! kparams

#![warn(missing_docs, clippy::unwrap_used)]

use kparams_core::error::{Error, Result};
use kparams_core::kernel::SysctlSection;
use kparams_core::reader;
use kparams_parser::parser::RstParser;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Mutex;

/// Runs `kparams`.
pub fn run() -> Result<()> {
    let kernel_docs = PathBuf::from("/usr/share/doc/linux");
    let sysctl_docs = kernel_docs.join("admin-guide").join("sysctl");

    let kernel_parameters = Mutex::new(Vec::new());
    SysctlSection::variants().par_iter().try_for_each(|s| {
        let mut kernel_parameters = kernel_parameters
            .lock()
            .map_err(|e| Error::ThreadLockError(e.to_string()))?;
        let mut parse = |section: SysctlSection| -> Result<()> {
            let docs = reader::read_to_string(&sysctl_docs.join(section.as_file()))?;
            Ok(kernel_parameters.extend(RstParser::parse_docs(&docs, section)?))
        };
        parse(*s)
    })?;

    for param in kernel_parameters
        .lock()
        .map_err(|e| Error::ThreadLockError(e.to_string()))?
        .iter()
    {
        println!("## {}::{}\n", param.section, param.name);
        println!("{}\n", param.description);
    }

    Ok(())
}
