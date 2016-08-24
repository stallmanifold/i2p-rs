use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_PUBLIC_KEY_LENGTH: usize = 256;

#[derive(Copy, Eq)]
pub struct PrivateKey {
    data: [u8; I2P_PUBLIC_KEY_LENGTH]
}

impl PrivateKey {
    fn new(slice: [u8; I2P_PUBLIC_KEY_LENGTH]) -> PrivateKey {
        let mut data: [u8; I2P_PUBLIC_KEY_LENGTH] = [0x00; I2P_PUBLIC_KEY_LENGTH];
        for i in 0..slice.len() {
            data[i] = slice[i];
        }

        PrivateKey {
            data: data
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn from_slice(slice: &[u8]) -> Option<PrivateKey> {
        if slice.len() <= I2P_PUBLIC_KEY_LENGTH {
            let mut key_bytes = [0x00; I2P_PUBLIC_KEY_LENGTH];
            let offset = I2P_PUBLIC_KEY_LENGTH - slice.len();
            for i in 0..slice.len() {
                key_bytes[i + offset] = slice[i];
            }
            Some(PrivateKey::new(key_bytes))
        } else {
            None
        }
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Clone for PrivateKey {
    fn clone(&self) -> PrivateKey {
        let mut cloned_array = [0x00; I2P_PUBLIC_KEY_LENGTH];
        for i in 0..cloned_array.len() {
            cloned_array[i] = self.data[i];
        }

        PrivateKey::new(cloned_array)
    }
}

impl PartialEq for PrivateKey {
    fn eq(&self, other: &PrivateKey) -> bool {
        for i in 0..self.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

impl Default for PrivateKey {
    fn default() -> PrivateKey {
        PrivateKey {
            data: [0x00; I2P_PUBLIC_KEY_LENGTH]
        }
    }
}

impl fmt::Display for PrivateKey {
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
