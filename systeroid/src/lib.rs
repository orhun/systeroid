//! systeroid

#![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;

use crate::args::Args;
use systeroid_core::error::{Error, Result};
use systeroid_core::parsers::PARSERS;
use systeroid_core::sysctl::Sysctl;
use systeroid_parser::document::Document;

/// Runs `systeroid`.
pub fn run(args: Args) -> Result<()> {
    let mut sysctl = Sysctl::init()?;

    if let Some(kernel_docs) = args.kernel_docs {
        let documents = PARSERS
            .iter()
            .try_fold(Vec::new(), |mut documents, parser| {
                documents.extend(parser.parse(&kernel_docs)?);
                Ok::<Vec<Document>, Error>(documents)
            })?;
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
