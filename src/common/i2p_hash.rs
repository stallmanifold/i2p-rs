use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_SHA256_HASH_LENGTH: usize = 32;

#[derive(Eq, Copy)]
pub struct Hash256 {
    data: [u8; I2P_SHA256_HASH_LENGTH]
}

impl Hash256 {
    fn new(data: [u8; I2P_SHA256_HASH_LENGTH]) -> Hash256 {
        Hash256 {
            data: data
        }
    }

    /// Returns the length of an `Hash256` in bytes.
    pub fn len(&self) -> usize {
        I2P_SHA256_HASH_LENGTH
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Default for Hash256 {
    fn default() -> Hash256 {
        Hash256::new([0x00; I2P_SHA256_HASH_LENGTH])
    }
}

impl Clone for Hash256 {
    fn clone(&self) -> Hash256 {
        let mut cloned_hash = [0x00; I2P_SHA256_HASH_LENGTH];
        for i in 0..self.len() {
            cloned_hash[i] = self.data[i];
        }

        Hash256::new(cloned_hash)
    }
}

impl PartialEq for Hash256 {
    fn eq(&self, other: &Hash256) -> bool {
        for i in 0..self.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

impl From<[u8; I2P_SHA256_HASH_LENGTH]> for Hash256 {
    fn from(data: [u8; I2P_SHA256_HASH_LENGTH]) -> Hash256 {
        Hash256::new(data)
    }
}

impl<'a> From<&'a [u8; I2P_SHA256_HASH_LENGTH]> for Hash256 {
    fn from(data: &'a [u8; I2P_SHA256_HASH_LENGTH]) -> Hash256 {
        let mut cloned_data = [0x00; I2P_SHA256_HASH_LENGTH];
        for i in 0..data.len() {
            cloned_data[i] = data[i];
        }

        Hash256::new(cloned_data)
    }
}

impl fmt::Display for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn config() -> base64::Config {
            base64::Config {
                char_set: base64::CharacterSet::Standard,
                newline: base64::Newline::LF,
                pad: false,
                line_length: None
            }
        }

        write!(f, "{}", self.as_slice().to_base64(config()))
    }
}

impl fmt::LowerHex for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:x}", byte);
        }

        write!(f, "{}", output)
    }
}

impl fmt::UpperHex for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:X}", byte);
        }

        write!(f, "{}", output)
    }
}

impl fmt::Binary for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:b}", byte);
        }

        write!(f, "{}", output)
    }
}

impl AsRef<[u8]> for Hash256 {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

pub trait Hashable {
    fn hash(&self) -> Hash256;
}


#[cfg(test)]
mod tests {

}
