//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;

use crate::args::Args;
use std::io;
use systeroid_core::error::Result;
use systeroid_core::sysctl::Sysctl;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let mut stdout = io::stdout();
    let mut sysctl = Sysctl::init()?;

    if let Some(kernel_docs) = args.kernel_docs {
        sysctl.update_docs(&kernel_docs)?;
    }

    if args.all {
        sysctl.print_all(&mut stdout)?;
    }

    Ok(())
}
