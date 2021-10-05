//! kparams-parser

#![warn(missing_docs, clippy::unwrap_used)]

/// RST parser.
pub mod parser;

/// Parsed title.
pub mod title;

#[macro_use]
extern crate pest_derive;
