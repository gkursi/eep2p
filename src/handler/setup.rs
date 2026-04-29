use crate::handler::{Handler, PacketHandler};
use crate::con::{ConnectionState, Message};
use crate::packet::Packet;
use tokio::sync::mpsc::UnboundedSender;
use crate::encrypt::EncryptionHandler;

#[derive(Clone, Copy)]
pub enum Side {
    Client,
    Server
}

#[derive(Clone, Copy)]
pub struct SetupPacketHandler;

impl PacketHandler for SetupPacketHandler {
    fn handle(self, packet: Packet, channel: &UnboundedSender<Message>, encrypt: &mut EncryptionHandler) -> Handler {
        match packet {
            Packet::KeyPacket(public) => {
                let shared = encrypt.x25_secret
                    .take()
                    .expect("keys already exchanged")
                    .diffie_hellman(&public);

                encrypt.derive_aes(
                    shared.as_bytes(),
                    b"x25519-aes256gcm-v1"
                );
            },

            _ => panic!(""),
        };

        Handler::Setup(self)
    }
}

