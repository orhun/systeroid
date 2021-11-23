use crate::document::{Document, Paragraph};
use crate::error::Error;
use crate::reader;
use globwalk::DirEntry;
use regex::{Captures, Regex, RegexBuilder};
use std::path::Path;
use std::result::Result as StdResult;

/// Regex-powered parser for text documents.
///
/// It is responsible for traversing the path specified with
/// a glob pattern and parsing the contents of the files.
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    /// Glob pattern to specify the files to parse.
    pub glob_path: &'a str,
    /// Files to check during path traversal.
    pub required_files: &'a [&'a str],
    /// Regular expression to use for parsing.
    pub regex: Regex,
}

impl<'a> Parser<'a> {
    /// Constructs a new instance.
    pub fn new(
        glob_path: &'a str,
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
        let glob_files = globwalk::glob(
            base_path
                .join(self.glob_path)
                .to_str()
                .ok_or(Error::Utf8Error)?,
        )?
        .filter_map(StdResult::ok)
        .collect::<Vec<DirEntry>>();
        self.required_files.iter().try_for_each(|file_name| {
            glob_files
                .iter()
                .find(|file| file.file_name().to_str() == Some(file_name))
                .map(drop)
                .ok_or_else(|| Error::MissingFileError(file_name.to_string()))
        })?;
        for file in glob_files {
            println!("{:?}", file);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_document_parser() -> Result<(), Error> {
        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let parser = Parser::new("Cargo.*", &["Cargo.toml"], r#"^(\[package\])\n"#)?;
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
