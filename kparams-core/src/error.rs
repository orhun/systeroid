use thiserror::Error as ThisError;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that might occur during I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    /// Error that might occur whenever a lock is acquired.
    #[error("Thread lock error: `{0}`")]
    ThreadLockError(String),
    /// Error that might occur while parsing documents.
    #[error("parse error: `{0}`")]
    ParseError(String),
}

unsafe impl Send for Error {}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = core::result::Result<T, Error>;
