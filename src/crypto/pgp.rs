use pgp::composed::Message;

use super::{Cipher, CypherError};

pub trait Pgp {
    fn decrypt<'a>(&self, data: &'a [u8]) -> Result<Message<'a>, CypherError>;
}

impl Pgp for Cipher {
    fn decrypt<'a>(&self, data: &'a [u8]) -> Result<Message<'a>, CypherError> {
        Message::from_bytes(data)
            .map_err(|_| CypherError::MessageDecodeError)?
            .decrypt(&self.pgp_pass, &self.pgp_private)
            .map_err(|_| CypherError::PgpDecryptError)
    }
}
