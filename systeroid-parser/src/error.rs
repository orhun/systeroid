use thiserror::Error as ThisError;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    /// Error that may occur due to invalid UTF-8 strings.
    #[error("non-UTF-8 string")]
    Utf8Error,
    /// Error that may occur when the capture group does not exist.
    #[error("capture group does not exist")]
    CaptureError,
    /// Error that may occur while traversing paths using a glob pattern.
    #[error("glob error: `{0}`")]
    GlobError(#[from] globwalk::GlobError),
    /// Error that may occur during the compilation of a regex.
    #[error("regex error: `{0}`")]
    RegexError(#[from] regex::Error),
}
