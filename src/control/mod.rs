pub mod registry;
pub mod state;

use state::{Channel, Receiver, ControllerError}
use registry::Registry;

pub struct CommandHandler {
    channel: Channel,
    receiver: Option<Receiver>,
    registry: Option<Registry>,
}

impl CommandHandler {
    pub fn create_channel(&self) -> Channel {
        self.channel.clone()
    }

    pub fn start(self) {
        let receiver = self.receiver.take().expect("Start called twice");
        let registry = self.registry.take().expect("Start called twice");
        let channel = self.create_channel();

        tokio::spawn(async move || {
            if let Err(e) = Self::handle(receiver, channel) {
                println!("An error occured while handling commands: {e:?}");

                // non-recoverable, terminate all threads
                std::process::exit(1);
            }
        });
    }

    async fn handle(receiver: Receiver, registry: Registry, channel: Channel) -> Result<(), ControllerError> {
        loop {
            let Some(command) = receiver.recv().await else {
                return Ok(());
            }

            match command {
                Command::ForwardData { origin, id, data } => {

                },
            }
        };
    }
}
