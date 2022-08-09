//! A more powerful alternative to sysctl.

#![warn(missing_docs, clippy::unwrap_used)]

/// Main application.
pub mod app;
/// Command-line argument parser.
pub mod args;

use crate::app::App;
use crate::args::Args;
use std::io::Write;
use std::path::PathBuf;
use systeroid_core::cache::Cache;
use systeroid_core::config::Config;
use systeroid_core::error::Result;
use systeroid_core::sysctl::controller::Sysctl;

/// Runs `systeroid`.
pub fn run<Output: Write>(args: Args, output: &mut Output) -> Result<()> {
    let mut config = Config {
        display_deprecated: args.display_deprecated,
        kernel_docs: args.kernel_docs,
        ..Default::default()
    };
    config.cli.verbose = args.verbose;
    config.cli.ignore_errors = args.ignore_errors;
    config.cli.quiet = args.quiet;
    config.cli.no_pager = args.no_pager;
    config.cli.display_type = args.display_type;
    config.cli.output_type = args.output_type;
    config.parse(args.config)?;
    let mut sysctl = Sysctl::init(config)?;
    if args.explain {
        sysctl.update_docs_from_cache(&Cache::init()?)?;
    }
    let mut app = App::new(&mut sysctl, output);

    if args.preload_system_files {
        app.preload_from_system()?;
    } else if args.values.is_empty() {
        app.display_parameters(args.pattern, args.explain)?;
    } else if args.explain {
        for param in args.values {
            app.display_documentation(&param)?;
        }
    } else if args.preload_files {
        for file in args.values {
            app.preload_from_file(PathBuf::from(file))?;
        }
    } else {
        for param in args.values {
            app.process_parameter(param, true, args.write)?;
        }
    }

    Ok(())
}
