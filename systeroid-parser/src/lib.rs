//! systeroid-parser

#![warn(missing_docs, clippy::unwrap_used)]

/// RST parser.
pub mod parser;

/// Parsed document.
pub mod document;

/// Error implementation.
pub mod error;

/// File reader.
pub mod reader;
