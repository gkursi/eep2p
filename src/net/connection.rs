use futures_util::{SinkExt, TryStreamExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use tokio_serde::formats::SymmetricalMessagePack;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::crypto::Cipher;
use crate::crypto::aes::Aes;
use crate::net::error::ConnectionError;
use crate::net::message::Message;
use crate::net::state::{Callback, Channel, ConnectionState, Receiver, RouterChannel};
use crate::proto::handler::PacketHandler;
use crate::proto::handlers::setup::SetupPacketHandler;
use crate::proto::packet::Packet;
use crate::proto::state::PacketState;

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
        callback: Option<Callback>,
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
                handler: SetupPacketHandler::new_handler(),
                sent_key: false,
                recv_key: false,
                callback,
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
    async fn read(channel: &Channel, input: OwnedReadHalf) -> Result<(), ConnectionError> {
        let len_delim = FramedRead::new(input, LengthDelimitedCodec::new());

        let mut deserialize = tokio_serde::SymmetricallyFramed::new(
            len_delim,
            SymmetricalMessagePack::<Packet>::default(),
        );

        while let Some(packet) = deserialize
            .try_next()
            .await
            .map_err(|_| ConnectionError::SerializationError)?
        {
            channel
                .send(Message::HandlePacket(packet))
                .map_err(|_| ConnectionError::IOError)?;
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
    ) -> Result<(), ConnectionError> {
        let len_delim = FramedWrite::new(output, LengthDelimitedCodec::new());

        let mut serialize = tokio_serde::SymmetricallyFramed::new(
            len_delim,
            SymmetricalMessagePack::<Packet>::default(),
        );

        loop {
            let Some(msg) = events.recv().await else {
                return Ok(());
            };

            match msg {
                Message::HandlePacket(packet) => {
                    let mut packet = packet;

                    if let Packet::Encrypted(bytes, nonce) = packet {
                        let bytes = state
                            .encryption
                            .decrypt(&bytes, nonce)
                            .map_err(ConnectionError::EncryptError)?;

                        packet = rmp_serde::from_slice::<Packet>(&bytes)
                            .map_err(|_| ConnectionError::SerializationError)?;
                    }

                    if let Packet::KeyExchange(_) = packet {
                        state.recv_key = true;
                        Self::invoke_callback(&input, &mut state)?;
                    }

                    state.handler = state
                        .handler
                        .handle(
                            packet,
                            PacketState {
                                origin: "", // sob
                                channel: &input,
                                controller: &controller,
                                encryption: &mut state.encryption,
                            },
                        )
                        .map_err(ConnectionError::HandlerError)?;
                }

                Message::StartExchange => {
                    serialize
                        .send(Packet::KeyExchange(
                            state
                                .encryption
                                .x25_public
                                .take()
                                .expect("StartExchange sent twice"),
                        ))
                        .await
                        .map_err(|_| ConnectionError::SerializationError)?;

                    state.sent_key = true;
                    Self::invoke_callback(&input, &mut state)?;
                }

                Message::SendPacket(p) => {
                    let (d, n) = state
                        .encryption
                        .encrypt(
                            &rmp_serde::to_vec(&p)
                                .map_err(|_| ConnectionError::SerializationError)?,
                        )
                        .map_err(ConnectionError::EncryptError)?;

                    serialize
                        .send(Packet::Encrypted(n, d))
                        .await
                        .map_err(|_| ConnectionError::IOError)?
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

    /// Invoke callback after key exchange
    fn invoke_callback(
        channel: &Channel,
        state: &mut ConnectionState,
    ) -> Result<(), ConnectionError> {
        if !state.sent_key || !state.recv_key {
            return Ok(());
        }

        let Some(callback) = state.callback.take() else {
            return Ok(());
        };

        callback(channel).map_err(|_| ConnectionError::CallbackError)?;

        Ok(())
    }
}
