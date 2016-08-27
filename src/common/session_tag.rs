use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_SESSION_TAG_LENGTH: usize = 32;

#[derive(Eq, Copy)]
pub struct SessionTag {
    data: [u8; I2P_SESSION_TAG_LENGTH]
}

impl SessionTag {
    fn new(data: [u8; I2P_SESSION_TAG_LENGTH]) -> SessionTag {
        SessionTag {
            data: data
        }
    }

    /// Returns the length of an `SessionTag` in bytes.
    pub fn len(&self) -> usize {
        I2P_SESSION_TAG_LENGTH
    }

    fn from_slice(slice: &[u8]) -> Option<SessionTag> {
        if slice.len() <= I2P_SESSION_TAG_LENGTH {
            let mut key_bytes = [0x00; I2P_SESSION_TAG_LENGTH];
            let offset = I2P_SESSION_TAG_LENGTH - slice.len();
            for i in 0..slice.len() {
                key_bytes[i + offset] = slice[i];
            }
            Some(SessionTag::new(key_bytes))
        } else {
            None
        }
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Default for SessionTag {
    fn default() -> SessionTag {
        SessionTag::new([0x00; I2P_SESSION_TAG_LENGTH])
    }
}

impl Clone for SessionTag {
    fn clone(&self) -> SessionTag {
        let mut cloned_hash = [0x00; I2P_SESSION_TAG_LENGTH];
        for i in 0..self.len() {
            cloned_hash[i] = self.data[i];
        }

        SessionTag::new(cloned_hash)
    }
}

impl PartialEq for SessionTag {
    fn eq(&self, other: &SessionTag) -> bool {
        for i in 0..self.len() {
            if self.data[i] != other.data[i] {
                return false;
            }
        }

        true
    }
}

impl From<[u8; I2P_SESSION_TAG_LENGTH]> for SessionTag {
    fn from(data: [u8; I2P_SESSION_TAG_LENGTH]) -> SessionTag {
        SessionTag::new(data)
    }
}

impl<'a> From<&'a [u8; I2P_SESSION_TAG_LENGTH]> for SessionTag {
    fn from(data: &'a [u8; I2P_SESSION_TAG_LENGTH]) -> SessionTag {
        let mut cloned_data = [0x00; I2P_SESSION_TAG_LENGTH];
        for i in 0..data.len() {
            cloned_data[i] = data[i];
        }

        SessionTag::new(cloned_data)
    }
}

impl fmt::Display for SessionTag {
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

impl fmt::LowerHex for SessionTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:02x}", byte);
        }

        write!(f, "{}", output)
    }
}

impl fmt::UpperHex for SessionTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:02X}", byte);
        }

        write!(f, "{}", output)
    }
}

impl fmt::Binary for SessionTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for byte in self.as_ref() {
            write!(output, "{:08b}", byte);
        }

        write!(f, "{}", output)
    }
}

impl AsRef<[u8]> for SessionTag {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}


#[cfg(test)]
mod tests {

}
