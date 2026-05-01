pub mod config;
pub mod connection;
pub mod encrypt;

use std::io::{self, Write};

use config::Config;
use connection::ConnectionInfo;
use connection::packet::{Intent, Packet};
use connection::state::{Callback, Channel, Message};
use encrypt::GlobalKeys;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    println!("eep2p 0.0.1");

    let config = config::Config::setup("./eep2p.json");
    let hosts = config.hosts.clone().0;
    let port = config.port;

    print!("PGP passphrase: ");
    io::stdout().flush().unwrap();

    // TODO secure input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let keys = GlobalKeys::from(input.trim(), &config);
    let server = listen_on(port, keys.clone());

    println!("Your identifier: {}", config.compute_identifier());

    tokio::spawn(async move {
        print!("Identifier: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        println!("locating {input}");

        let fwd_data = vec![0_u8, 1_u8, 67_u8];
        for host in hosts {
            let data = fwd_data.clone();

            message_addr(
                host,
                &keys,
                Some(Box::new(|channel: &Channel| {
                    println!("callback!");

                    channel
                        .send(Message::SendPacket(Packet::ServerboundIntentPacket(
                            Intent::Fwd,
                        )))
                        .unwrap();

                    channel
                        .send(Message::SendPacket(Packet::ServerboundFwdDataPacket(
                            uuid::uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
                            data,
                        )))
                        .unwrap();
                })),
            )
            .await;
        }
    });

    // handles incoming messages
    server.await;
    unreachable!();
}

pub async fn message_addr(
    address: String,
    keys: &GlobalKeys,
    callback: Option<Callback>,
) -> ConnectionInfo {
    let stream = TcpStream::connect(address).await.unwrap();

    let mut con = connection::handle(stream, keys, callback);

    con.start();
    con.create_channel().send(Message::StartExchange).unwrap();
    con
}

async fn listen_on(port: u16, keys: GlobalKeys) {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let mut con = connection::handle(stream, &keys, None);

        println!("Accepted connection");

        con.start();
        con.create_channel().send(Message::StartExchange).unwrap();
    }
}
