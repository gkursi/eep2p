use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};
use tokio_serde::formats::SymmetricalMessagePack;
use crate::packet::{Packet, Intent};
use crate::handler::{PacketHandler, setup::SetupPacketHandler};
use std::sync::Arc;
use futures_util::{SinkExt, TryStreamExt};

pub enum Message {
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

struct ConnectionState {
    pub intent: Option<Intent>,
    handler: Box<dyn PacketHandler>
}

impl ConnectionInfo {
    pub fn create_channel(&self) -> UnboundedSender<Message> {
        self.channel.clone()
    }

    pub fn start(&mut self) {
        let channel_a = self.create_channel();
        let channel_b = self.create_channel();
        let mut events = self.events.take()
            .expect("start called twice");
        let (read, write) = self.stream.take()
            .expect("start called twice")
            .into_split();
        let mut state = self.state.take()
            .expect("start called twice");
        

        // handles internal events
        tokio::spawn(async move {
            Self::handle(events, channel_a, write, &state).await;
            println!("Handler exit");
        });
        
        // handles incoming packets
        tokio::spawn(async move {
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
            dbg!(&packet);
            
            let mut packet = packet;
            if let Packet::EncryptedPacket(_) = packet {
                packet = packet.decrypt();
            }

            channel.send(Message::Packet(packet));
        }
    }

    async fn handle(
        mut channel: UnboundedReceiver<Message>,
        input: UnboundedSender<Message>,
        output: OwnedWriteHalf, 
        mut state: &ConnectionState
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
                        dbg!(&packet);
                        state.handler = state.handler.handle(packet, &input);
                    },

                    Message::SendPacket(p) => serialize.send(p).await.unwrap(),

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

pub fn handle(stream: TcpStream) -> ConnectionInfo {
    let (channel, events) = mpsc::unbounded_channel();

    ConnectionInfo {
        events: Some(events), 
        channel,
        stream: Some(stream),
        state: Some(ConnectionState { 
            intent: None,
            handler: Box::new(SetupPacketHandler {}) 
        }),
    }
}
