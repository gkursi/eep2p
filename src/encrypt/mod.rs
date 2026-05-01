use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key};
use generic_array::GenericArray;
use hkdf::Hkdf;
use pgp::composed::{Deserializable, Message, SignedSecretKey};
use pgp::types::Password;
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey};

use crate::Config;

#[derive(Clone)]
pub struct GlobalKeys {
    pgp_private: SignedSecretKey,
    pgp_pass: String,
}

#[derive(Debug)]
pub enum EncryptError {
    KeyDeriveError,
    MessageDecodeError,
    PgpDecryptError,
    AesEncryptError,
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

    pub fn encrypt(
        &self,
        data: &[u8],
    ) -> Result<(GenericArray<u8, typenum::consts::U12>, Vec<u8>), EncryptError> {
        let Some(ref cipher) = self.cipher else {
            panic!("Encrypt called before key exchange");
        };

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let encrypted = cipher
            .encrypt(&nonce, data)
            .map_err(|_| EncryptError::AesEncryptError)?;

        Ok((nonce, encrypted))
    }

    pub fn decrypt(
        &self,
        data: &[u8],
        nonce: GenericArray<u8, typenum::consts::U12>,
    ) -> Result<Vec<u8>, EncryptError> {
        let Some(ref cipher) = self.cipher else {
            panic!("Decrypt called before key exchange");
        };

        cipher
            .decrypt(&nonce, data)
            .map_err(|_| EncryptError::AesDecryptError)
    }

    pub fn decrypt_pgp<'a>(&self, data: &'a [u8]) -> Result<Message<'a>, EncryptError> {
        let message = Message::from_bytes(data).map_err(|_| EncryptError::MessageDecodeError)?;
        message
            .decrypt(&self.pgp_pass, &self.pgp_private)
            .map_err(|_| EncryptError::PgpDecryptError)
    }
}
