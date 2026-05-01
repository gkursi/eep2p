use uuid::Uuid;
use thiserror::Error;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver}

pub type Channel = UnboundedSender<Command>;
type Receiver = UnboundedReciver<Command>;

#[derive(Debug, Error)]
pub enum ControllerError {

}

#[derive(Debug)]
pub enum Command {
    ForwardData {
        origin: String
        id: Uuid,
        data: Vec<u8>,
    },
}
