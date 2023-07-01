use tokio::{net, io::split};
use crate::packet::*;
use std::io;

pub type TcpPacketStream = PacketStream<net::TcpStream, net::TcpStream>;

pub async fn connect<Url: net::ToSocketAddrs>(url: Url) -> Result<TcpPacketStream, io::Error> {
    let stream = net::TcpStream::connect(url).await?;
    let (read, write) = split(stream);
    Ok(PacketStream {
        read: PacketRead { stream: read },
        write: PacketWrite { stream: write }
    })
}

#[cfg(feature = "tls")]
pub mod tls {
    use tokio::io::split;
    use tokio::net;
    use tokio_rustls::TlsConnector;
    use tokio_rustls::rustls::ClientConfig;
    use crate::packet::*;
    use tokio_rustls::rustls;
    use std::io;
    use std::sync::Arc;

    pub type TlsClientTcpPacketStream = PacketStream<
        tokio_rustls::client::TlsStream<
            tokio::net::TcpStream
        >,
        tokio_rustls::client::TlsStream<
            tokio::net::TcpStream
        >
    >;

    pub async fn connect<Url: AsRef<str> + net::ToSocketAddrs>(url: Url, config: Arc<ClientConfig>) -> Result<
        TlsClientTcpPacketStream,
        io::Error
    > {
        let connector = TlsConnector::from(config);
        let stream = net::TcpStream::connect(&url).await?;
        let domain = rustls::ServerName::try_from(url.as_ref().split(":").filter(|i| {
            i != &""
        }).collect::<Vec<&str>>()[0])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid dnsname"))?;
        let stream = connector.connect(domain, stream).await?;
        let (read, write) = split(stream);
        Ok(PacketStream {
            read: PacketRead { stream: read },
            write: PacketWrite { stream: write }
        })
    }
}