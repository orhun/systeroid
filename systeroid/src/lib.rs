//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Main application.
pub mod app;
/// Command-line argument parser.
pub mod args;

use crate::app::App;
use crate::args::Args;
use systeroid_core::config::Config;
use systeroid_core::error::Result;
use systeroid_core::sysctl::Sysctl;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let mut config = Config::default();
    config.color.no_color = args.no_color;
    let mut sysctl = Sysctl::init()?;

    if let Some(kernel_docs) = args.kernel_docs {
        sysctl.update_docs(&kernel_docs)?;
    }

    let mut app = App::new(&mut sysctl, &config);

    if args.param_names.is_empty() {
        app.display_parameters()?;
    } else {
        for param_name in args.param_names {
            app.process_parameter(param_name)?;
        }
    }

    Ok(())
}
