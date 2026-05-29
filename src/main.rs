pub mod config;
pub mod control;
pub mod encrypt;
pub mod handle;
pub mod net;

use std::io::{self, Write};

use config::Config;
use control::CommandHandler;
use encrypt::{EncryptionHandler, GlobalKeys};
use net::state::Message;
use tokio::net::TcpListener;

use crate::net::{connection::Connection, state::ControllerChannel};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("eep2p 0.0.1");

    let config = Config::setup("./eep2p.json")?;
    let hosts = config.hosts.clone();
    let port = config.port;

    print!("PGP passphrase: ");
    io::stdout().flush()?;

    // todo secure input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let keys = GlobalKeys::from(input.trim(), &config);
    let mut controller = CommandHandler::new(&keys, hosts);
    let server = listen_on(port, keys.clone(), controller.create_channel());

    println!("Your identifier: {}", config.compute_identifier());

    // global events
    controller.start();

    //
    server.await?;
    Ok(())
}

async fn listen_on(
    port: u16,
    keys: GlobalKeys,
    controller: ControllerChannel,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        let mut con = Connection::new(
            stream,
            addr.to_string(),
            EncryptionHandler::from(&keys),
            controller.clone(),
            None,
        );

        println!("Accepted connection");

        con.start();
        con.create_channel().send(Message::StartExchange)?;
    }
}
