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

const PORT: usize = 8188;

#[tokio::main]
async fn main() {
    println!("eep2p 0.0.1");
  
    let config = config::Config::setup("./eep2p.json");

    // handles incoming messages
    tokio::spawn(async move {
        server_main().await;
    });

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

    println!("sending intent");
    ch.send(Message::SendPacket(
        Packet::ServerboundIntentPacket(Intent::Fwd)
    )).unwrap();
    
    loop {}
}

pub async fn message_addr(address: String) -> ConnectionInfo {
    let stream = TcpStream::connect(format!("{address}"))
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
