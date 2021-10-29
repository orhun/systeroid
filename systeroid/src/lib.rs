//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;

use crate::args::Args;
use systeroid_core::error::Result;
use systeroid_core::sysctl::Sysctl;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let mut sysctl = Sysctl::init()?;

    if let Some(kernel_docs) = args.kernel_docs {
        sysctl.update_docs(&kernel_docs)?;
    }

    for param in sysctl.parameters {
        println!(
            "{}\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n{}\n(from: {:?})\n",
            param.name,
            param
                .description
                .unwrap_or_else(|| String::from("no documentation")),
            param.document.map(|d| d.path).unwrap_or_default(),
        );
    }

    Ok(())
}
