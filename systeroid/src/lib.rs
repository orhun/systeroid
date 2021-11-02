//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;

use crate::args::Args;
use std::io;
use systeroid_core::config::Config;
use systeroid_core::error::Result;
use systeroid_core::sysctl::Sysctl;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let config = Config::default();
    let mut stdout = io::stdout();
    let mut sysctl = Sysctl::init(config.sysctl)?;

    if let Some(kernel_docs) = args.kernel_docs {
        sysctl.update_docs(&kernel_docs)?;
    }

    if args.display_all {
        sysctl.display(&mut stdout, args.param_names, !args.no_color)?;
    }

    Ok(())
}
