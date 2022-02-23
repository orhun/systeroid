use parseit::error::Error;
use parseit::parser::Parser;
use std::path::PathBuf;

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
