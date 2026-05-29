use crate::handle::handlers::intent::IntentPacketHandler;
use crate::handle::util::{
    error::HandlerError, handler::Handler, handler::PacketHandler, state::PacketState,
};
use crate::net::packet::Packet;

#[derive(Clone, Copy)]
pub struct SetupPacketHandler;

impl PacketHandler for SetupPacketHandler {
    fn new_handler() -> Handler {
        Handler::Encrypt(SetupPacketHandler {})
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
                    .derive_aes(shared.as_bytes(), b"x25519-aes256gcm-v1")
                    .map_err(|_| HandlerError::IOError)?;
            }

            _ => Err(HandlerError::PacketOrderError)?,
        };

        Ok(IntentPacketHandler::new_handler())
    }
}
