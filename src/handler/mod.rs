pub mod setup;
pub mod fwd;
pub mod sync;

use crate::con::{ConnectionState, Message};
use crate::packet::Packet;
use crate::encrypt::EncryptionHandler;
use tokio::sync::mpsc::UnboundedSender;
use setup::SetupPacketHandler;

pub enum Handler {
    Setup(SetupPacketHandler),
}

pub trait PacketHandler {
    fn handle(
        self,
        packet: Packet,
        channel: &UnboundedSender<Message>,
        encrypt: &mut EncryptionHandler
    ) -> Handler;
}

impl Handler {
    pub fn handle(self, packet: Packet, channel: &UnboundedSender<Message>, encrypt: &mut EncryptionHandler) -> Handler {
        match self {
            Handler::Setup(handler) => handler.handle(packet, channel, encrypt),
        }
    }
}
