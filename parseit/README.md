# parseit

Simple text file parsing library powered by [regex](https://en.wikipedia.org/wiki/Regular_expression) and [glob patterns](<https://en.wikipedia.org/wiki/Glob_(programming)>).

```rs
// Create a parser to parse sections in Cargo.toml (and optionally Cargo.lock)
let parser = Parser::new(&["Cargo.*"], &["Cargo.toml"], r#"^\[(.*)\]$\n"#).unwrap();

// Parse the files in the manifest directory.
let documents = parser
    .parse(&PathBuf::from(env!("CARGO_MANIFEST_DIR")))
    .unwrap();

// Print results.
for document in documents {
    println!("Path: {}", document.path.to_string_lossy());
    for paragraph in document.paragraphs {
        println!("Title: {}", paragraph.title);
        println!("Contents: {}", paragraph.contents);
        println!();
    }
}
```

## Examples

See [examples](./examples/).

## License

Licensed under either of [Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or [The MIT License](http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache 2.0 License, shall be dual licensed as above, without any additional terms or conditions.

## Copyright

Copyright © 2022, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
