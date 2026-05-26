pub mod config;
pub mod connection;
pub mod control;
pub mod encrypt;

use std::io::{self, Write};

use config::Config;
use connection::state::Message;
use control::CommandHandler;
use encrypt::{EncryptionHandler, GlobalKeys};
use tokio::net::TcpListener;

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
    let server = listen_on(port, keys.clone());

    println!("Your identifier: {}", config.compute_identifier());

    // global events
    //
    controller.start();

    //
    server.await?;
    Ok(())
}

async fn listen_on(port: u16, keys: GlobalKeys) -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let mut con = connection::handle(stream, EncryptionHandler::from(&keys), None);

        println!("Accepted connection");

        con.start();
        con.create_channel().send(Message::StartExchange)?;
    }
}
