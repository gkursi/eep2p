pub mod registry;
pub mod state;

use crate::config::data::Hosts;
use crate::encrypt::{EncryptionHandler, GlobalKeys};
use registry::Registry;
use state::{Channel, Command, ControllerError, FwdConnection, Receiver};
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;

const LIFETIME: u64 = 10 * 60;

pub struct CommandHandler {
    channel: Channel,
    receiver: Option<Receiver>,
    registry: Option<Registry>,
    encrypt: Option<EncryptionHandler>,
}

impl CommandHandler {
    pub fn new(keys: &GlobalKeys, hosts: Hosts) -> Self {
        let (channel, events) = mpsc::unbounded_channel();

        CommandHandler {
            channel,
            receiver: Some(events),
            registry: Some(Registry::new(hosts)),
            encrypt: Some(EncryptionHandler::from(keys)),
        }
    }

    pub fn create_channel(&self) -> Channel {
        self.channel.clone()
    }

    pub fn start(&mut self) {
        let receiver = self.receiver.take().expect("Start called twice");
        let registry = self.registry.take().expect("Start called twice");
        let encrypt = self.encrypt.take().expect("Start called twice");
        let channel = self.create_channel();

        tokio::spawn(async move {
            if let Err(e) = Self::handle(receiver, registry, channel, encrypt).await {
                println!("An error occured while handling commands: {e:?}");

                // non-recoverable, terminate all threads
                std::process::exit(1);
            }
        });
    }

    async fn handle(
        mut receiver: Receiver,
        registry: Registry,
        _channel: Channel,
        _encrypt: EncryptionHandler,
    ) -> Result<(), ControllerError> {
        loop {
            let Some(command) = receiver.recv().await else {
                return Err(ControllerError::ClosedChannelError);
            };

            match command {
                Command::ForwardData { origin, id, .. } => {
                    let new_id = Uuid::new_v4();
                    let fwd = FwdConnection {
                        origin,
                        origin_id: id,
                    };

                    registry
                        .fwd_connections
                        .insert(new_id, fwd, Duration::from_secs(LIFETIME));

                    // for host in registry.hosts {
                    // connection::message_addr()
                    // }

                    // iterate trough hosts:
                    // send to each
                    // add to integrity
                }
            };
        }
    }
}
