use std::io;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, time::Instant};

pub struct PacketStream<S: AsyncWriteExt + AsyncReadExt> {
    pub(crate) stream: S
}

impl<S: AsyncReadExt + AsyncWriteExt + core::marker::Unpin> PacketStream<S> {
    pub async fn recv(&mut self) -> Result<Packet, io::Error> {
        let arrived_at = Instant::now();
        let v = &[0u8; 4];
        let len = u32::from_le_bytes(*v);
        let mut v = Vec::new();
        self.read_to_vec(&mut v, len).await?;
        Ok(Packet {
            body: v,
            arrived_at
        })
    }
    async fn read_to_vec(&mut self, v: &mut Vec<u8>, len: u32) -> Result<(), io::Error> {
        for _ in 0..len {
            let m = &mut [0u8; 1];
            self.stream.read_exact(m).await?;
            v.push(m[0]);
        }
        Ok(())
    }
    pub async fn send(&mut self, b: impl AsRef<[u8]>) -> Result<usize, io::Error> {
        let b = b.as_ref();
        let mut written = 4;
        self.stream.write_all(&<usize as TryInto<u32>>::try_into(b.len()).map_err(|_| {
            io::Error::from(io::ErrorKind::InvalidData);
        })?.to_le_bytes()).await?;
        Ok(written)
    }
}

pub struct Packet {
    pub body: Vec<u8>,
    pub arrived_at: Instant
}

impl Packet {
    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
    /// Retrieve the body.  
    /// If you need it owned, you're better off just taking packet.body directly.
    pub fn body_owned(self) -> Vec<u8> {
        self.body
    }
    pub fn arrival_time(&self) -> Instant {
        self.arrived_at
    }
}