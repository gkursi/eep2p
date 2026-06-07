use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("channel closed unexpectedly")]
    ClosedChannelError,
}
