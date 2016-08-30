pub enum Error {
    BufferTooSmall(usize, usize),
    OtherError(String)
}

pub trait Serialize {
    fn serialize(buf: &[u8]) -> Result<usize, Error>;
}

pub trait Deserialize {
    type Output;

    fn deserialize(&self, &mut [u8]) -> Result<Self::Output, Error>;
}
