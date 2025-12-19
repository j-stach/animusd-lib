
pub use bincode::error::{ EncodeError, DecodeError };

/// Error that occurs while processing commands.
/// These should be handled within the program loop.
#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {

    #[error("IO operation failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to Decode message: {0}")]
    Decode(#[from] bincode::error::DecodeError),

    #[error("Failed to Encode message: {0}")]
    Encode(#[from] bincode::error::EncodeError),
}


