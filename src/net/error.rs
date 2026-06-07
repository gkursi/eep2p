use thiserror::Error;

use crate::crypto::CipherError;
use crate::sequence::error::SequenceError;

#[derive(Debug, Clone, Error)]
pub enum ConnectionHandleError {
    #[error("error while handling packet: {0}")]
    HandlerError(SequenceError),
    #[error("error while encrypting: {0}")]
    EncryptError(CipherError),
    #[error("invalid packet")]
    SerializationError,
    #[error("input/output error")]
    IOError,
    #[error("illegal state")]
    IllegalStateError,
}

#[derive(Debug, Error)]
pub enum ConnectionCreateError {
    #[error("input/output error: {0}")]
    IOError(std::io::Error),
    #[error("channel unexpectedly closed")]
    ChannelError,
    #[error("failed to bind address")]
    BindError,
    #[error("failed to accept listener")]
    AcceptListenerError,
}
