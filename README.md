# Packetz
Create async packet-based servers with ease, Built with gamedev in mind. Stay tuned for UDP support, TLS support and more!

## Basic usage

### Server
```rust
use packetz::{server::*, packet::*};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server: Server<&str> = Server::bind("0.0.0.0:5515"); // The extra &str is the type of the argumet in `server::bind()`, so that server::bind is able to be a const fn.
    let listener: ServerListener = server.listen().await?;

    loop {
        let mut (
            connection: PacketStream<tokio::net::TcpStream>,
            addr: std::net::SocketAddr
        ) = listener.accept().await?;
        tokio::spawn(async move {
            'l: loop { // In a real world scenario we would check for errors on the `send` and `recv` methods, and break the loop if one is found, and disconnect the client without disconnecting all other clients.
                let msg = connection.recv().await?;
                connection.send(msg).await?;
                connection.disconnect(); // This is optional, as all it does is drop the PacketStream, and breaking the loop should automaticall drop it.
                break 'l;
            }
        })
    }
}
```

### Client
```rust
use packetz::{client::*, packet::*};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut client = packetz::client::connect("0.0.0.0:5515").await?;
    client.send(b"Hello, Packetz!").await?;
    println!("{}", String::from_utf8(
        client.recv().await?
    )?);
    Ok(())
}
```

### Dependencies
Dependencies for these examples:

```toml
[dependencies]
packetz = "0.1.0" # Replace this with the latest version, if it's not already the latest version.
tokio = { version = "1.29.0", features = ["net", "io-util", "time", "rt", "rt-multi-thread"] }
```