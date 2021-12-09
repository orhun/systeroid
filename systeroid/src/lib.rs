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
    let mut config = Config::default();
    config.sysctl.verbose = args.verbose;
    config.sysctl.ignore_errors = args.ignore_errors;
    config.app.quiet = args.quiet;
    config.app.no_color = env::var("NO_COLOR").is_ok();
    config.app.no_pager = args.no_pager;
    config.app.display_type = args.display_type;
    let mut sysctl = Sysctl::init(config.sysctl)?;
    let mut app = App::new(&mut sysctl, &config.app)?;

    if let Some(param) = args.param_to_explain {
        app.update_documentation(args.kernel_docs.as_ref())?;
        app.display_documentation(&param)?;
    } else if args.values.is_empty() {
        app.display_parameters(args.pattern)?;
    } else if args.preload_files {
        for file in args.values {
            app.preload_values(file)?;
        }
    } else {
        for param_name in args.values {
            app.process_parameter(param_name, true)?;
        }
    }

    Ok(())
}
