use thiserror::Error;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::connection::handler::{Handler, HandlerError};
use crate::connection::packet::Packet;
use crate::encrypt::{EncryptError, EncryptionHandler};

pub type Channel = UnboundedSender<Message>;
pub type Receiver = UnboundedReceiver<Message>;
pub type Callback = Box<dyn FnOnce(&Channel) -> anyhow::Result<()> + Send + Sync + 'static>;

pub enum Message {
    /// Begin key exchange
    StartExchange,
    /// Handle packet
    Packet(Packet),
    /// Send packet
    SendPacket(Packet),
    /// Close connection
    End,
    /// Close connection with an error
    EndError(ConnectionError),
}

#[derive(Debug, Error)]
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

pub struct ConnectionState {
    /// Encryption handler
    pub encryption: EncryptionHandler,
    /// Packet handler
    pub handler: Handler,

    /// Tracks key exchange
    pub sent_key: bool,
    /// Tracks key exchange
    pub recv_key: bool,

    /// Called after key exchange
    pub callback: Option<Callback>,
}
