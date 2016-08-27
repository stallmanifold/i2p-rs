use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_SESSION_KEY_LENGTH: usize = 32;

#[derive(Eq, Copy)]
pub struct SessionKey {
    data: [u8; I2P_SESSION_KEY_LENGTH]
}

impl SessionKey {
    fn new(data: [u8; I2P_SESSION_KEY_LENGTH]) -> SessionKey {
        SessionKey {
            data: data
        }
    }

    /// Returns the length of a `SessionKey` in bytes.
    pub fn len(&self) -> usize {
        I2P_SESSION_KEY_LENGTH
    }

    fn from_slice(slice: &[u8]) -> Option<SessionKey> {
        if slice.len() <= I2P_SESSION_KEY_LENGTH {
            let mut key_bytes = [0x00; I2P_SESSION_KEY_LENGTH];
            let offset = I2P_SESSION_KEY_LENGTH - slice.len();
            for i in 0..slice.len() {
                key_bytes[i + offset] = slice[i];
            }
            Some(SessionKey::new(key_bytes))
        } else {
            None
        }
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Default for SessionKey {
    fn default() -> SessionKey {
        SessionKey::new([0x00; I2P_SESSION_KEY_LENGTH])
    }
}

impl Clone for SessionKey {
    fn clone(&self) -> SessionKey {
        let mut cloned_hash = [0x00; I2P_SESSION_KEY_LENGTH];
        for i in 0..self.len() {
            cloned_hash[i] = self.data[i];
        }

        SessionKey::new(cloned_hash)
    }
}

impl PartialEq for SessionKey {
    fn eq(&self, other: &SessionKey) -> bool {
        for i in 0..self.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

impl From<[u8; I2P_SESSION_KEY_LENGTH]> for SessionKey {
    fn from(data: [u8; I2P_SESSION_KEY_LENGTH]) -> SessionKey {
        SessionKey::new(data)
    }
}

impl<'a> From<&'a [u8; I2P_SESSION_KEY_LENGTH]> for SessionKey {
    fn from(data: &'a [u8; I2P_SESSION_KEY_LENGTH]) -> SessionKey {
        let mut cloned_data = [0x00; I2P_SESSION_KEY_LENGTH];
        for i in 0..data.len() {
            cloned_data[i] = data[i];
        }

        SessionKey::new(cloned_data)
    }
}

impl fmt::Display for SessionKey {
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

impl fmt::LowerHex for SessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:02x}", byte);
        }

        write!(f, "{}", output)
    }
}

impl fmt::UpperHex for SessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:02X}", byte);
        }

        write!(f, "{}", output)
    }
}

impl fmt::Binary for SessionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:08b}", byte);
        }

        write!(f, "{}", output)
    }
}

impl AsRef<[u8]> for SessionKey {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}


#[cfg(test)]
mod tests {

}
