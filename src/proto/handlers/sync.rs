use crate::proto::handlers::intent::IntentPacketHandler;
use crate::proto::state::PacketState;
use crate::{
    net::message::Message, proto::error::HandlerError, proto::handler::Handler,
    proto::handler::PacketHandler, proto::packet::Packet,
};

#[derive(Clone, Copy)]
pub struct SyncPacketHandler;

impl PacketHandler for SyncPacketHandler {
    fn new_handler() -> Handler {
        Handler::Sync(SyncPacketHandler {})
    }

    fn handle(self, packet: Packet, state: PacketState) -> Result<Handler, HandlerError> {
        match packet {
            Packet::ServerboundSyncHosts(_) => {
                // todo

                state
                    .channel
                    .send(Message::HandlePacket(Packet::EndSequence))
                    .map_err(|_| HandlerError::IOError)?;
            }

            _ => Err(HandlerError::PacketOrderError)?,
        };

        Ok(IntentPacketHandler::new_handler())
    }
}
