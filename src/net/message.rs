use crate::net::error::ConnectionHandleError;
use crate::net::task::Task;
use crate::protocol::packet::OuterPacket;

#[derive(Debug, Clone)]
pub enum Message {
    HandlePacket(OuterPacket),
    /// Send a packet with encryption
    SendPacket(OuterPacket),
    /// Send a packet without encryption
    SendPacketDirect(OuterPacket),

    ExecuteTask(Task),

    /// Close connection
    End,

    /// Close connection with an error
    EndError(ConnectionHandleError),
}
