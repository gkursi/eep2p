use uuid::Uuid;

use crate::crypto::aes::Aes;
use crate::net::message::Message;
use crate::protocol::packet::Packet;
use crate::protocol::state::PacketState;
use crate::router::command::state::Command;
use crate::sequence::Sequence;
use crate::sequence::common::end::EndPacketHandler;
use crate::sequence::error::SequenceError;

#[derive(Clone, Copy)]
pub struct SetupPacketHandler {
    sequence_id: Uuid,
}

impl SetupPacketHandler {
    pub fn new_sequence(sequence_id: Uuid) -> Sequence {
        Sequence::send_encrypt(SetupPacketHandler { sequence_id })
    }

    pub fn handle(
        self,
        packet: Packet,
        state: PacketState,
    ) -> Result<Option<Sequence>, SequenceError> {
        match packet {
            Packet::ClientboundKeyExchange(public) => {
                let shared = state
                    .encryption
                    .x25_secret
                    .take()
                    .ok_or(SequenceError::PacketOrderError)?
                    .diffie_hellman(&public);

                state
                    .encryption
                    .derive_key(shared.as_bytes(), b"x25519-aes256gcm-v1")
                    .map_err(|_| SequenceError::IOError)?;

                // we can use regular Message::SendPacket from this point on

                state
                    .channel
                    .send(Message::SendPacket(Packet::new_end_sequence(
                        self.sequence_id,
                    )))
                    .map_err(|_| SequenceError::IOError)?;

                state
                    .controller
                    .send(Command::AddConnection {
                        origin: state.origin.to_string(),
                        channel: state.channel.clone(),
                    })
                    .map_err(|_| SequenceError::IOError)?;

                Ok(Some(EndPacketHandler::new_sequence()))
            }

            _ => Err(SequenceError::PacketOrderError),
        }
    }
}
