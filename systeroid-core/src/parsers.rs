use systeroid_parser::parser::Parser;
use systeroid_parser::regex::RegexBuilder;

lazy_static! {
    /// Pre-defined parsers for parsing the kernel documentation.
    pub static ref PARSERS: Vec<Parser<'static>> = vec![
        Parser {
            glob_path: "admin-guide/sysctl/*.rst",
            regex: RegexBuilder::new("^\n([a-z].*)\n[=,-]{2,}+\n\n")
                .multi_line(true)
                .build()
                .expect("failed to compile regex"),
        },
        Parser {
            glob_path: "networking/*-sysctl.rst",
            regex: RegexBuilder::new("^([a-zA-Z0-9_/-]+)[ ]-[ ][a-zA-Z].*$")
                .multi_line(true)
                .build()
                .expect("failed to compile regex"),
        },
    ];
}
