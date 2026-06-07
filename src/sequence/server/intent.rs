use super::fwd::ForwardPacketHandler;
use super::setup::SetupSequence;
use super::sync::SyncPacketHandler;
use crate::protocol::packet::Packet;
use crate::protocol::packet::intent::Intent;
use crate::protocol::state::PacketState;
use crate::sequence::Sequence;
use crate::sequence::error::SequenceError;

#[derive(Clone, Copy)]
pub struct IntentPacketHandler;

impl IntentPacketHandler {
    pub fn new_sequence() -> Sequence {
        Sequence::receive_intent(IntentPacketHandler {})
    }

    pub fn handle(
        self,
        packet: Packet,
        state: PacketState,
    ) -> Result<Option<Sequence>, SequenceError> {
        match packet {
            Packet::ServerboundIntent(intent, id) => match intent {
                Intent::Encrypt => {
                    if state.encryption.has_aes() {
                        Err(SequenceError::PacketOrderError)
                    } else {
                        Ok(Some(SetupSequence::new_sequence(id)))
                    }
                }

                Intent::Forward => Ok(Some(ForwardPacketHandler::new_sequence(id))),
                Intent::Sync => Ok(Some(SyncPacketHandler::new_sequence(id))),
            },

            _ => Err(SequenceError::PacketOrderError),
        }
    }
}
