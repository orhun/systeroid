use getopts::Options;
use std::env;
use std::path::PathBuf;
use systeroid_core::display::DisplayType;
use systeroid_core::regex::Regex;

/// Help message for the arguments.
const HELP_MESSAGE: &str = r#"
Usage:
    {bin} [options] [variable[=value] ...]

Options:
{usage}

For more details see {bin}(8)."#;

/// Command-line arguments.
#[derive(Debug, Default)]
pub struct Args {
    /// Path of the Linux kernel documentation.
    pub kernel_docs: Option<PathBuf>,
    /// Display type of the variables.
    pub display_type: DisplayType,
    /// Whether if the unknown variable errors should be ignored.
    pub ignore_errors: bool,
    /// Do not pipe output into a pager.
    pub no_pager: bool,
    /// Pattern for matching the parameters.
    pub pattern: Option<Regex>,
    /// Parameter to explain.
    pub param_to_explain: Option<String>,
    /// Parameter names.
    pub param_names: Vec<String>,
}

impl Args {
    /// Parses the command-line arguments.
    pub fn parse() -> Option<Self> {
        let mut opts = Options::new();
        opts.optflag("a", "all", "display all variables");
        opts.optflag("A", "", "alias of -a");
        opts.optflag("X", "", "alias of -a");
        opts.optflag("b", "binary", "print value without new line");
        opts.optflag("e", "ignore", "ignore unknown variables errors");
        opts.optflag("N", "names", "print variable names without values");
        opts.optflag("n", "values", "print only values of the given variable(s)");
        opts.optopt(
            "r",
            "pattern",
            "select setting that match expression",
            "<expression>",
        );
        opts.optopt(
            "E",
            "explain",
            "provide a detailed explanation for a variable",
            "<var>",
        );
        opts.optopt(
            "d",
            "docs",
            "set the path of the kernel documentation",
            "<path>",
        );
        opts.optflag("P", "no-pager", "Do not pipe output into a pager");
        opts.optflag("h", "help", "display this help and exit");
        opts.optflag("V", "version", "output version information and exit");

        let env_args = env::args().collect::<Vec<String>>();
        let matches = opts
            .parse(&env_args[1..])
            .map_err(|e| eprintln!("error: `{}`", e))
            .ok()?;

        let required_args_present = matches.opt_present("a")
            || matches.opt_present("A")
            || matches.opt_present("X")
            || !matches.free.is_empty()
            || matches.opt_str("explain").is_some();

        if matches.opt_present("h") || env_args.len() == 1 {
            let usage = opts.usage_with_format(|opts| {
                HELP_MESSAGE
                    .replace("{bin}", env!("CARGO_PKG_NAME"))
                    .replace("{usage}", &opts.collect::<Vec<String>>().join("\n"))
            });
            println!("{}", usage);
            None
        } else if !required_args_present {
            println!(
                "{}: no variables specified\n\
                Try `{} --help' for more information.",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_NAME")
            );
            None
        } else if matches.opt_present("V") {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            None
        } else {
            let display_type = if matches.opt_present("N") {
                DisplayType::Name
            } else if matches.opt_present("n") {
                DisplayType::Value
            } else if matches.opt_present("b") {
                DisplayType::Binary
            } else {
                DisplayType::Default
            };
            Some(Args {
                kernel_docs: matches.opt_str("d").map(PathBuf::from),
                display_type,
                ignore_errors: matches.opt_present("e"),
                no_pager: matches.opt_present("P"),
                pattern: matches
                    .opt_str("r")
                    .map(|v| Regex::new(&v).expect("invalid regex")),
                param_to_explain: matches.opt_str("E"),
                param_names: matches.free,
            })
        }
    }
}
