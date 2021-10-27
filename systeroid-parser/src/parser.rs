use crate::document::{Document, Paragraph};
use crate::error::Error;
use crate::reader;
use regex::{Captures, Regex, RegexBuilder};
use std::path::Path;
use std::result::Result as StdResult;

/// Parser for the reStructuredText format.
#[derive(Clone, Debug)]
pub struct RstParser<'a> {
    /// Glob pattern to specify the files to parse.
    pub glob_path: &'a str,
    /// Regular expression to use for parsing.
    pub regex: Regex,
}

impl<'a> RstParser<'a> {
    /// Constructs a new instance.
    pub fn new(glob_path: &'a str, regex: &'a str) -> Result<Self, Error> {
        Ok(Self {
            glob_path,
            regex: RegexBuilder::new(regex).multi_line(true).build()?,
        })
    }

    /// Parses the files in the given base path and returns the documents.
    pub fn parse(&self, base_path: &Path) -> Result<Vec<Document>, Error> {
        let mut documents = Vec::new();
        for file in globwalk::glob(
            base_path
                .join(self.glob_path)
                .to_str()
                .ok_or(Error::Utf8Error)?,
        )?
        .filter_map(StdResult::ok)
        {
            let input = reader::read_to_string(file.path())?;
            let capture_group = self
                .regex
                .captures_iter(&input)
                .collect::<Vec<Captures<'_>>>();
            documents.push(Document::new(
                Paragraph::from_captures(capture_group, &input)?,
                file.path().to_path_buf(),
            ));
        }
        Ok(documents)
    }
}
