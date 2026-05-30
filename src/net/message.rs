use crate::net::error::ConnectionError;
use crate::net::packet::Packet;

#[derive(Debug, Clone)]
pub enum Message {
    HandlePacket(Packet),
    SendPacket(Packet),

    /// Begin key exchange
    StartExchange,

    /// Close connection
    End,

    /// Close connection with an error
    EndError(ConnectionError),
}
