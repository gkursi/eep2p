use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_util::codec::{
    FramedRead,
    FramedWrite,
    LengthDelimitedCodec
};
use tokio_serde::formats::SymmetricalMessagePack;
use crate::packet::Packet;
use crate::handler::{
    Handler,
    setup::SetupPacketHandler
};
use crate::handler::setup::Side;
use crate::encrypt::EncryptionHandler;
use futures_util::{SinkExt, TryStreamExt};

// TODO General cleanup
// TODO Fix memory leak
// TODO Fix thread lifetimes

pub enum Message {
    StartExchange,
    Packet(Packet),
    SendPacket(Packet),
    End
}

pub struct ConnectionInfo {
    pub events: Option<UnboundedReceiver<Message>>,
    channel: UnboundedSender<Message>,
    pub stream: Option<TcpStream>,
    pub state: Option<ConnectionState>,
}

pub struct ConnectionState {
    encryption: EncryptionHandler,
    handler: Handler
}

impl ConnectionInfo {
    pub fn create_channel(&self) -> UnboundedSender<Message> {
        self.channel.clone()
    }

    pub fn start(&mut self) {
        let channel_a = self.create_channel();
        let channel_b = self.create_channel();
        let events = self.events.take()
            .expect("start called twice");
        let (read, write) = self.stream.take()
            .expect("start called twice")
            .into_split();
        let state = self.state.take()
            .expect("start called twice");
        

        // handles internal events
        tokio::spawn(async move {
            Self::handle(events, channel_a, write, state).await;
            println!("Handler exit");
        });
        
        // handles incoming packets
        Ftokio::spawn(async move {
            Self::read(channel_b, read).await;
            println!("Reader exit");
        });
    }
    
    async fn read(channel: UnboundedSender<Message>, input: OwnedReadHalf) {
        let len_delim = FramedRead::new(input, LengthDelimitedCodec::new());
        
        let mut deserialize = tokio_serde::SymmetricallyFramed::new(
            len_delim,
            SymmetricalMessagePack::<Packet>::default()
        );

        while let Some(packet) = deserialize.try_next().await.unwrap() {
            channel.send(Message::Packet(packet)).unwrap();
        }
    }

    async fn handle(
        mut channel: UnboundedReceiver<Message>,
        input: UnboundedSender<Message>,
        output: OwnedWriteHalf, 
        mut state: ConnectionState
    ) {
        let len_delim = FramedWrite::new(output, LengthDelimitedCodec::new()); 

        let mut serialize = tokio_serde::SymmetricallyFramed::new(
            len_delim, 
            SymmetricalMessagePack::<Packet>::default()
        );

        loop {
            let msg = channel.recv().await;

            if let Some(msg) = msg {
                match msg {
                    Message::Packet(packet) => {
                        let mut packet = packet;

                        if let Packet::EncryptedPacket(bytes, nonce) = packet {
                            let bytes = state.encryption.decrypt(&bytes, nonce);
                            packet = rmp_serde::from_slice::<Packet>(&bytes)
                                .expect("failed to decode packet");

                            dbg!("Decrypted packet: ");
                            dbg!(&packet);
                        }

                        dbg!(&packet);
                        state.handler = state.handler.handle(packet, &input, &mut state.encryption);
                    },

                    Message::StartExchange => {
                        serialize.send(Packet::KeyPacket(
                            state.encryption.x25_public
                                .take()
                                .expect("StartExchange sent twice")
                        ))
                            .await
                            .unwrap();
                    },

                    Message::SendPacket(p) => {
                        let (d, n) = state.encryption.encrypt(
                            &rmp_serde::to_vec(&p).unwrap()
                        );

                        serialize.send(Packet::EncryptedPacket(n, d)).await.unwrap()
                    },

                    Message::End => {
                        println!("Received end, closing connection");
                        break;
                    },
                };

                continue;
            }

            channel.close();
            return;
        }
    }
}

pub fn handle(stream: TcpStream, _side: Side) -> ConnectionInfo {
    let (channel, events) = mpsc::unbounded_channel();

    ConnectionInfo {
        events: Some(events), 
        channel,
        stream: Some(stream),
        state: Some(ConnectionState { 
            encryption: EncryptionHandler::new(),
            handler: Handler::Setup(
                SetupPacketHandler {}
            )
        }),
    }
}
