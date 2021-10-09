//! kparams

#![warn(missing_docs, clippy::unwrap_used)]

use kparams_core::error::Result;
use kparams_core::kernel::SysctlSection;
use kparams_core::reader;
use kparams_parser::parser::RstParser;
use std::path::PathBuf;

/// Runs `kparams`.
pub fn run() -> Result<()> {
    let kernel_docs = PathBuf::from("/usr/share/doc/linux");
    let sysctl_docs = kernel_docs.join("admin-guide").join("sysctl");

    for sysctl_section in SysctlSection::iter() {
        let sysctl_section_docs =
            reader::read_to_string(&sysctl_docs.join(sysctl_section.as_file()))?;
        let kernel_parameters = RstParser::parse_docs(&sysctl_section_docs, sysctl_section)?;
        for param in kernel_parameters {
            println!("## {}::{}\n", param.section, param.name);
            println!("{}\n", param.description);
        }
    }

    Ok(())
}
