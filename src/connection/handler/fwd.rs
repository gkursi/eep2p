use crate::connection::{Channel, Handler, HandlerError, Message, Packet, PacketHandler};
use crate::encrypt::EncryptionHandler;

#[derive(Clone, Copy)]
pub struct ForwardPacketHandler;

impl PacketHandler for ForwardPacketHandler {
    fn new_handler() -> Handler {
        Handler::Fwd(ForwardPacketHandler {})
    }

    fn handle(
        self,
        packet: Packet,
        channel: &Channel,
        _encrypt: &mut EncryptionHandler,
    ) -> Result<Handler, HandlerError> {
        match packet {
            Packet::ServerboundFwdDataPacket(_, _) => {
                channel
                    .send(Message::End)
                    .map_err(|_| HandlerError::IOError)?;
            }

            _ => Err(HandlerError::PacketOrderError)?,
        };

        Ok(Handler::Fwd(self))
    }
}
