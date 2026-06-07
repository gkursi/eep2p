use std::fmt::Debug;

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum SequenceError {
    #[error("invalid packet order")]
    PacketOrderError,
    #[error("input/output error")]
    IOError,
}
