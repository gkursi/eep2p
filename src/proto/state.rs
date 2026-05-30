use crate::{
    crypto::Cipher,
    net::state::{Channel, RouterChannel},
};

pub struct PacketState<'a> {
    pub origin: &'a str,
    pub channel: &'a Channel,
    pub controller: &'a RouterChannel,
    pub encryption: &'a mut Cipher,
}
