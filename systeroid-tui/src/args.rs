use getopts::Options;

/// Help message for the arguments.
const HELP_MESSAGE: &str = r#"
Usage:
    {bin} [options]

Options:
{usage}

For more details see {bin}(8)."#;

/// Command-line arguments.
#[derive(Debug, Default)]
pub struct Args {
    /// Refresh rate of the terminal.
    pub tick_rate: u64,
}

impl Args {
    /// Returns the available options.
    fn get_options() -> Options {
        let mut opts = Options::new();
        opts.optopt(
            "t",
            "tick-rate",
            "set the tick rate of the terminal [default: 250]",
            "<ms>",
        );
        opts.optflag("h", "help", "display this help and exit");
        opts.optflag("V", "version", "output version information and exit");
        opts
    }

    /// Parses the command-line arguments.
    pub fn parse(env_args: Vec<String>) -> Option<Self> {
        let opts = Self::get_options();
        let matches = opts
            .parse(&env_args[1..])
            .map_err(|e| eprintln!("error: `{}`", e))
            .ok()?;
        if matches.opt_present("h") {
            let usage = opts.usage_with_format(|opts| {
                HELP_MESSAGE
                    .replace("{bin}", env!("CARGO_PKG_NAME"))
                    .replace("{usage}", &opts.collect::<Vec<String>>().join("\n"))
            });
            println!("{}", usage);
            None
        } else if matches.opt_present("V") {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            None
        } else {
            Some(Args {
                tick_rate: matches
                    .opt_get("t")
                    .map_err(|e| eprintln!("error: `{}`", e))
                    .ok()?
                    .unwrap_or(250),
            })
        }
    }
}
