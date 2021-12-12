//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Main application.
pub mod app;
/// Command-line argument parser.
pub mod args;

use crate::app::App;
use crate::args::Args;
use std::env;
use systeroid_core::config::Config;
use systeroid_core::error::Result;
use systeroid_core::sysctl::controller::Sysctl;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let config = Config {
        verbose: args.verbose,
        ignore_errors: args.ignore_errors,
        quiet: args.quiet,
        no_pager: args.no_pager,
        display_type: args.display_type,
        no_color: env::var("NO_COLOR").is_ok(),
        ..Default::default()
    };
    let mut sysctl = Sysctl::init(config)?;
    let mut app = App::new(&mut sysctl)?;

    if args.values.is_empty() {
        app.display_parameters(args.pattern, args.display_deprecated)?;
    } else if args.explain_params {
        app.update_documentation(args.kernel_docs.as_ref())?;
        for param in args.values {
            app.display_documentation(&param)?;
        }
    } else if args.preload_files {
        for file in args.values {
            app.preload_values(file)?;
        }
    } else {
        for param in args.values {
            app.process_parameter(param, true)?;
        }
    }

    Ok(())
}
