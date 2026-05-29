use crate::encrypt::{EncryptionHandler, GlobalKeys};
use crate::net::connection::Connection;
use crate::net::state::{Channel, ControllerChannel, Message};

use tokio::net::TcpStream;

pub async fn message_addr_single(
    address: &str,
    keys: &GlobalKeys,
    controller: ControllerChannel,
    msg: Message,
) -> anyhow::Result<Connection> {
    let address = address.to_string();
    let stream = TcpStream::connect(&address).await?;

    let mut con = Connection::new(
        stream,
        address,
        EncryptionHandler::from(keys),
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
