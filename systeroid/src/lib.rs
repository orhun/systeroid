//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Mutex;
use systeroid_core::error::{Error, Result};
use systeroid_core::kernel::SysctlSection;
use systeroid_core::reader;
use systeroid_parser::parser::RstParser;

/// Runs `systeroid`.
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
