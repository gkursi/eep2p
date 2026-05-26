use thiserror::Error;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

pub type Channel = UnboundedSender<Command>;
pub type Receiver = UnboundedReceiver<Command>;

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error("channel closed unexpectedly")]
    ClosedChannelError,
}

#[derive(Debug, Clone)]
pub enum Command {
    ForwardData {
        origin: String,
        id: Uuid,
        data: Vec<u8>,
    },
}

#[derive(Debug, Clone)]
pub struct Connection {
    // target: String,
    // id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FwdConnection {
    pub origin: String,
    pub origin_id: Uuid,
}
