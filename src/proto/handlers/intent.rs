use crate::proto::error::HandlerError;
use crate::proto::handlers::fwd::ForwardPacketHandler;
use crate::proto::handlers::sync::SyncPacketHandler;
use crate::proto::packet::Packet;
use crate::proto::packet::intent::Intent;
use crate::proto::{handler::Handler, handler::PacketHandler, state::PacketState};

#[derive(Clone, Copy)]
pub struct IntentPacketHandler;

impl PacketHandler for IntentPacketHandler {
    fn new_handler() -> Handler {
        Handler::Intent(IntentPacketHandler {})
    }

    fn handle(self, packet: Packet, _state: PacketState) -> Result<Handler, HandlerError> {
        match packet {
            Packet::ServerboundIntent(intent) => Ok(match intent {
                Intent::Fwd => ForwardPacketHandler::new_handler(),
                Intent::Sync => SyncPacketHandler::new_handler(),
            }),

            _ => Err(HandlerError::PacketOrderError),
        }
    }
}
