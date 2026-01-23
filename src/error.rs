
/// Error that occurs while processing commands.
/// These should be handled within the program loop.
#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {

    #[error("IO operation failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to Decode message")]
    Decode,

    #[error("Failed to Encode message")]
    Encode,
}


