use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, AeadCore, OsRng};
use generic_array::GenericArray;
use typenum::consts::U12;

use crate::encrypt::{EncryptError, EncryptionHandler};

pub trait Aes {
    fn encrypt(&self, data: &[u8]) -> Result<(GenericArray<u8, U12>, Vec<u8>), EncryptError>;

    fn decrypt(&self, data: &[u8], nonce: GenericArray<u8, U12>) -> Result<Vec<u8>, EncryptError>;
}

impl Aes for EncryptionHandler {
    fn encrypt(
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

    fn decrypt(
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
}
