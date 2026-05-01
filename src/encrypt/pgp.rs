use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key};
use generic_array::GenericArray;
use hkdf::Hkdf;
use pgp::composed::{Deserializable, Message, SignedSecretKey};
use pgp::types::Password;
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey};
use crate::encrypt::{EncryptionHandler, EncryptError};

pub trait Pgp {
    fn decrypt<'a>(&self, data: &'a [u8]) -> Result<Message<'a>, EncryptError>;
}

impl Pgp for EncryptionHandler {
    fn decrypt<'a>(&self, data: &'a [u8]) -> Result<Message<'a>, EncryptError> {
        Message::from_bytes(data)
            .map_err(|_| EncryptError::MessageDecodeError)?
            .decrypt(&self.pgp_pass, &self.pgp_private)
            .map_err(|_| EncryptError::PgpDecryptError)
    }
}
