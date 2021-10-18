use thiserror::Error as ThisError;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    /// Error that may occur whenever a lock is acquired.
    #[error("Thread lock error: `{0}`")]
    ThreadLockError(String),
    /// Error that may occur while parsing documents.
    #[error("parse error: `{0}`")]
    ParseError(String),
    /// Error that may occur while handling sysctl operations.
    #[error("sysctl error: `{0}`")]
    SysctlError(#[from] sysctl::SysctlError),
}

unsafe impl Send for Error {}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = core::result::Result<T, Error>;
