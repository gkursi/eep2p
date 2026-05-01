use tokio::sync::mpsc::UnboundedSender;

use crate::connection::handler::{Handler, HandlerError};
use crate::connection::packet::Packet;
use crate::encrypt::{EncryptError, EncryptionHandler};

pub type Channel = UnboundedSender<Message>;
pub type Callback = Box<dyn FnOnce(&Channel) + Send + Sync + 'static>;

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

#[derive(Debug)]
pub enum ConnectionError {
    HandlerError(HandlerError),
    EncryptError(EncryptError),
    SerializationError,
    IOError,
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
