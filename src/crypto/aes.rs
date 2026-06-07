use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key};
use generic_array::GenericArray;
use hkdf::Hkdf;
use sha2::Sha256;
use typenum::consts::U12;

use super::{Cipher, CipherError};

pub trait Aes {
    fn encrypt(&self, data: &[u8]) -> Result<(GenericArray<u8, U12>, Vec<u8>), CipherError>;

    fn decrypt(&self, data: &[u8], nonce: GenericArray<u8, U12>) -> Result<Vec<u8>, CipherError>;

    fn derive_key(&mut self, secret: &[u8; 32], transcript: &[u8]) -> Result<(), CipherError>;
}

impl Aes for Cipher {
    fn encrypt(
        &self,
        data: &[u8],
    ) -> Result<(GenericArray<u8, typenum::consts::U12>, Vec<u8>), CipherError> {
        let Some(ref cipher) = self.aes_cipher else {
            return Err(CipherError::EarlyCallError);
        };

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let encrypted = cipher
            .encrypt(&nonce, data)
            .map_err(|_| CipherError::AesEncryptError)?;

        Ok((nonce, encrypted))
    }

    fn decrypt(
        &self,
        data: &[u8],
        nonce: GenericArray<u8, typenum::consts::U12>,
    ) -> Result<Vec<u8>, CipherError> {
        let Some(ref cipher) = self.aes_cipher else {
            return Err(CipherError::EarlyCallError);
        };

        cipher
            .decrypt(&nonce, data)
            .map_err(|_| CipherError::AesDecryptError)
    }

    fn derive_key(&mut self, secret: &[u8; 32], transcript: &[u8]) -> Result<(), CipherError> {
        let mut key = [0u8; 32];
        let hk = Hkdf::<Sha256>::new(Some(transcript), secret);

        hk.expand(b"aes-256-gcm key", &mut key)
            .map_err(|_| CipherError::KeyDeriveError)?;
        self.aes_cipher = Some(Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key)));

        Ok(())
    }
}
