use thiserror::Error;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use uuid::Uuid;

use crate::router::command::state::Command;

pub type Channel = UnboundedSender<Command>;
pub type Receiver = UnboundedReceiver<Command>;

#[derive(Debug, Error)]
pub enum ControllerError {
    #[error("channel closed unexpectedly")]
    ClosedChannelError,
}

#[derive(Debug, Clone)]
pub struct ActiveConnection {
    // target: String,
    // id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FwdConnection {
    pub origin: String,
    pub origin_id: Uuid,
}
