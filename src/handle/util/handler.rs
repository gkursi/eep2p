use std::fmt::Debug;

use crate::handle::handlers::fwd::ForwardPacketHandler;
use crate::handle::handlers::intent::IntentPacketHandler;
use crate::handle::handlers::setup::SetupPacketHandler;
use crate::handle::handlers::sync::SyncPacketHandler;
use crate::handle::util::error::HandlerError;
use crate::handle::util::state::PacketState;
use crate::net::packet::Packet;

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
