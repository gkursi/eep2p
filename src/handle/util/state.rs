use crate::{
    encrypt::EncryptionHandler,
    net::state::{Channel, ControllerChannel},
};

pub struct PacketState<'a> {
    pub origin: &'a str,
    pub channel: &'a Channel,
    pub controller: &'a ControllerChannel,
    pub encryption: &'a mut EncryptionHandler,
}
