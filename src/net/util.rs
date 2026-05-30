use crate::crypto::{Cipher, CipherKeys};
use crate::net::connection::Connection;
use crate::net::message::Message;
use crate::net::state::{Channel, RouterChannel};

use tokio::net::TcpStream;

pub async fn message_addr_single(
    address: &str,
    keys: &CipherKeys,
    controller: RouterChannel,
    msg: Message,
) -> anyhow::Result<Connection> {
    let address = address.to_string();
    let stream = TcpStream::connect(&address).await?;

    let mut con = Connection::new(
        stream,
        address,
        Cipher::new(keys),
        controller,
        // callback
        Some(Box::new(|ch: &Channel| {
            ch.send(msg)?;
            Ok(())
        })),
    );

    con.start();
    con.create_channel().send(Message::StartExchange)?;
    Ok(con)
}
