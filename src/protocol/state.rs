use crate::crypto::Cipher;
use crate::net::state::{Channel, RouterChannel};
use crate::sequence::splitter::SequenceSplitter;

pub struct PacketState<'a> {
    pub origin: &'a str,
    pub channel: &'a Channel,
    pub controller: &'a RouterChannel,
    pub encryption: &'a mut Cipher,
    pub handler: Option<&'a mut SequenceSplitter>,
}
