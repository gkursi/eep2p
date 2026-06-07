pub mod config;
pub mod crypto;
pub mod net;
pub mod protocol;
pub mod router;
pub mod sequence;

use std::io::{self, Write};

use config::Config;
use crypto::CipherKeys;
use router::command::handler::CommandHandler;

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

    let keys = CipherKeys::new(input.trim(), &config)?;
    let mut controller = CommandHandler::new(&keys, hosts);
    let server = net::util::listen_on(port, keys.clone(), controller.create_channel());

    println!("Your identifier: {}", config.compute_identifier());

    // global events
    controller.start();

    //
    server.await?;
    Ok(())
}
