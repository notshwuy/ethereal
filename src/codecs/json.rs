use crate::codec::{Codec, CodecError};
use serde::{de::DeserializeOwned, Serialize};

pub struct JsonCodec;

impl JsonCodec {
    pub fn new() -> JsonCodec {
        JsonCodec
    }
}

impl<T: Serialize + DeserializeOwned> Codec<T> for JsonCodec {
    fn serialize(&self, message: T) -> Result<Vec<u8>, CodecError> {
        match serde_json::to_vec::<T>(&message) {
            Ok(message) => Ok(message),
            Err(_) => Err(CodecError::CantSerialize),
        }
    }

    fn deserialize(&self, message: &[u8]) -> Result<T, CodecError> {
        match serde_json::from_slice::<T>(message) {
            Ok(message) => Ok(message),
            Err(_) => Err(CodecError::CantSerialize),
        }
    }
}
