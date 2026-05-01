use generic_array::GenericArray;
use serde::{Deserialize, Serialize};
use typenum::consts::U12;
use uuid::Uuid;
use x25519_dalek::PublicKey;

#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    EncryptedPacket(Vec<u8>, GenericArray<u8, U12>),

    CommonKeyPacket(PublicKey),

    ServerboundIntentPacket(Intent),
    ServerboundFwdDataPacket(Uuid, Vec<u8>),
    ServerboundSyncPacket(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Intent {
    Fwd,
    Sync,
}
