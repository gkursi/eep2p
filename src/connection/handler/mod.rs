pub mod fwd;
pub mod setup;
pub mod sync;

use fwd::ForwardPacketHandler;
use setup::SetupPacketHandler;
use sync::SyncPacketHandler;

use crate::connection::{Channel, Packet};
use crate::encrypt::EncryptionHandler;

#[derive(Debug)]
pub enum HandlerError {
    PacketOrderError,
    IOError,
}

pub enum Handler {
    Setup(SetupPacketHandler),
    Fwd(ForwardPacketHandler),
    Sync(SyncPacketHandler),
}

pub trait PacketHandler {
    fn new_handler() -> Handler;

    fn handle(
        self,
        packet: Packet,
        channel: &Channel,
        encrypt: &mut EncryptionHandler,
    ) -> Result<Handler, HandlerError>;
}

impl Handler {
    pub fn handle(
        self,
        packet: Packet,
        channel: &Channel,
        encrypt: &mut EncryptionHandler,
    ) -> Result<Handler, HandlerError> {
        match self {
            Handler::Setup(handler) => handler.handle(packet, channel, encrypt),
            Handler::Fwd(handler) => handler.handle(packet, channel, encrypt),
            Handler::Sync(handler) => handler.handle(packet, channel, encrypt),
        }
    }
}
