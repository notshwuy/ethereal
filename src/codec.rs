#[derive(Debug)]
pub enum CodecError {
    CantSerialize,
    CantDeserialize,
}

pub trait Codec<T> {
    fn serialize(&self, message: T) -> Result<Vec<u8>, CodecError>;
    fn deserialize(&self, message: &[u8]) -> Result<T, CodecError>;
}
