//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Main application.
pub mod app;
/// Command-line argument parser.
pub mod args;
/// Application output types.
pub mod output;

use crate::app::App;
use crate::args::Args;
use std::io::Write;
use systeroid_core::config::Config;
use systeroid_core::error::Result;
use systeroid_core::sysctl::controller::Sysctl;

/// Runs `systeroid`.
pub fn run<Output: Write>(args: Args, output: &mut Output) -> Result<()> {
    let config = Config {
        verbose: args.verbose,
        ignore_errors: args.ignore_errors,
        quiet: args.quiet,
        no_pager: args.no_pager,
        display_type: args.display_type,
        ..Default::default()
    };
    let mut sysctl = Sysctl::init(config)?;
    let mut app = App::new(&mut sysctl, output, args.output_type)?;

    if args.preload_system_files {
        app.preload_from_system()?;
    } else if args.values.is_empty() {
        app.display_parameters(args.pattern, args.display_deprecated)?;
    } else if args.explain {
        app.update_documentation(args.kernel_docs.as_ref())?;
        for param in args.values {
            app.display_documentation(&param)?;
        }
    } else if args.preload_files {
        for file in args.values {
            app.preload_from_file(file)?;
        }
    } else {
        for param in args.values {
            app.process_parameter(param, true, args.write)?;
        }
    }

    Ok(())
}
