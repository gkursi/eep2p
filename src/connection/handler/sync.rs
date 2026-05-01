use crate::connection::{Channel, Handler, HandlerError, Message, Packet, PacketHandler};
use crate::encrypt::EncryptionHandler;

#[derive(Clone, Copy)]
pub struct SyncPacketHandler;

impl PacketHandler for SyncPacketHandler {
    fn new_handler() -> Handler {
        Handler::Sync(SyncPacketHandler {})
    }

    fn handle(
        self,
        packet: Packet,
        channel: &Channel,
        _encrypt: &mut EncryptionHandler,
    ) -> Result<Handler, HandlerError> {
        match packet {
            Packet::ServerboundSyncPacket(_) => {
                channel
                    .send(Message::End)
                    .map_err(|_| HandlerError::IOError)?;
                // TODO add hosts to registry
            }

            _ => Err(HandlerError::PacketOrderError)?,
        };

        Ok(Handler::Sync(self))
    }
}
