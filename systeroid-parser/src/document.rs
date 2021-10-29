use crate::error::Error;
use regex::Captures;
use std::path::PathBuf;

/// Representation of a paragraph in a [`Document`].
#[derive(Clone, Debug, PartialEq)]
pub struct Paragraph {
    /// Paragraph title.
    pub title: String,
    /// Raw contents of a paragraph.
    pub contents: String,
}

impl Paragraph {
    /// Constructs a new instance.
    pub fn new(title: String, contents: String) -> Self {
        Self { title, contents }
    }

    /// Constructs a vector of paragraphs from the given regex capture groups.
    pub fn from_captures(
        capture_group: Vec<Captures<'_>>,
        input: &str,
    ) -> Result<Vec<Self>, Error> {
        let mut paragraphs = Vec::new();
        for (i, captures) in capture_group.iter().enumerate() {
            let title_capture = captures
                .iter()
                .last()
                .flatten()
                .ok_or(Error::CaptureError)?;
            let content_capture = captures
                .iter()
                .next()
                .flatten()
                .ok_or(Error::CaptureError)?;
            paragraphs.push(Paragraph::new(
                title_capture.as_str().trim().to_string(),
                if let Some(next_capture) = capture_group.get(i + 1) {
                    let next_capture = next_capture
                        .iter()
                        .next()
                        .flatten()
                        .ok_or(Error::CaptureError)?;
                    (input[content_capture.end()..next_capture.start()])
                        .trim()
                        .to_string()
                } else {
                    (input[content_capture.end()..]).trim().to_string()
                },
            ));
        }
        Ok(paragraphs)
    }
}

/// Representation of a parsed document which consists of paragraphs.
#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    /// Paragraphs in the document.
    pub paragraphs: Vec<Paragraph>,
    /// Source of the document.
    pub path: PathBuf,
}

impl Document {
    /// Constructs a new instance.
    pub fn new(paragraphs: Vec<Paragraph>, path: PathBuf) -> Self {
        Self { paragraphs, path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader;
    use regex::RegexBuilder;

    #[test]
    fn test_paragraph() -> Result<(), Error> {
        let input =
            reader::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"))?;
        let captures = RegexBuilder::new(r#"^\[[a-zA-Z]+\]\n"#)
            .multi_line(true)
            .build()?
            .captures_iter(&input)
            .collect::<Vec<_>>();
        let paragraphs = Paragraph::from_captures(captures, &input)?;
        assert!(paragraphs.len() >= 2);

        assert_eq!("[package]", paragraphs[0].title);
        assert!(paragraphs[0]
            .contents
            .contains(&format!("version = \"{}\"", env!("CARGO_PKG_VERSION"))));

        assert_eq!("[dependencies]", paragraphs[1].title);
        assert!(paragraphs[1].contents.contains("regex = "));
        Ok(())
    }
}
