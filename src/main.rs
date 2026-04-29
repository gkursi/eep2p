pub mod con;
pub mod packet;
pub mod handler;
pub mod encrypt;
pub mod config;

use con::{handle, Message, ConnectionInfo};
use tokio::net::{TcpListener, TcpStream};
use std::io::{self, Write};
use handler::setup::Side;
use packet::{Packet, Intent};
use tokio::time::{sleep, Duration};

const PORT: usize = 8189;

#[tokio::main]
async fn main() {
    println!("eep2p 0.0.1");
  
    let _config = config::Config::setup("./eep2p.json");

    tokio::spawn(async move {
        print!("Identifier: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        println!("connecting to {input}!");
        let ch = message_addr(input).await
            .create_channel();
        
        ch.send(Message::StartExchange).unwrap();
        sleep(Duration::from_millis(2000)).await;
        ch.send(Message::SendPacket(Packet::ServerboundIntentPacket(Intent::Fwd))).unwrap();
    });

    // handles incoming messages
    server_main().await;
}

pub async fn message_addr(address: String) -> ConnectionInfo {
    let stream = TcpStream::connect(address)
        .await
        .unwrap();
    
    let mut con = handle(stream, Side::Client);
    con.start();

    con
}

async fn server_main() {
    let listener = TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .unwrap();

    loop {
        let (stream, _) = listener.accept()
            .await
            .unwrap();

        println!("Accepted connection");
        let mut con = handle(stream, Side::Server);
        con.start();
        con.create_channel()
            .send(Message::StartExchange)
            .unwrap();
    }
}
