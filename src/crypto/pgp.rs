use pgp::composed::Message;

use super::{Cipher, CipherError};

pub trait Pgp {
    fn decrypt<'a>(&self, data: &'a [u8]) -> Result<Message<'a>, CipherError>;
}

impl Pgp for Cipher {
    fn decrypt<'a>(&self, data: &'a [u8]) -> Result<Message<'a>, CipherError> {
        Message::from_bytes(data)
            .map_err(|_| CipherError::MessageDecodeError)?
            .decrypt(&self.pgp_pass, &self.pgp_private)
            .map_err(|_| CipherError::PgpDecryptError)
    }
}
