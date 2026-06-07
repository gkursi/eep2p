use futures_util::{SinkExt, TryStreamExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use tokio_serde::formats::SymmetricalMessagePack;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::crypto::Cipher;
use crate::crypto::aes::Aes;
use crate::net::error::ConnectionHandleError;
use crate::net::message::Message;
use crate::net::state::{Channel, ConnectionState, Receiver, RouterChannel};
use crate::protocol::packet::{OuterPacket, Packet};
use crate::protocol::state::PacketState;
use crate::sequence::splitter::SequenceSplitter;

pub struct Connection {
    pub events: Option<Receiver>,
    pub channel: Channel,
    pub stream: Option<TcpStream>,
    pub state: Option<ConnectionState>,
    pub controller: Option<RouterChannel>,
    pub origin: Option<String>,
}

impl Connection {
    pub fn new(
        stream: TcpStream,
        address: String,
        encryption: Cipher,
        controller: RouterChannel,
    ) -> Self {
        let (channel, events) = mpsc::unbounded_channel();

        Self {
            events: Some(events),
            channel,
            stream: Some(stream),
            controller: Some(controller),
            origin: Some(address),
            state: Some(ConnectionState {
                encryption,
                handler: SequenceSplitter::new(),
            }),
        }
    }

    pub fn create_channel(&self) -> Channel {
        self.channel.clone()
    }

    pub fn start(&mut self) {
        let (read, write) = self.stream.take().expect("start called twice").into_split();

        self.spawn_event_handler(write);
        self.spawn_reader(read);
    }

    fn spawn_event_handler(&mut self, write: OwnedWriteHalf) {
        let mut events = self.events.take().expect("start called twice");
        let state = self.state.take().expect("start called twice");
        let controller = self.controller.take().expect("start called twice");
        let channel = self.create_channel();

        tokio::spawn(async move {
            if let Err(e) = Self::handle(&mut events, channel, write, controller, state).await {
                println!("Error in connection: {e:?}");
            }

            events.close();
        });
    }

    fn spawn_reader(&mut self, read: OwnedReadHalf) {
        let channel = self.create_channel();

        tokio::spawn(async move {
            if let Err(err) = Self::read(&channel, read).await {
                // we ignore the result, since an error should already be logged
                // if the channel is closed
                let _ = channel.send(Message::EndError(err));
            }
        });
    }

    /// Receives packets
    async fn read(channel: &Channel, input: OwnedReadHalf) -> Result<(), ConnectionHandleError> {
        let len_delim = FramedRead::new(input, LengthDelimitedCodec::new());

        let mut deserialize = tokio_serde::SymmetricallyFramed::new(
            len_delim,
            SymmetricalMessagePack::<OuterPacket>::default(),
        );

        while let Some(packet) = deserialize
            .try_next()
            .await
            .map_err(|_| ConnectionHandleError::SerializationError)?
        {
            channel
                .send(Message::HandlePacket(packet))
                .map_err(|_| ConnectionHandleError::IOError)?;
        }

        Ok(())
    }

    /// Handles messages and sends packets
    async fn handle(
        events: &mut Receiver,
        input: Channel,
        output: OwnedWriteHalf,
        controller: RouterChannel,
        mut state: ConnectionState,
    ) -> Result<(), ConnectionHandleError> {
        let len_delim = FramedWrite::new(output, LengthDelimitedCodec::new());

        let mut serialize = tokio_serde::SymmetricallyFramed::new(
            len_delim,
            SymmetricalMessagePack::<OuterPacket>::default(),
        );

        loop {
            let Some(msg) = events.recv().await else {
                return Ok(());
            };

            match msg {
                Message::HandlePacket(packet) => {
                    let mut packet = packet;

                    if let Packet::Encrypted(bytes, nonce) = packet.peek() {
                        let bytes = state
                            .encryption
                            .decrypt(&bytes, *nonce)
                            .map_err(ConnectionHandleError::EncryptError)?;

                        packet = rmp_serde::from_slice::<OuterPacket>(&bytes)
                            .map_err(|_| ConnectionHandleError::SerializationError)?;
                    }

                    state
                        .handler
                        .handle_packet(
                            packet,
                            PacketState {
                                origin: "", // :p
                                channel: &input,
                                controller: &controller,
                                encryption: &mut state.encryption,
                                handler: None,
                            },
                        )
                        .map_err(ConnectionHandleError::HandlerError)?;
                }

                Message::SendPacket(p) => {
                    let (n, d) = state
                        .encryption
                        .encrypt(
                            &rmp_serde::to_vec(&p)
                                .map_err(|_| ConnectionHandleError::SerializationError)?,
                        )
                        .map_err(ConnectionHandleError::EncryptError)?;

                    serialize
                        .send(Packet::new_encrypted(d, n))
                        .await
                        .map_err(|_| ConnectionHandleError::IOError)?
                }

                Message::SendPacketDirect(p) => serialize
                    .send(p)
                    .await
                    .map_err(|_| ConnectionHandleError::IOError)?,

                Message::ExecuteTask(task) => {
                    task.call(PacketState {
                        origin: "", // bleh
                        channel: &input,
                        controller: &controller,
                        encryption: &mut state.encryption,
                        handler: Some(&mut state.handler),
                    })?;
                }

                Message::End => {
                    println!("Closing connection");
                    return Ok(());
                }

                Message::EndError(err) => {
                    return Err(err);
                }
            };
        }
    }
}
