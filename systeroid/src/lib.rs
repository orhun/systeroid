//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;

use crate::args::Args;
use rayon::prelude::*;
use std::sync::Mutex;
use systeroid_core::error::{Error, Result};
use systeroid_core::sysctl::Sysctl;
use systeroid_parser::document::Document;
use systeroid_parser::parser::RstParser;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let mut sysctl = Sysctl::init()?;

    let parsers = vec![
        RstParser::new("admin-guide/sysctl/*.rst", "^\n([a-z].*)\n[=,-]{2,}+\n\n")?,
        RstParser::new(
            "networking/*-sysctl.rst",
            "^([a-zA-Z0-9_/-]+)[ ]-[ ][a-zA-Z].*$",
        )?,
    ];

    let documents = if let Some(kernel_docs) = args.kernel_docs {
        let documents = Mutex::new(Vec::new());
        parsers.par_iter().try_for_each(|s| {
            let mut documents = documents
                .lock()
                .map_err(|e| Error::ThreadLockError(e.to_string()))?;
            let mut parse = |parser: RstParser| -> Result<()> {
                documents.extend(parser.parse(&kernel_docs)?);
                Ok(())
            };
            parse(s.clone())
        })?;
        let documents = documents
            .lock()
            .map_err(|e| Error::ThreadLockError(e.to_string()))?
            .clone()
            .into_iter()
            .collect::<Vec<Document>>();
        Some(documents)
    } else {
        None
    };

    if let Some(documents) = documents {
        sysctl.update_docs(documents);
    }

    for param in sysctl.parameters {
        println!(
            "{}\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n{}\n(from: {:?})\n",
            param.name,
            param
                .description
                .unwrap_or_else(|| String::from("no documentation")),
            param.document.map(|d| d.path).unwrap_or_default(),
        );
    }

    Ok(())
}
