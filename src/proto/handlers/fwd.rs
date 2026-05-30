use crate::net::{message::Message, packet::Packet};
use crate::proto::error::HandlerError;
use crate::proto::handlers::intent::IntentPacketHandler;
use crate::proto::state::PacketState;
use crate::proto::{handler::Handler, handler::PacketHandler};
use crate::router::command::state::Command;

#[derive(Clone, Copy)]
pub struct ForwardPacketHandler;

impl PacketHandler for ForwardPacketHandler {
    fn new_handler() -> Handler {
        Handler::Fwd(ForwardPacketHandler {})
    }

    fn handle(self, packet: Packet, state: PacketState) -> Result<Handler, HandlerError> {
        match packet {
            Packet::ServerboundFwdDataPacket(id, data) => {
                state
                    .channel
                    .send(Message::HandlePacket(Packet::CommonEndSequencePacket))
                    .map_err(|_| HandlerError::IOError)?;

                state
                    .controller
                    .send(Command::ForwardRequest {
                        origin: state.origin.to_string(),
                        id,
                        data,
                    })
                    .map_err(|_| HandlerError::IOError)?;

                Ok(IntentPacketHandler::new_handler())
            }

            _ => Err(HandlerError::PacketOrderError),
        }
    }
}
