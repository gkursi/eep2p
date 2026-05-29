use crate::handle::handlers::intent::IntentPacketHandler;
use crate::handle::util::state::PacketState;
use crate::{
    handle::util::error::HandlerError, handle::util::handler::Handler,
    handle::util::handler::PacketHandler, net::packet::Packet, net::state::Message,
};

#[derive(Clone, Copy)]
pub struct SyncPacketHandler;

impl PacketHandler for SyncPacketHandler {
    fn new_handler() -> Handler {
        Handler::Sync(SyncPacketHandler {})
    }

    fn handle(self, packet: Packet, state: PacketState) -> Result<Handler, HandlerError> {
        match packet {
            Packet::ServerboundSyncPacket(_) => {
                // todo

                state
                    .channel
                    .send(Message::Packet(Packet::CommonEndSequencePacket))
                    .map_err(|_| HandlerError::IOError)?;
            }

            _ => Err(HandlerError::PacketOrderError)?,
        };

        Ok(IntentPacketHandler::new_handler())
    }
}
