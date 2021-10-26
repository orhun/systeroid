use regex::{Captures, RegexBuilder};
use std::path::Path;
use std::result::Result as StdResult;
use systeroid_core::docs::{Documentation, SysctlSection};
use systeroid_core::error::{Error, Result};
use systeroid_core::reader;

/// Parser for the reStructuredText format.
#[derive(Clone, Copy, Debug)]
pub struct RstParser<'a> {
    /// Glob pattern to specify the files to parse.
    pub glob_path: &'a str,
    /// Regular expression to use for parsing.
    pub regex: &'a str,
    /// Section of the parsed documents.
    pub section: Option<SysctlSection>,
}

impl RstParser<'_> {
    /// Parses the given reStructuredText input and returns the [`documentation`] of kernel parameters.
    ///
    /// [`documentation`]: Documentation
    pub fn parse(&self, kernel_docs: &Path) -> Result<Vec<Documentation>> {
        let mut param_docs = Vec::new();

        let regex = RegexBuilder::new(self.regex)
            .multi_line(true)
            .build()
            .map_err(|e| Error::RegexError(e.to_string()))?;
        for file in globwalk::glob(
            kernel_docs
                .join(self.glob_path)
                .to_str()
                .ok_or(Error::Utf8Error)?,
        )
        .map_err(|e| Error::GlobError(e.to_string()))?
        .filter_map(StdResult::ok)
        {
            let section = self
                .section
                .unwrap_or_else(|| SysctlSection::from(file.path()));
            let input = reader::read_to_string(file.path())?;
            let capture_group = regex.captures_iter(&input).collect::<Vec<Captures<'_>>>();

            for (i, captures) in capture_group.iter().enumerate() {
                let title_capture = captures.iter().last().flatten().unwrap();
                let capture = captures.iter().next().flatten().unwrap();

                param_docs.push(Documentation::new(
                    title_capture.as_str().trim().to_string(),
                    if let Some(next_capture) = capture_group.get(i + 1) {
                        let next_capture = next_capture.iter().next().flatten().unwrap();
                        (input[capture.end()..next_capture.start()])
                            .trim()
                            .to_string()
                    } else {
                        (input[capture.end()..]).trim().to_string()
                    },
                    section,
                ));
            }
        }

        Ok(param_docs)
    }
}
