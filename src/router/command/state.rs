use uuid::Uuid;

use crate::net::message::Message;

#[derive(Debug, Clone)]
pub enum Command {
    ForwardRequest {
        origin: String,
        id: Uuid,
        data: Vec<u8>,
    },

    SendMessage {
        target: String,
        msg: Message,
    },
}
