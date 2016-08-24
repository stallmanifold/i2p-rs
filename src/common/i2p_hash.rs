use std::fmt;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_SHA256_HASH_LENGTH: usize = 32;

#[derive(Eq, Copy)]
pub struct I2pHash {
    data: [u8; I2P_SHA256_HASH_LENGTH]
}

impl I2pHash {
    fn new(data: [u8; I2P_SHA256_HASH_LENGTH]) -> I2pHash {
        I2pHash {
            data: data
        }
    }

    /// Returns the length of an `I2pHash` in bytes.
    pub fn len() -> usize {
        I2P_SHA256_HASH_LENGTH
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Default for I2pHash {
    fn default() -> I2pHash {
        I2pHash::new([0x00; I2P_SHA256_HASH_LENGTH])
    }
}

impl Clone for I2pHash {
    fn clone(&self) -> I2pHash {
        let mut cloned_hash = [0x00; I2P_SHA256_HASH_LENGTH];
        for i in 0..self.data.len() {
            cloned_hash[i] = self.data[i];
        }

        I2pHash::new(cloned_hash)
    }
}

impl PartialEq for I2pHash {
    fn eq(&self, other: &I2pHash) -> bool {
        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

impl From<[u8; I2P_SHA256_HASH_LENGTH]> for I2pHash {
    fn from(data: [u8; I2P_SHA256_HASH_LENGTH]) -> I2pHash {
        I2pHash::new(data)
    }
}

impl<'a> From<&'a [u8; I2P_SHA256_HASH_LENGTH]> for I2pHash {
    fn from(data: &'a [u8; I2P_SHA256_HASH_LENGTH]) -> I2pHash {
        I2pHash::new(data.clone())
    }
}

impl fmt::Display for I2pHash {
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

pub trait Hashable {
    fn hash(&self) -> I2pHash;
}


#[cfg(test)]
mod tests {

}
