#![allow(missing_docs)] // pest_derive does not generate doc comments

use crate::title::Title;
use kparams_core::error::{Error, Result};
use kparams_core::kernel::{Parameter, SysctlSection};
use pest::Parser;
use std::convert::TryFrom;

/// Parser for the reStructuredText format.
#[derive(Parser)]
#[grammar = "rst.pest"]
pub struct RstParser;

impl RstParser {
    /// Parses the given reStructuredText input and returns the [`kernel parameters`].
    ///
    /// [`kernel parameters`]: Parameter
    pub fn parse_docs<'a>(
        input: &'a str,
        section: &'a SysctlSection,
    ) -> Result<Vec<Parameter<'a>>> {
        let mut kernel_parameters = Vec::new();
        let rst_document =
            Self::parse(Rule::document, input).map_err(|e| Error::ParseError(e.to_string()))?;
        let titles = rst_document
            .filter_map(|pair| Title::try_from(pair).ok())
            .collect::<Vec<Title<'_>>>();
        for (i, title) in titles.iter().enumerate() {
            kernel_parameters.push(Parameter::new(
                title.value,
                if let Some(next_title) = titles.get(i + 1) {
                    (input[title.end_pos..next_title.start_pos]).trim()
                } else {
                    (input[title.end_pos..]).trim()
                },
                section,
            ));
        }
        Ok(kernel_parameters)
    }
}
