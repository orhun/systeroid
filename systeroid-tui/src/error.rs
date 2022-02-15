use thiserror::Error as ThisError;

/// Custom error type.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    IoError(#[from] std::io::Error),
    /// Error that may occur while receiving messages from the channel.
    #[error("channel receive error: `{0}`")]
    ChannelReceiveError(#[from] std::sync::mpsc::RecvError),
    /// Error that may occur while getting/setting the contents of the clipboard.
    #[error("clipboard error: `{0}`")]
    ClipboardError(String),
    /// Error that may occur while parsing a color.
    #[error(transparent)]
    ColorParseError(#[from] colorsys::ParseError),
    /// Error that may occur in the core library.
    #[error(transparent)]
    SysctlError(#[from] systeroid_core::error::Error),
}

/// Type alias for the standard [`Result`] type.
pub type Result<T> = core::result::Result<T, Error>;
