use generic_array::GenericArray;
use serde::{Deserialize, Serialize};
use typenum::U12;
use uuid::Uuid;
use x25519_dalek::PublicKey;

use super::intent::Intent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OuterPacket {
    pub sequence_id: Uuid,
    content: Packet,
}

/// "Server" and "client" in this context refer to the initializer and the target of the sequence respectively
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packet {
    Encrypted(Vec<u8>, GenericArray<u8, U12>),

    // common
    EndSequence,

    // clientbound
    ClientboundKeyExchange(PublicKey),

    // serverbound
    ServerboundKeyExchange(PublicKey),
    ServerboundIntent(Intent, Uuid),
    ServerboundForwardData(Uuid, Vec<u8>),
    ServerboundSyncHosts(Vec<String>),
}

impl OuterPacket {
    pub fn unwrap(self) -> Packet {
        self.content
    }

    pub fn peek<'a>(&'a self) -> &'a Packet {
        &self.content
    }
}

impl Packet {
    pub fn new_encrypted(data: Vec<u8>, nonce: GenericArray<u8, U12>) -> OuterPacket {
        OuterPacket {
            sequence_id: Uuid::nil(),
            content: Packet::Encrypted(data, nonce),
        }
    }

    pub fn new_server_key_exchange(sequence_id: Uuid, key: PublicKey) -> OuterPacket {
        OuterPacket {
            sequence_id,
            content: Packet::ServerboundKeyExchange(key),
        }
    }

    pub fn new_client_key_exchange(sequence_id: Uuid, key: PublicKey) -> OuterPacket {
        OuterPacket {
            sequence_id,
            content: Packet::ClientboundKeyExchange(key),
        }
    }

    pub fn new_end_sequence(sequence_id: Uuid) -> OuterPacket {
        OuterPacket {
            sequence_id,
            content: Packet::EndSequence,
        }
    }

    pub fn new_server_intent(sequence_id: Uuid, intent: Intent) -> OuterPacket {
        OuterPacket {
            sequence_id,
            content: Packet::ServerboundIntent(intent, sequence_id),
        }
    }
}
