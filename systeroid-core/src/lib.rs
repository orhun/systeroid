//! systeroid-core

#![warn(missing_docs, clippy::unwrap_used)]

#[macro_use]
extern crate lazy_static;

/// Kernel parameter handler.
pub mod sysctl;

/// Error implementation.
pub mod error;

/// Parsers for the kernel documentation.
pub mod parsers;
