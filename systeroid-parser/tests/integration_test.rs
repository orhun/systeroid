use std::path::PathBuf;
use systeroid_parser::error::Error;
use systeroid_parser::parser::Parser;

#[test]
fn test_parser() -> Result<(), Error> {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let parser = Parser::new("src/*.rs", r#"^(#\[cfg\(test\)\])$\n"#)?;
    let documents = parser.parse(base_path.as_path())?;

    assert!(documents
        .iter()
        .find(|d| d.path == PathBuf::from(base_path.join("src").join("lib.rs")))
        .unwrap()
        .paragraphs
        .is_empty());

    assert!(documents
        .iter()
        .find(|d| d.path == PathBuf::from(base_path.join("src").join("reader.rs")))
        .unwrap()
        .paragraphs[0]
        .contents
        .contains("fn test_file_reader()"));

    documents.iter().for_each(|document| {
        document.paragraphs.iter().for_each(|paragraph| {
            assert_eq!("#[cfg(test)]", paragraph.title);
            assert!(paragraph.contents.contains("mod tests"));
            assert!(paragraph.contents.contains("use super::*;"));
        });
    });

    Ok(())
}
