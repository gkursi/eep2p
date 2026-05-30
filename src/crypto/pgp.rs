use pgp::composed::Message;

use super::{Cipher, EncryptError};

pub trait Pgp {
    fn decrypt<'a>(&self, data: &'a [u8]) -> Result<Message<'a>, EncryptError>;
}

impl Pgp for Cipher {
    fn decrypt<'a>(&self, data: &'a [u8]) -> Result<Message<'a>, EncryptError> {
        Message::from_bytes(data)
            .map_err(|_| EncryptError::MessageDecodeError)?
            .decrypt(&self.pgp_pass, &self.pgp_private)
            .map_err(|_| EncryptError::PgpDecryptError)
    }
}
