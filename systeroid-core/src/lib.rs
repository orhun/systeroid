//! systeroid-core

#![warn(missing_docs, clippy::unwrap_used)]

/// Linux kernel documentation.
pub mod docs;

/// Linux kernel parameter handler.
pub mod sysctl;

/// File reader.
pub mod reader;

/// Error handler.
pub mod error;
