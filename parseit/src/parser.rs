use crate::document::{Document, Paragraph};
use crate::error::Error;
use crate::reader;
use globwalk::DirEntry;
use regex::{Captures, Regex, RegexBuilder};
use std::path::Path;
use std::result::Result as StdResult;

/// Parser for text files.
///
/// It is responsible for traversing the path specified with
/// a glob pattern and parsing the contents of the files.
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    /// Glob patterns to specify the files to parse.
    pub glob_path: &'a [&'a str],
    /// Files to check during path traversal.
    pub required_files: &'a [&'a str],
    /// Regular expression to use for parsing.
    pub regex: Regex,
}

impl<'a> Parser<'a> {
    /// Constructs a new instance.
    pub fn new(
        glob_path: &'a [&'a str],
        required_files: &'a [&'a str],
        regex: &'a str,
    ) -> Result<Self, Error> {
        Ok(Self {
            glob_path,
            required_files,
            regex: RegexBuilder::new(regex).multi_line(true).build()?,
        })
    }

    /// Parses the files in the given base path and returns the documents.
    pub fn parse(&self, base_path: &Path) -> Result<Vec<Document>, Error> {
        let mut documents = Vec::new();
        let mut glob_files = Vec::new();
        for glob in self.glob_path {
            glob_files.extend(
                globwalk::glob(base_path.join(glob).to_str().ok_or(Error::Utf8Error)?)?
                    .filter_map(StdResult::ok)
                    .collect::<Vec<DirEntry>>(),
            );
        }
        if glob_files.is_empty() {
            return Err(Error::EmptyFileListError);
        }
        self.required_files
            .iter()
            .filter(|file_name| !file_name.is_empty())
            .try_for_each(|file_name| {
                glob_files
                    .iter()
                    .find(|file| file.file_name().to_str() == Some(file_name))
                    .map(drop)
                    .ok_or_else(|| Error::MissingFileError(file_name.to_string()))
            })?;
        for file in glob_files {
            let input = {
                #[cfg(feature = "gzip")]
                if file.path().extension().and_then(|ext| ext.to_str()) == Some("gz") {
                    reader::read_gzip(file.path())
                } else {
                    reader::read_to_string(file.path())
                }
                #[cfg(not(feature = "gzip"))]
                reader::read_to_string(file.path())
            }?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_document_parser() -> Result<(), Error> {
        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let parser = Parser::new(&["Cargo.*"], &[], r#"^(\[package\])\n"#)?;
        let mut documents = parser.parse(base_path.as_path())?;

        assert!(documents[0].paragraphs[0]
            .contents
            .contains(&format!("name = \"{}\"", env!("CARGO_PKG_NAME"))));

        documents[0].paragraphs[0].contents = String::new();
        assert_eq!(
            Document {
                paragraphs: vec![Paragraph {
                    title: String::from("[package]"),
                    contents: String::new(),
                }],
                path: base_path.join("Cargo.toml")
            },
            documents[0]
        );
        Ok(())
    }
}
