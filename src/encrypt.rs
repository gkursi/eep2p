use x25519_dalek::{EphemeralSecret, PublicKey};
use generic_array::GenericArray;
use hkdf::Hkdf;
use sha2::Sha256;
use aes_gcm::{
    aead::{
        Aead,
        AeadCore,
        KeyInit,
        OsRng,
    },
    Aes256Gcm, 
    Key,
};

pub struct EncryptionHandler {
    pub cipher: Option<Aes256Gcm>,
    pub x25_secret: Option<EphemeralSecret>,
    pub x25_public: Option<PublicKey>,
}

impl EncryptionHandler {
    pub fn new() -> Self {
        let secret = EphemeralSecret::random();
        let public = PublicKey::from(&secret);

        Self {
            cipher: None,
            x25_secret: Some(secret),
            x25_public: Some(public),
        }
    }

    pub fn derive_aes(&mut self, secret: &[u8; 32], transcript: &[u8]) {
        let mut key = [0u8; 32];
        let hk = Hkdf::<Sha256>::new(
            Some(transcript),
            secret
        );
        
        hk.expand(b"aes-256-gcm key", &mut key).unwrap();
        
        self.cipher = Some(
            Aes256Gcm::new(
                Key::<Aes256Gcm>::from_slice(
                    &key
                )
            )
        );
    }

    pub fn encrypt(&self, data: &[u8]) -> (GenericArray<u8, typenum::consts::U12>, Vec<u8>) {
        let Some(ref cipher) = self.cipher else {
            panic!("Encrypt called before key exchange");
        };

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let encrypted = cipher.encrypt(&nonce, data)
            .expect("failed to encrypt");
        
        (nonce, encrypted)
    }

    pub fn decrypt(&self, data: &[u8], nonce: GenericArray<u8, typenum::consts::U12>) -> Vec<u8> {
        let Some(ref cipher) = self.cipher else {
            panic!("Decrypt called before key exchange");
        };

        let decrypted = cipher.decrypt(&nonce, data)
            .expect("failed to encrypt");
        
        decrypted    
    }
}
