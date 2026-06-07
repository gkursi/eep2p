use tokio::sync::mpsc;

use super::state::Command;
use crate::config::data::hosts::Hosts;
use crate::crypto::CipherKeys;
use crate::net::message::Message;
use crate::net::util;
use crate::router::connection::{Channel, Receiver};
use crate::router::error::RouterError;
use crate::router::registry::Registry;

pub struct CommandHandler {
    channel: Channel,
    receiver: Option<Receiver>,
    registry: Option<Registry>,
    keys: Option<CipherKeys>,
}

impl CommandHandler {
    pub fn new(keys: &CipherKeys, hosts: Hosts) -> Self {
        let (channel, events) = mpsc::unbounded_channel();

        CommandHandler {
            channel,
            receiver: Some(events),
            registry: Some(Registry::new(hosts)),
            keys: Some(keys.clone()),
        }
    }

    pub fn create_channel(&self) -> Channel {
        self.channel.clone()
    }

    pub fn start(&mut self) {
        let receiver = self.receiver.take().expect("Start called twice");
        let registry = self.registry.take().expect("Start called twice");
        let encrypt = self.keys.take().expect("Start called twice");
        let channel = self.create_channel();

        tokio::spawn(async move {
            if let Err(e) = Self::handle(receiver, registry, channel, encrypt).await {
                println!("An error occured while handling commands: {e:?}");

                // non-recoverable, so we terminate all threads
                std::process::exit(1);
            }
        });
    }

    /// handles commands from users/connections
    async fn handle(
        mut receiver: Receiver,
        mut registry: Registry,
        channel: Channel,
        keys: CipherKeys,
    ) -> Result<(), RouterError> {
        loop {
            let Some(command) = receiver.recv().await else {
                return Err(RouterError::ClosedChannelError);
            };

            match command {
                Command::TryOpenConnection { target } => {
                    // the host can be offline, so we simply log a warning
                    if let Err(err) = util::open_connection(&target, &keys, channel.clone()).await {
                        println!("failed connection to {target}: {err:?}")
                    }
                }

                Command::AddConnection { origin, channel } => {
                    let Some(c) = registry.connections.insert(origin, channel) else {
                        continue;
                    };

                    // close any old channels
                    let _ = c.send(Message::End);
                }
            };
        }
    }
}
