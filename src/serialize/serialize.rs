use std::result;


pub enum BufferSize {
    Need(usize),
    Available(usize)
}

pub enum Error {
    // The buffer is too small. The first field is the needed number of bytes in the buffer,
    // the second field is the available amount of bytes in the buffer.
    BufferTooSmall(BufferSize, BufferSize),
    EncodingError(String),
    DecodingError(String),
}

impl Error {
    pub fn buffer_too_small(need: usize, available: usize) -> Error {
        Error::BufferTooSmall(BufferSize::Need(need), BufferSize::Available(available))
    }
}

pub type Result<T> = result::Result<T, Error>;

pub trait Serialize {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Deserialize {
    type Output;

    fn deserialize(buf: &[u8]) -> Result<Self::Output>;
}
