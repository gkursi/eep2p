use crate::connection::Channel;
use crate::connection::handler::fwd::ForwardPacketHandler;
use crate::connection::handler::sync::SyncPacketHandler;
use crate::connection::handler::{Handler, HandlerError, PacketHandler};
use crate::connection::packet::{Intent, Packet};
use crate::encrypt::EncryptionHandler;

#[derive(Clone, Copy)]
pub struct SetupPacketHandler;

impl PacketHandler for SetupPacketHandler {
    fn new_handler() -> Handler {
        Handler::Setup(SetupPacketHandler {})
    }

    fn handle(
        self,
        packet: Packet,
        _channel: &Channel,
        encrypt: &mut EncryptionHandler,
    ) -> Result<Handler, HandlerError> {
        match packet {
            Packet::CommonKeyPacket(public) => {
                let shared = encrypt
                    .x25_secret
                    .take()
                    .expect("keys already exchanged")
                    .diffie_hellman(&public);

                encrypt
                    .derive_aes(shared.as_bytes(), b"x25519-aes256gcm-v1")
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

        Ok(Handler::Setup(self))
    }
}
