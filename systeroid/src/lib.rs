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
use systeroid_core::sysctl::Sysctl;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let mut config = Config::default();
    config.sysctl.ignore_errors = args.ignore_errors;
    config.color.no_color = env::var("NO_COLOR").is_ok();
    let mut sysctl = Sysctl::init(config.sysctl)?;
    let mut app = App::new(&mut sysctl, &config.color);

    if let Some(param) = args.param_to_explain {
        app.display_documentation(&param, &args.kernel_docs)?;
    } else if args.param_names.is_empty() {
        app.display_parameters()?;
    } else {
        for param_name in args.param_names {
            app.process_parameter(param_name)?;
        }
    }

    Ok(())
}
