use std::result;
use std::error;
use std::fmt;


#[derive(Clone, Debug)]
pub enum Error {
    // The buffer is too small. The first field is the needed number of bytes in the buffer,
    // the second field is the available amount of bytes in the buffer.
    BufferTooSmall(usize, usize),
    Encoding,
    Decoding,
}

impl Error {
    pub fn buffer_too_small(need: usize, have: usize) -> Error {
        Error::BufferTooSmall(need, have)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BufferTooSmall(need, have) => {
                writeln!(f, "The buffer size is too small. Got: {} bytes; Need: {} bytes.", need, have)
            }
            Error::Encoding => {
                writeln!(f, "An error occurred in serialization.")
            },
            Error::Decoding => {
                writeln!(f, "An error occurred in deserialization.")
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BufferTooSmall(_, _) => "The buffer size is too small to write into.",
            Error::Encoding => "An error occurred in serialization.",
            Error::Decoding => "An error occurred in deserialization."
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
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
