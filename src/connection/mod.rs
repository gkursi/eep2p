pub mod handler;
pub mod packet;
pub mod state;

use futures_util::{SinkExt, TryStreamExt};
use handler::setup::SetupPacketHandler;
use handler::{Handler, HandlerError, PacketHandler};
use packet::Packet;
use state::{Callback, Channel, ConnectionError, ConnectionState, Message, Receiver};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use tokio_serde::formats::SymmetricalMessagePack;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use crate::encrypt::aes::Aes;
use crate::encrypt::{EncryptionHandler, GlobalKeys};

pub struct ConnectionInfo {
    pub events: Option<Receiver>,
    pub channel: Channel,
    pub stream: Option<TcpStream>,
    pub state: Option<ConnectionState>,
}

impl ConnectionInfo {
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
        let channel = self.create_channel();

        tokio::spawn(async move {
            if let Err(e) = Self::handle(&mut events, channel, write, state).await {
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
                .send(Message::Packet(packet))
                .map_err(|_| ConnectionError::IOError)?;
        }

        Ok(())
    }

    /// Handles messages and sends packets
    async fn handle(
        events: &mut Receiver,
        input: Channel,
        output: OwnedWriteHalf,
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
                Message::Packet(packet) => {
                    let mut packet = packet;

                    if let Packet::EncryptedPacket(bytes, nonce) = packet {
                        let bytes = state
                            .encryption
                            .decrypt(&bytes, nonce)
                            .map_err(ConnectionError::EncryptError)?;

                        packet = rmp_serde::from_slice::<Packet>(&bytes)
                            .map_err(|_| ConnectionError::SerializationError)?;
                    }

                    if let Packet::CommonKeyPacket(_) = packet {
                        state.recv_key = true;
                        Self::invoke_callback(&input, &mut state)?;
                    }

                    state.handler = state
                        .handler
                        .handle(packet, &input, &mut state.encryption)
                        .map_err(ConnectionError::HandlerError)?;
                }

                Message::StartExchange => {
                    serialize
                        .send(Packet::CommonKeyPacket(
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
                        .send(Packet::EncryptedPacket(n, d))
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

            continue;
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
        state.callback = None;

        Ok(())
    }
}

pub fn handle(stream: TcpStream, keys: &GlobalKeys, callback: Option<Callback>) -> ConnectionInfo {
    let (channel, events) = mpsc::unbounded_channel();

    ConnectionInfo {
        events: Some(events),
        channel,
        stream: Some(stream),
        state: Some(ConnectionState {
            encryption: EncryptionHandler::from(keys),
            handler: SetupPacketHandler::new_handler(),
            sent_key: false,
            recv_key: false,
            callback,
        }),
    }
}
