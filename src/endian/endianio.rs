pub enum Error {
    BufferTooSmall(usize, usize),
    OtherError(String)
}

pub trait ReadBytesBigEndian {
    type Output;

    fn read_bytes_be(buf: &[u8]) -> Result<Self::Output, Error>;
}

pub trait WriteBytesBigEndian {
    fn write_bytes_be(&self, buf: &mut [u8]) -> Result<usize, Error>;
}

pub trait ReadBytesLittleEndian {
    type Output;

    fn read_bytes_le(buf: &[u8]) -> Result<Self::Output, Error>;
}

pub trait WriteBytesLittleEndian {
    fn write_bytes_le(&self, buf: &mut [u8]) -> Result<usize, Error>;
}
