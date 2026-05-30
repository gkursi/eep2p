use crate::crypto::aes::Aes;
use crate::net::packet::{Intent, Packet};
use crate::proto::error::HandlerError;
use crate::proto::handlers::fwd::ForwardPacketHandler;
use crate::proto::handlers::sync::SyncPacketHandler;
use crate::proto::{handler::Handler, handler::PacketHandler, state::PacketState};

#[derive(Clone, Copy)]
pub struct IntentPacketHandler;

impl PacketHandler for IntentPacketHandler {
    fn new_handler() -> Handler {
        Handler::Intent(IntentPacketHandler {})
    }

    fn handle(self, packet: Packet, state: PacketState) -> Result<Handler, HandlerError> {
        match packet {
            Packet::CommonKeyPacket(public) => {
                let shared = state
                    .encryption
                    .x25_secret
                    .take()
                    .expect("keys already exchanged")
                    .diffie_hellman(&public);

                state
                    .encryption
                    .derive_key(shared.as_bytes(), b"x25519-aes256gcm-v1")
                    .map_err(|_| HandlerError::IOError)?;
            }

            Packet::ServerboundIntentPacket(intent) => {
                return Ok(match intent {
                    Intent::Fwd => ForwardPacketHandler::new_handler(),
                    Intent::Sync => SyncPacketHandler::new_handler(),
                });
            }

            _ => Err(HandlerError::PacketOrderError)?,
        };

        Ok(Handler::Intent(self))
    }
}
