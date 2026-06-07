use uuid::Uuid;

use crate::net::message::Message;
use crate::protocol::packet::Packet;
use crate::protocol::state::PacketState;
use crate::sequence::Sequence;
use crate::sequence::error::SequenceError;

#[derive(Clone, Copy)]
pub struct SyncPacketHandler {
    sequence_id: Uuid,
}

impl SyncPacketHandler {
    pub fn new_sequence(sequence_id: Uuid) -> Sequence {
        Sequence::receive_sync(SyncPacketHandler { sequence_id })
    }

    pub fn handle(
        self,
        packet: Packet,
        state: PacketState,
    ) -> Result<Option<Sequence>, SequenceError> {
        match packet {
            Packet::ServerboundSyncHosts(_) => {
                // todo implement

                state
                    .channel
                    .send(Message::HandlePacket(Packet::new_end_sequence(
                        self.sequence_id,
                    )))
                    .map_err(|_| SequenceError::IOError)?;
            }

            _ => Err(SequenceError::PacketOrderError)?,
        };

        Ok(None)
    }
}
