use std::io;
use tokio::{net, io::split};
use std::net::SocketAddr;
use crate::packet::*;

pub type TcpPacketStream = PacketStream<net::TcpStream, net::TcpStream>;

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
    pub async fn accept(&self) -> Result<(TcpPacketStream, SocketAddr), io::Error> {
        let (stream, addr) = self.listener.accept().await?;
        let (read, write) = split(stream);
        Ok((
            PacketStream {
                read: PacketRead { stream: read },
                write: PacketWrite { stream: write }
            },
            addr
        ))
    }
}

#[cfg(feature = "tls")]
pub mod tls {
    use std::io;
    use std::sync::Arc;
    use tokio::io::split;
    use tokio::net;
    use std::net::SocketAddr;
    use tokio_rustls::TlsAcceptor;
    use crate::packet::*;
    use tokio_rustls::rustls::ServerConfig;

    pub type TlsServerTcpPacketStream = PacketStream<tokio_rustls::server::TlsStream<tokio::net::TcpStream>, tokio_rustls::server::TlsStream<tokio::net::TcpStream>>;

    pub struct Server<Url: net::ToSocketAddrs> {
        bind: Url
    }

    impl<Url: net::ToSocketAddrs> Server<Url> {
        pub const fn bind(bind: Url) -> Self {
            return Self {
                bind
            }
        }
        pub async fn listen(&self, config: Arc<ServerConfig>) -> Result<ServerListener, io::Error> {
            Ok(ServerListener {
                listener: net::TcpListener::bind(&self.bind).await?,
                acceptor: TlsAcceptor::from(config)
            })
        }
    }

    pub struct ServerListener {
        listener: net::TcpListener,
        acceptor: TlsAcceptor
    }

    impl ServerListener {
        pub async fn accept(&self) -> Result<(TlsServerTcpPacketStream, SocketAddr), io::Error> {
            let (stream, addr) = self.listener.accept().await?;
            let stream = self.acceptor.accept(stream).await?;
            let (read, write) = split(stream);
            Ok((
                PacketStream {
                    read: PacketRead { stream: read },
                    write: PacketWrite { stream: write }
                },
                addr
            ))
        }
    }
}