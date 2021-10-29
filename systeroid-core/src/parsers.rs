use crate::error::{Error, Result};
use rayon::prelude::*;
use std::path::Path;
use systeroid_parser::document::Document;
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

/// Parses the kernel documentation using the defined parsers.
pub fn parse_kernel_docs(kernel_docs: &Path) -> Result<Vec<Document>> {
    PARSERS
        .par_iter()
        .try_fold(Vec::new, |mut documents, parser| {
            documents.extend(parser.parse(kernel_docs)?);
            Ok::<Vec<Document>, Error>(documents)
        })
        .try_reduce(Vec::new, |mut v1, v2| {
            v1.extend(v2);
            Ok(v1)
        })
}
