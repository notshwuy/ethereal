use std::io::ErrorKind;

use crate::codec::Codec;
use serde::de::DeserializeOwned;
use serde::Serialize;

use tokio::io::{AsyncReadExt, AsyncWriteExt, Error, Result};
use tokio::net::TcpStream;

pub struct Socket {
    pub stream: TcpStream,
}

impl Socket {
    pub fn new(stream: TcpStream) -> Socket {
        Socket { stream }
    }
}

impl Socket {
    pub async fn send<T: Serialize>(&mut self, message: T, codec: &impl Codec<T>) -> Result<usize> {
        let message = codec.serialize(message).unwrap();

        self.stream.write(&message).await
    }

    pub async fn recv<T: DeserializeOwned>(
        &mut self,
        read_buffer: &mut [u8],
        codec: &impl Codec<T>,
    ) -> Result<Option<T>> {
        match self.stream.read(read_buffer).await {
            Ok(0) => Ok(None),
            Err(error) => Err(error),
            Ok(bytes_read) => match codec.deserialize(&read_buffer[..bytes_read]) {
                Ok(message) => Ok(Some(message)),
                Err(_) => Err(Error::new(
                    ErrorKind::InvalidData,
                    "failed to decode message",
                )),
            },
        }
    }
}
