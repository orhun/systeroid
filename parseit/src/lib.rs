//! Simple text file parsing library powered by [regex](https://en.wikipedia.org/wiki/Regular_expression) and [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).

#![warn(missing_docs, clippy::unwrap_used)]

/// Export regex crate.
pub use regex;

/// Export globwalk crate.
pub use globwalk;

/// Document parser.
pub mod parser;

/// Parser results.
pub mod document;

/// Error implementation.
pub mod error;

/// File reader.
pub mod reader;
