//! systeroid-core

#![warn(missing_docs, clippy::unwrap_used)]

#[macro_use]
extern crate lazy_static;

/// Export regex crate.
pub use systeroid_parser::regex;

/// Sysctl implementation.
pub mod sysctl;

/// Error implementation.
pub mod error;

/// Parsers for the kernel documentation.
pub mod parsers;

/// Configuration.
pub mod config;

/// Cache manager.
pub mod cache;
