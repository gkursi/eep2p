use std::collections::HashMap;

use uuid::Uuid;

use crate::protocol::packet::OuterPacket;
use crate::protocol::state::PacketState;
use crate::sequence::Sequence;
use crate::sequence::error::SequenceError;
use crate::sequence::server::intent::IntentPacketHandler;

#[derive(Debug)]
pub struct SequenceSplitter {
    sequence_by_id: HashMap<Uuid, Sequence>,
}

impl SequenceSplitter {
    pub fn new() -> Self {
        Self {
            sequence_by_id: HashMap::new(),
        }
    }

    pub fn handle_packet(
        &mut self,
        packet: OuterPacket,
        state: PacketState,
    ) -> Result<(), SequenceError> {
        let id = packet.sequence_id;
        let handler = self
            .sequence_by_id
            .entry(id)
            .or_insert(IntentPacketHandler::new_sequence());

        let handler = handler.handle(packet.unwrap(), state)?;

        match handler {
            Some(handler) => {
                self.sequence_by_id.insert(id, handler);
            }

            None => {
                self.sequence_by_id.remove_entry(&id);
            }
        };

        Ok(())
    }

    pub fn insert_sequence(&mut self, id: Uuid, sequence: Sequence) {
        self.sequence_by_id.insert(id, sequence);
    }
}
