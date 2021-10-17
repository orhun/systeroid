#![allow(missing_docs)] // pest_derive does not generate doc comments

use crate::title::Title;
use pest::Parser;
use std::convert::TryFrom;
use systeroid_core::error::{Error, Result};
use systeroid_core::kernel::{ParamDoc, SysctlSection};

/// Parser for the reStructuredText format.
#[derive(Parser)]
#[grammar = "rst.pest"]
pub struct RstParser;

impl RstParser {
    /// Parses the given reStructuredText input and returns the [`documentation`] of kernel parameters.
    ///
    /// [`documentation`]: ParamDoc
    pub fn parse_docs(input: &str, section: SysctlSection) -> Result<Vec<ParamDoc>> {
        let mut param_docs = Vec::new();
        let rst_document =
            Self::parse(Rule::document, input).map_err(|e| Error::ParseError(e.to_string()))?;
        let titles = rst_document
            .filter_map(|pair| Title::try_from(pair).ok())
            .collect::<Vec<Title<'_>>>();
        for (i, title) in titles.iter().enumerate() {
            param_docs.push(ParamDoc::new(
                title.value.to_string(),
                if let Some(next_title) = titles.get(i + 1) {
                    (input[title.end_pos..next_title.start_pos])
                        .trim()
                        .to_string()
                } else {
                    (input[title.end_pos..]).trim().to_string()
                },
                section,
            ));
        }
        Ok(param_docs)
    }
}
