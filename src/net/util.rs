use tokio::net::{TcpListener, TcpStream};

use crate::crypto::{Cipher, CipherKeys};
use crate::net::connection::Connection;
use crate::net::error::ConnectionCreateError;
use crate::net::message::Message;
use crate::net::state::RouterChannel;
use crate::net::task::Task;

pub async fn open_connection(
    address: &str,
    keys: &CipherKeys,
    controller: RouterChannel,
) -> Result<(), ConnectionCreateError> {
    let address = address.to_string();
    let stream = TcpStream::connect(&address)
        .await
        .map_err(|e| ConnectionCreateError::IOError(e))?;
    let mut con = Connection::new(stream, address, Cipher::new(keys), controller);

    con.start();
    con.channel
        .send(Message::ExecuteTask(Task::InitConnection))
        .map_err(|_| ConnectionCreateError::ChannelError)?;

    Ok(())
}

pub async fn listen_on(
    port: u16,
    keys: CipherKeys,
    controller: RouterChannel,
) -> Result<(), ConnectionCreateError> {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .map_err(|_| ConnectionCreateError::BindError)?;

    loop {
        let (stream, addr) = listener
            .accept()
            .await
            .map_err(|_| ConnectionCreateError::AcceptListenerError)?;

        let mut con = Connection::new(
            stream,
            addr.to_string(),
            Cipher::new(&keys),
            controller.clone(),
        );

        println!("Accepted connection");

        con.start();
    }
}
