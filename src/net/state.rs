use std::fmt::Debug;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::crypto::Cipher;
use crate::net::message::Message;
use crate::sequence::splitter::SequenceSplitter;

pub type RouterChannel = crate::router::connection::Channel;
pub type Channel = UnboundedSender<Message>;
pub type Receiver = UnboundedReceiver<Message>;

pub struct ConnectionState {
    /// Encryption handler
    pub encryption: Cipher,
    /// Packet handler
    pub handler: SequenceSplitter,
}

impl Debug for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection({:?})", self.handler)
    }
}
