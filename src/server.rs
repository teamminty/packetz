use std::io;
use tokio::net;
use std::net::SocketAddr;
use crate::packet::*;

pub struct Server<Url: net::ToSocketAddrs> {
    bind: Url
}

impl<Url: net::ToSocketAddrs> Server<Url> {
    pub const fn bind(bind: Url) -> Self {
        return Self {
            bind
        }
    }
    pub async fn listen(&self) -> Result<ServerListener, io::Error> {
        Ok(ServerListener {
            listener: net::TcpListener::bind(&self.bind).await?
        })
    }
}

pub struct ServerListener {
    listener: net::TcpListener
}

impl ServerListener {
    pub async fn accept(&self) -> Result<(PacketStream<net::TcpStream>, SocketAddr), io::Error> {
        let (stream, addr) = self.listener.accept().await?;
        Ok((
            PacketStream {
                stream
            },
            addr
        ))
    }
}