use std::fmt::Debug;

use crate::proto::error::HandlerError;
use crate::proto::handlers::fwd::ForwardPacketHandler;
use crate::proto::handlers::intent::IntentPacketHandler;
use crate::proto::handlers::setup::SetupPacketHandler;
use crate::proto::handlers::sync::SyncPacketHandler;
use crate::proto::packet::Packet;
use crate::proto::state::PacketState;

pub enum Handler {
    Encrypt(SetupPacketHandler),
    Fwd(ForwardPacketHandler),
    Sync(SyncPacketHandler),
    Intent(IntentPacketHandler),
}

pub trait PacketHandler {
    fn new_handler() -> Handler;
    fn handle(self, packet: Packet, state: PacketState) -> Result<Handler, HandlerError>;
}

impl Handler {
    pub fn handle(self, packet: Packet, state: PacketState) -> Result<Handler, HandlerError> {
        match self {
            Handler::Encrypt(handler) => handler.handle(packet, state),
            Handler::Fwd(handler) => handler.handle(packet, state),
            Handler::Sync(handler) => handler.handle(packet, state),
            Handler::Intent(handler) => handler.handle(packet, state),
        }
    }
}

impl Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Handler::Encrypt(_) => "Encrypt",
            Handler::Fwd(_) => "Fwd",
            Handler::Sync(_) => "Sync",
            Handler::Intent(_) => "Intent",
        })
    }
}
