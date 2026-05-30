use std::fmt::Debug;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::crypto::Cipher;
use crate::net::message::Message;
use crate::proto::handler::Handler;

pub type RouterChannel = crate::router::connection::Channel;
pub type Channel = UnboundedSender<Message>;
pub type Receiver = UnboundedReceiver<Message>;
pub type Callback = Box<dyn FnOnce(&Channel) -> anyhow::Result<()> + Send + Sync + 'static>;

pub struct ConnectionState {
    /// Encryption handler
    pub encryption: Cipher,
    /// Packet handler
    pub handler: Handler,

    /// Tracks key exchange
    pub sent_key: bool,
    /// Tracks key exchange
    pub recv_key: bool,

    /// Called after key exchange
    pub callback: Option<Callback>,
}

impl Debug for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Connection({:?}, {:?}, {:?})",
            self.handler, self.sent_key, self.recv_key
        )
    }
}
