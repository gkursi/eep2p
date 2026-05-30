pub mod aes;
pub mod pgp;

use ::pgp::composed::{Deserializable, SignedSecretKey};
use ::pgp::types::Password;
use aes_gcm::Aes256Gcm;
use thiserror::Error;
use x25519_dalek::{EphemeralSecret, PublicKey};

use crate::config::Config;

#[derive(Clone)]
pub struct CipherKeys {
    pgp_private_key: SignedSecretKey,
    pgp_passphrase: String,
}

#[derive(Debug, Error, Clone)]
pub enum EncryptError {
    #[error("failed to derive key")]
    KeyDeriveError,
    #[error("failed to decode pgp message")]
    MessageDecodeError,
    #[error("failed to decrypt pgp message")]
    PgpDecryptError,
    #[error("failed to encrypt with aes")]
    AesEncryptError,
    #[error("failed to decrypt with aes")]
    AesDecryptError,
}

impl CipherKeys {
    pub fn from(passwd: &str, config: &Config) -> Self {
        let (key, _) = SignedSecretKey::from_armor_file(&config.pgp.private)
            .expect("Invalid or corrupted private key");

        Self {
            pgp_private_key: key,
            pgp_passphrase: passwd.to_string(),
        }
    }
}

pub struct Cipher {
    cipher: Option<Aes256Gcm>,
    pub x25_secret: Option<EphemeralSecret>,
    pub x25_public: Option<PublicKey>,
    pgp_private: SignedSecretKey,
    pgp_pass: Password,
}

impl Cipher {
    pub fn from(keys: &CipherKeys) -> Self {
        let secret = EphemeralSecret::random();
        let public = PublicKey::from(&secret);

        Self {
            cipher: None,
            x25_secret: Some(secret),
            x25_public: Some(public),
            pgp_private: keys.pgp_private_key.clone(),
            pgp_pass: keys.pgp_passphrase.clone().into(),
        }
    }
}
