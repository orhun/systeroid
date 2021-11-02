use getopts::Options;
use std::env;
use std::path::PathBuf;

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
    /// Path of the Linux kernel documentation.
    pub kernel_docs: Option<PathBuf>,
    /// Display all of the kernel parameters.
    pub display_all: bool,
    /// Disable colored output.
    pub no_color: bool,
    /// Parameter names.
    pub param_names: Vec<String>,
}

impl Args {
    /// Parses the command-line arguments.
    pub fn parse() -> Option<Self> {
        let mut opts = Options::new();
        opts.optflag("h", "help", "display this help and exit");
        opts.optflag("V", "version", "output version information and exit");
        opts.optflag("a", "all", "display all variables");
        opts.optflag("A", "", "alias of -a");
        opts.optflag("X", "", "alias of -a");
        opts.optflag("", "no-color", "disable colored output");
        opts.optopt(
            "d",
            "docs",
            "set the path of the kernel documentation",
            "<path>",
        );

        let matches = opts
            .parse(&env::args().collect::<Vec<String>>()[1..])
            .map_err(|e| eprintln!("error: {}", e))
            .ok()?;

        let display_all = matches.opt_present("a")
            || matches.opt_present("A")
            || matches.opt_present("X")
            || !matches.free.is_empty();

        if matches.opt_present("h") || !display_all {
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
                kernel_docs: matches.opt_str("d").map(PathBuf::from),
                display_all,
                no_color: matches.opt_present("no-color"),
                param_names: matches.free,
            })
        }
    }
}
