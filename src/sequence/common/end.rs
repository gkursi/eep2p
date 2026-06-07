use crate::protocol::packet::Packet;
use crate::protocol::state::PacketState;
use crate::sequence::Sequence;
use crate::sequence::error::SequenceError;

#[derive(Clone, Copy)]
pub struct EndPacketHandler;

impl EndPacketHandler {
    pub fn new_sequence() -> Sequence {
        Sequence::common_end(EndPacketHandler {})
    }

    pub fn handle(
        self,
        packet: Packet,
        _state: PacketState,
    ) -> Result<Option<Sequence>, SequenceError> {
        match packet {
            Packet::EndSequence => Ok(None),
            _ => Err(SequenceError::PacketOrderError),
        }
    }
}
