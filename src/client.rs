use tokio::net;
use crate::packet::*;
use std::io;

pub async fn connect<Url: net::ToSocketAddrs>(url: Url) -> Result<PacketStream<net::TcpStream>, io::Error> {
    Ok(PacketStream {
        stream: net::TcpStream::connect(url).await?
    })
}