//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;

use crate::args::Args;
use std::io::{self, Write};
use systeroid_core::config::Config;
use systeroid_core::error::Result;
use systeroid_core::sysctl::Sysctl;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let mut config = Config::default();
    config.color.no_color = args.no_color;
    let (mut stdout, mut stderr) = (io::stdout(), io::stderr());
    let mut sysctl = Sysctl::init()?;

    if let Some(kernel_docs) = args.kernel_docs {
        sysctl.update_docs(&kernel_docs)?;
    }

    if args.param_names.is_empty() {
        sysctl
            .parameters
            .iter()
            .try_for_each(|parameter| parameter.display(&config.color, &mut stdout))?;
    } else {
        for mut param_name in args.param_names {
            let new_value = if param_name.contains('=') {
                let fields = param_name
                    .split('=')
                    .take(2)
                    .map(String::from)
                    .collect::<Vec<String>>();
                param_name = fields[0].to_string();
                Some(fields[1].to_string())
            } else {
                None
            };
            match sysctl
                .parameters
                .iter_mut()
                .find(|param| param.name == *param_name)
            {
                Some(parameter) => {
                    if let Some(new_value) = new_value {
                        parameter.update(&new_value, &config.color, &mut stdout)?;
                    } else {
                        parameter.display(&config.color, &mut stdout)?;
                    }
                }
                None => writeln!(
                    stderr,
                    "{}: cannot stat /proc/{}: No such file or directory",
                    env!("CARGO_PKG_NAME"),
                    param_name.replace(".", "/")
                )?,
            }
        }
    }

    Ok(())
}
