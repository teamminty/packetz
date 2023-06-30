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

#[cfg(feature = "tls")]
pub mod tls {
    use std::io;
    use std::sync::Arc;
    use tokio::net;
    use std::net::SocketAddr;
    use tokio_rustls::TlsAcceptor;
    use crate::packet::*;
    use tokio_rustls::rustls::ServerConfig;

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
        pub async fn accept(&self) -> Result<(PacketStream<tokio_rustls::server::TlsStream<tokio::net::TcpStream>>, SocketAddr), io::Error> {
            let (stream, addr) = self.listener.accept().await?;
            let stream = self.acceptor.accept(stream).await?;
            Ok((
                PacketStream {
                    stream
                },
                addr
            ))
        }
    }
}