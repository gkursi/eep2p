pub mod config;
pub mod crypto;
pub mod net;
pub mod proto;
pub mod router;

use std::io::{self, Write};

use config::Config;
use crypto::{Cipher, CipherKeys};
use net::message::Message;
use router::command::handler::CommandHandler;
use tokio::net::TcpListener;

use crate::net::{connection::Connection, state::RouterChannel};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("eep2p 0.0.1");

    let config = Config::read_or_create("./eep2p.json")?;
    let hosts = config.hosts.clone();
    let port = config.port;

    print!("PGP passphrase: ");
    io::stdout().flush()?;

    // todo secure input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let keys = CipherKeys::from(input.trim(), &config);
    let mut controller = CommandHandler::new(&keys, hosts);
    let server = listen_on(port, keys.clone(), controller.create_channel());

    println!("Your identifier: {}", config.compute_identifier());

    // global events
    controller.start();

    //
    server.await?;
    Ok(())
}

async fn listen_on(port: u16, keys: CipherKeys, controller: RouterChannel) -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        let mut con = Connection::new(
            stream,
            addr.to_string(),
            Cipher::from(&keys),
            controller.clone(),
            None,
        );

        println!("Accepted connection");

        con.start();
        con.create_channel().send(Message::StartExchange)?;
    }
}
