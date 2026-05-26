pub mod fwd;
pub mod setup;
pub mod sync;

use std::fmt::Debug;

use fwd::ForwardPacketHandler;
use setup::SetupPacketHandler;
use sync::SyncPacketHandler;
use thiserror::Error;

use crate::connection::{Channel, Packet};
use crate::encrypt::EncryptionHandler;

type ControllerChannel = crate::control::state::Channel;

#[derive(Debug, Error, Clone)]
pub enum HandlerError {
    #[error("invalid packet order")]
    PacketOrderError,
    #[error("input/output error")]
    IOError,
}

pub enum Handler {
    Setup(SetupPacketHandler),
    Fwd(ForwardPacketHandler),
    Sync(SyncPacketHandler),
}

pub struct PacketState {
    pub controller: ControllerChannel,
}

pub trait PacketHandler {
    fn new_handler() -> Handler;

    fn handle(
        self,
        packet: Packet,
        channel: &Channel,
        encrypt: &mut EncryptionHandler,
    ) -> Result<Handler, HandlerError>;
}

impl Handler {
    pub fn handle(
        self,
        packet: Packet,
        channel: &Channel,
        encrypt: &mut EncryptionHandler,
    ) -> Result<Handler, HandlerError> {
        match self {
            Handler::Setup(handler) => handler.handle(packet, channel, encrypt),
            Handler::Fwd(handler) => handler.handle(packet, channel, encrypt),
            Handler::Sync(handler) => handler.handle(packet, channel, encrypt),
        }
    }
}

impl Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Handler::Setup(_) => "Setup",
            Handler::Fwd(_) => "Fwd",
            Handler::Sync(_) => "Sync",
        })
    }
}
