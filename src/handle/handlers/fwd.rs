use crate::control::state::Command;
use crate::handle::handlers::intent::IntentPacketHandler;
use crate::handle::util::state::PacketState;
use crate::handle::util::{error::HandlerError, handler::Handler, handler::PacketHandler};
use crate::net::{packet::Packet, state::Message};

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
                    .send(Message::Packet(Packet::CommonEndSequencePacket))
                    .map_err(|_| HandlerError::IOError)?;

                state
                    .controller
                    .send(Command::ForwardData {
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
