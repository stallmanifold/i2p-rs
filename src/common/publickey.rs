use std::fmt;
use std::fmt::Write;


const I2P_PUBLIC_KEY_LENGTH: usize = 256;

#[derive(Copy, Eq)]
pub struct PublicKey {
    data: [u8; I2P_PUBLIC_KEY_LENGTH]
}

impl PublicKey {
    fn new(slice: [u8; I2P_PUBLIC_KEY_LENGTH]) -> PublicKey {
        let mut data: [u8; I2P_PUBLIC_KEY_LENGTH] = [0x00; I2P_PUBLIC_KEY_LENGTH];
        for i in 0..slice.len() {
            data[i] = slice[i];
        }

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

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        for i in 0..self.len() {
            write!(string, "{:X}", self.data[i]).unwrap();
        }

        write!(f, "{}", string)
    }
}
