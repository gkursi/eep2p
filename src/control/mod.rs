pub mod registry;

use uuid::Uuid;
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver}

pub type Channel = UnboundedSender<Command>;
type Receiver = UnboundedReciver<Command>;

pub enum Command {
    ForwardData(Uuid, Vec<u8>),
}

pub struct CommandHandler {
    receiver: Option<Receiver>,
}

impl CommandHandler {
    pub fn start(self) {
        let receiver = self.receiver.take().expect("Start called twice");

        tokio::spawn(async move || {

        });
    }

    async fn handle(receiver: Receiver) -> anyhow::Result<()> {
        loop {

        }
    }
}
