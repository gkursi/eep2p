use serde::{Serialize, Deserialize};
use x25519_dalek::PublicKey;
use generic_array::GenericArray;

#[derive(Debug, Serialize, Deserialize)]
pub enum Packet {
    KeyPacket(PublicKey),
    EncryptedPacket(Vec<u8>, GenericArray<u8, typenum::consts::U12>),
    
    ServerboundIntentPacket(Intent),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Intent {
    Fwd,
    Sync,
}
