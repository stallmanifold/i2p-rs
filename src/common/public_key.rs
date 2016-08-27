use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_PUBLIC_KEY_LENGTH: usize = 256;

#[derive(Copy, Eq)]
pub struct PublicKey {
    data: [u8; I2P_PUBLIC_KEY_LENGTH]
}

impl PublicKey {
    fn new(data: [u8; I2P_PUBLIC_KEY_LENGTH]) -> PublicKey {
        PublicKey {
            data: data
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn from_slice(slice: &[u8]) -> Option<PublicKey> {
        if slice.len() <= I2P_PUBLIC_KEY_LENGTH {
            let mut key_bytes = [0x00; I2P_PUBLIC_KEY_LENGTH];
            let offset = I2P_PUBLIC_KEY_LENGTH - slice.len();
            for i in 0..slice.len() {
                key_bytes[i + offset] = slice[i];
            }
            Some(PublicKey::new(key_bytes))
        } else {
            None
        }
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Clone for PublicKey {
    fn clone(&self) -> PublicKey {
        let mut cloned_array = [0x00; I2P_PUBLIC_KEY_LENGTH];
        for i in 0..cloned_array.len() {
            cloned_array[i] = self.data[i];
        }

        PublicKey::new(cloned_array)
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &PublicKey) -> bool {
        for i in 0..self.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

impl Default for PublicKey {
    fn default() -> PublicKey {
        PublicKey {
            data: [0x00; I2P_PUBLIC_KEY_LENGTH]
        }
    }
}

impl From<[u8; I2P_PUBLIC_KEY_LENGTH]> for PublicKey {
    fn from(data: [u8; I2P_PUBLIC_KEY_LENGTH]) -> PublicKey {
        PublicKey::new(data)
    }
}

impl<'a> From<&'a [u8; I2P_PUBLIC_KEY_LENGTH]> for PublicKey {
    fn from(data: &'a [u8; I2P_PUBLIC_KEY_LENGTH]) -> PublicKey {
        let mut cloned_data = [0x00; I2P_PUBLIC_KEY_LENGTH];
        for i in 0..data.len() {
            cloned_data[i] = data[i];
        }

        PublicKey::new(cloned_data)
    }
}

impl fmt::Display for PublicKey {
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

impl fmt::LowerHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:02x}", byte);
        }

        write!(f, "{}", output)
    }
}

impl fmt::UpperHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:02X}", byte);
        }

        write!(f, "{}", output)
    }
}

impl fmt::Binary for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:08b}", byte);
        }

        write!(f, "{}", output)
    }
}

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}


#[cfg(test)]
mod tests {

}
