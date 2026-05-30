use thiserror::Error;

use crate::{crypto::EncryptError, proto::error::HandlerError};

#[derive(Debug, Clone, Error)]
pub enum ConnectionError {
    #[error("error while handling packet: {0}")]
    HandlerError(HandlerError),
    #[error("error while encrypting: {0}")]
    EncryptError(EncryptError),
    #[error("invalid packet")]
    SerializationError,
    #[error("input/output error")]
    IOError,
    #[error("unknown error in callback")]
    CallbackError,
}
