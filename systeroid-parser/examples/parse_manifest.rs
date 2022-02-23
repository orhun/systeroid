use std::path::PathBuf;
use systeroid_parser::error::Error;
use systeroid_parser::parser::Parser;

// Parse Cargo manifest and print sections.
fn main() -> Result<(), Error> {
    // Create a parser.
    let parser = Parser::new(&["Cargo.*"], &[], r#"^\[(.*)\]$\n"#)?;

    // Parse documents.
    let documents = parser.parse(&PathBuf::from(env!("CARGO_MANIFEST_DIR")))?;

    // Print results.
    println!("Total parsed files: {}", documents.len());
    for document in documents {
        println!("Contents of {}:", document.path.to_string_lossy());
        println!();
        for paragraph in document.paragraphs {
            println!("[{}]", paragraph.title);
            println!("{}", paragraph.contents);
            println!();
        }
    }

    Ok(())
}
