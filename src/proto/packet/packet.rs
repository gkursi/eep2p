use generic_array::GenericArray;
use serde::{Deserialize, Serialize};
use typenum::U12;
use uuid::Uuid;
use x25519_dalek::PublicKey;

use super::intent::Intent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packet {
    Encrypted(Vec<u8>, GenericArray<u8, U12>),

    KeyExchange(PublicKey),
    EndSequence,

    ServerboundIntent(Intent),
    ServerboundForwardData(Uuid, Vec<u8>),
    ServerboundSyncHosts(Vec<String>),
}
