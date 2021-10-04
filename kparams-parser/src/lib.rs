//! kparams-parser

#![warn(missing_docs, clippy::unwrap_used)]

/// Linux kernel types.
pub mod kernel;

/// RST parser.
pub mod parser;

/// File reader.
pub mod reader;

#[macro_use]
extern crate pest_derive;
