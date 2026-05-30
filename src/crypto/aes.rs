use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key};
use generic_array::GenericArray;
use hkdf::Hkdf;
use sha2::Sha256;
use typenum::consts::U12;

use super::{Cipher, CypherError};

pub trait Aes {
    fn encrypt(&self, data: &[u8]) -> Result<(GenericArray<u8, U12>, Vec<u8>), CypherError>;

    fn decrypt(&self, data: &[u8], nonce: GenericArray<u8, U12>) -> Result<Vec<u8>, CypherError>;

    fn derive_key(&mut self, secret: &[u8; 32], transcript: &[u8]) -> Result<(), CypherError>;
}

impl Aes for Cipher {
    fn encrypt(
        &self,
        data: &[u8],
    ) -> Result<(GenericArray<u8, typenum::consts::U12>, Vec<u8>), CypherError> {
        let Some(ref cipher) = self.cipher else {
            return Err(CypherError::EarlyCallError);
        };

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let encrypted = cipher
            .encrypt(&nonce, data)
            .map_err(|_| CypherError::AesEncryptError)?;

        Ok((nonce, encrypted))
    }

    fn decrypt(
        &self,
        data: &[u8],
        nonce: GenericArray<u8, typenum::consts::U12>,
    ) -> Result<Vec<u8>, CypherError> {
        let Some(ref cipher) = self.cipher else {
            return Err(CypherError::EarlyCallError);
        };

        cipher
            .decrypt(&nonce, data)
            .map_err(|_| CypherError::AesDecryptError)
    }

    fn derive_key(&mut self, secret: &[u8; 32], transcript: &[u8]) -> Result<(), CypherError> {
        let mut key = [0u8; 32];
        let hk = Hkdf::<Sha256>::new(Some(transcript), secret);

        hk.expand(b"aes-256-gcm key", &mut key)
            .map_err(|_| CypherError::KeyDeriveError)?;
        self.cipher = Some(Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key)));

        Ok(())
    }
}
