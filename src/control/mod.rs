pub mod registry;
pub mod state;

use crate::config::data::Hosts;
use crate::encrypt::GlobalKeys;
use crate::net;
use crate::net::packet::Packet;
use crate::net::state::Message;

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
    keys: Option<GlobalKeys>,
}

impl CommandHandler {
    pub fn new(keys: &GlobalKeys, hosts: Hosts) -> Self {
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

                // non-recoverable, terminate all threads
                std::process::exit(1);
            }
        });
    }

    // todo reduce cloning
    /// handles commands from users/connections
    async fn handle(
        mut receiver: Receiver,
        registry: Registry,
        channel: Channel,
        keys: GlobalKeys,
    ) -> Result<(), ControllerError> {
        loop {
            let Some(command) = receiver.recv().await else {
                return Err(ControllerError::ClosedChannelError);
            };

            match command {
                Command::ForwardData { origin, id, data } => {
                    let new_id = Uuid::new_v4();
                    let fwd = FwdConnection {
                        origin,
                        origin_id: id,
                    };

                    registry
                        .fwd_connections
                        .insert(new_id, fwd, Duration::from_secs(LIFETIME));

                    let msg = Message::SendPacket(Packet::ServerboundFwdDataPacket(id, data));

                    for host in &registry.hosts.0 {
                        channel
                            .send(Command::ForwardDataTo {
                                target: host.clone(),
                                msg: msg.clone(),
                            })
                            .map_err(|_| ControllerError::ClosedChannelError)?;
                    }
                }

                Command::ForwardDataTo { target, msg } => {
                    let keys = keys.clone();
                    let channel = channel.clone();

                    tokio::spawn(async move {
                        // hosts can be offline, fails are ignored
                        let _ = net::util::message_addr_single(&target, &keys, channel, msg).await;
                    });
                }
            };
        }
    }
}
