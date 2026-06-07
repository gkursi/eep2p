use std::fmt::Debug;

use uuid::Uuid;

use crate::net::error::ConnectionHandleError;
use crate::net::message::Message;
use crate::protocol::packet::Packet;
use crate::protocol::packet::intent::Intent;
use crate::protocol::state::PacketState;
use crate::sequence::server::setup::SetupSequence;

#[derive(Debug, Clone, Copy)]
pub enum Task {
    InitConnection,
}

impl Task {
    pub fn call(self, state: PacketState) -> Result<(), ConnectionHandleError> {
        match self {
            Self::InitConnection => {
                let handler = state
                    .handler
                    .ok_or(ConnectionHandleError::IllegalStateError)?;
                let id = Uuid::new_v4();

                handler.insert_sequence(id, SetupSequence::new_sequence(id));

                state
                    .channel
                    .send(Message::SendPacketDirect(Packet::new_server_intent(
                        id,
                        Intent::Encrypt,
                    )))
                    .map_err(|_| ConnectionHandleError::IOError)?;

                state
                    .channel
                    .send(Message::SendPacketDirect(Packet::new_server_key_exchange(
                        id,
                        state
                            .encryption
                            .x25_public
                            .ok_or(ConnectionHandleError::IllegalStateError)?,
                    )))
                    .map_err(|_| ConnectionHandleError::IOError)?;
            }
        };

        dbg!("finished init_connection");

        Ok(())
    }
}
