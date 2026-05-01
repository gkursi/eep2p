pub mod aes;
pub mod pgp;

use ::pgp::composed::{Deserializable, SignedSecretKey};
use ::pgp::types::Password;
use aes_gcm::aead::KeyInit;
use aes_gcm::{Aes256Gcm, Key};
use hkdf::Hkdf;
use sha2::Sha256;
use thiserror::Error;
use x25519_dalek::{EphemeralSecret, PublicKey};

use crate::Config;

#[derive(Clone)]
pub struct GlobalKeys {
    pgp_private: SignedSecretKey,
    pgp_pass: String,
}

#[derive(Debug, Error)]
pub enum EncryptError {
    #[error("failed to derive key")]
    KeyDeriveError,
    #[error("failed to decode pgp-encrypted message")]
    MessageDecodeError,
    #[error("failed to decrypt pgp message")]
    PgpDecryptError,
    #[error("failed to encrypt with aes")]
    AesEncryptError,
    #[error("failed to decrypt with aes")]
    AesDecryptError,
}

impl GlobalKeys {
    pub fn from(passwd: &str, config: &Config) -> Self {
        let (key, _) = SignedSecretKey::from_armor_file(&config.pgp_private)
            .expect("Invalid or corrupted private key");

        Self {
            pgp_private: key,
            pgp_pass: passwd.to_string(),
        }
    }
}

pub struct EncryptionHandler {
    cipher: Option<Aes256Gcm>,
    pub x25_secret: Option<EphemeralSecret>,
    pub x25_public: Option<PublicKey>,
    pgp_private: SignedSecretKey,
    pgp_pass: Password,
}

impl EncryptionHandler {
    pub fn from(keys: &GlobalKeys) -> Self {
        let secret = EphemeralSecret::random();
        let public = PublicKey::from(&secret);

        Self {
            cipher: None,
            x25_secret: Some(secret),
            x25_public: Some(public),
            pgp_private: keys.pgp_private.clone(),
            pgp_pass: keys.pgp_pass.clone().into(),
        }
    }

    pub fn derive_aes(&mut self, secret: &[u8; 32], transcript: &[u8]) -> Result<(), EncryptError> {
        let mut key = [0u8; 32];
        let hk = Hkdf::<Sha256>::new(Some(transcript), secret);

        hk.expand(b"aes-256-gcm key", &mut key)
            .map_err(|_| EncryptError::KeyDeriveError)?;
        self.cipher = Some(Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key)));

        Ok(())
    }
}
