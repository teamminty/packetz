use tokio::net;
use crate::packet::*;
use std::io;

pub async fn connect<Url: net::ToSocketAddrs>(url: Url) -> Result<PacketStream<net::TcpStream>, io::Error> {
    let stream = net::TcpStream::connect(url).await?;
    Ok(PacketStream {
        stream
    })
}

#[cfg(feature = "tls")]
pub mod tls {
    use tokio::net;
    use tokio_rustls::TlsConnector;
    use tokio_rustls::rustls::ClientConfig;
    use crate::packet::*;
    use tokio_rustls::rustls;
    use std::io;
    use std::sync::Arc;

    pub async fn connect<Url: AsRef<str> + net::ToSocketAddrs>(url: Url, config: Arc<ClientConfig>) -> Result<
        PacketStream<
            tokio_rustls::client::TlsStream<
                tokio::net::TcpStream
            >
        >,
        io::Error
    > {
        let connector = TlsConnector::from(config);
        let stream = net::TcpStream::connect(&url).await?;
        let domain = rustls::ServerName::try_from(url.as_ref().split(":").filter(|i| {
            i != &""
        }).collect::<Vec<&str>>()[0])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid dnsname"))?;
        let stream = connector.connect(domain, stream).await?;
        Ok(PacketStream {
            stream
        })
    }
}