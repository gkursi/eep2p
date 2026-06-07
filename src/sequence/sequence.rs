use std::fmt::Debug;

use super::common::end::EndPacketHandler;
use super::error::SequenceError;
use super::server::fwd::ForwardPacketHandler;
use super::server::intent::IntentPacketHandler;
use super::server::sync::SyncPacketHandler;
use crate::protocol::packet::Packet;
use crate::protocol::state::PacketState;
use crate::sequence::client::setup::SetupPacketHandler as SendSetupPacketHandler;
use crate::sequence::server::setup::SetupSequence as ReceiveSetupPacketHandler;

#[derive(Debug)]
pub enum Sequence {
    Common(CommonSequence),
    Receive(ReceiveSequence),
    Send(SendSequence),
}

pub enum ReceiveSequence {
    Encrypt(ReceiveSetupPacketHandler),
    Forward(ForwardPacketHandler),
    Sync(SyncPacketHandler),
    Intent(IntentPacketHandler),
}

pub enum SendSequence {
    Encrypt(SendSetupPacketHandler),
}

pub enum CommonSequence {
    End(EndPacketHandler),
}

impl Sequence {
    pub fn handle(
        &self,
        packet: Packet,
        state: PacketState,
    ) -> Result<Option<Sequence>, SequenceError> {
        match self {
            Sequence::Receive(handler) => handler.handle(packet, state),
            Sequence::Send(handler) => handler.handle(packet, state),
            Sequence::Common(handler) => handler.handle(packet, state),
        }
    }

    pub fn common_end(handler: EndPacketHandler) -> Self {
        Self::Common(CommonSequence::End(handler))
    }

    pub fn receive_fwd(handler: ForwardPacketHandler) -> Self {
        Self::Receive(ReceiveSequence::Forward(handler))
    }

    pub fn receive_intent(handler: IntentPacketHandler) -> Self {
        Self::Receive(ReceiveSequence::Intent(handler))
    }

    pub fn receive_encrypt(handler: ReceiveSetupPacketHandler) -> Self {
        Self::Receive(ReceiveSequence::Encrypt(handler))
    }

    pub fn receive_sync(handler: SyncPacketHandler) -> Self {
        Self::Receive(ReceiveSequence::Sync(handler))
    }

    pub fn send_encrypt(handler: SendSetupPacketHandler) -> Self {
        Self::Send(SendSequence::Encrypt(handler))
    }
}

impl ReceiveSequence {
    pub fn handle(
        &self,
        packet: Packet,
        state: PacketState,
    ) -> Result<Option<Sequence>, SequenceError> {
        match self {
            ReceiveSequence::Encrypt(handler) => handler.handle(packet, state),
            ReceiveSequence::Forward(handler) => handler.handle(packet, state),
            ReceiveSequence::Sync(handler) => handler.handle(packet, state),
            ReceiveSequence::Intent(handler) => handler.handle(packet, state),
        }
    }
}

impl SendSequence {
    pub fn handle(
        &self,
        packet: Packet,
        state: PacketState,
    ) -> Result<Option<Sequence>, SequenceError> {
        match self {
            SendSequence::Encrypt(handler) => handler.handle(packet, state),
        }
    }
}

impl CommonSequence {
    pub fn handle(
        &self,
        packet: Packet,
        state: PacketState,
    ) -> Result<Option<Sequence>, SequenceError> {
        match self {
            CommonSequence::End(handler) => handler.handle(packet, state),
        }
    }
}

impl Debug for ReceiveSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ReceiveSequence::Encrypt(_) => "Encrypt",
            ReceiveSequence::Forward(_) => "Fwd",
            ReceiveSequence::Sync(_) => "Sync",
            ReceiveSequence::Intent(_) => "Intent",
        })
    }
}

impl Debug for SendSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SendSequence::Encrypt(_) => "Encrypt",
        })
    }
}

impl Debug for CommonSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CommonSequence::End(_) => "End",
        })
    }
}
