use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum HandlerError {
    #[error("invalid packet order")]
    PacketOrderError,
    #[error("input/output error")]
    IOError,
}
