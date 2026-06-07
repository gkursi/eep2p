use uuid::Uuid;

use crate::net::message::Message;
use crate::protocol::packet::Packet;
use crate::protocol::state::PacketState;
use crate::sequence::Sequence;
use crate::sequence::common::end::EndPacketHandler;
use crate::sequence::error::SequenceError;

#[derive(Clone, Copy)]
pub struct ForwardPacketHandler {
    sequence_id: Uuid,
}

impl ForwardPacketHandler {
    pub fn new_sequence(sequence_id: Uuid) -> Sequence {
        Sequence::receive_fwd(ForwardPacketHandler { sequence_id })
    }

    pub fn handle(
        self,
        packet: Packet,
        state: PacketState,
    ) -> Result<Option<Sequence>, SequenceError> {
        match packet {
            Packet::ServerboundForwardData(_, _) => {
                state
                    .channel
                    .send(Message::HandlePacket(Packet::new_end_sequence(
                        self.sequence_id,
                    )))
                    .map_err(|_| SequenceError::IOError)?;

                // todo

                Ok(Some(EndPacketHandler::new_sequence()))
            }

            _ => Err(SequenceError::PacketOrderError),
        }
    }
}
