use std::convert::From;
use std::str;


const I2P_MAX_STRING_LENGTH: usize = 256;

/// The `I2pString` type represents a UTF-8 encoded string.
/// An `I2pString` has length at most 255 bytes. It may have a length of 0.
// The length of an I2pString includes the leading byte describing the length of the string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct I2pString {
    length: u32,
    data: String
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum I2pStringError {
    NotEnoughCapacity,
    InvalidUtf8,
}

impl I2pString {
    fn new() -> I2pString {
        let mut data = String::with_capacity(I2P_MAX_STRING_LENGTH);
        data.push(0 as char);

        I2pString {
            length: 0,
            data:   data
        }
    }

    /// Returns the complete length of the string, including the
    /// leading length character.
    fn len_bytes(&self) -> usize {
        self.data.len()
    }

    /// Return the total number of characters.
    fn len(&self) -> usize {
        self.length as usize
    }

    /// Returns the maximum number of characters a string can store.
    fn capacity(&self) -> usize {
        I2P_MAX_STRING_LENGTH - 1
    }

    fn from_utf8(slice: &[u8]) -> Result<I2pString, I2pStringError> {
        if slice.len() < I2P_MAX_STRING_LENGTH {
            let str_slice = str::from_utf8(slice);
            if str_slice.is_ok() {
                let length = slice.len();
                let mut data = String::with_capacity(I2P_MAX_STRING_LENGTH);
                data.push(length as u8 as char);
                data.push_str(str_slice.unwrap());

                let string = I2pString {
                    length: length as u32,
                    data: data
                };
                Ok(string)
            } else {
                Err(I2pStringError::InvalidUtf8)
            }
        } else {
            Err(I2pStringError::NotEnoughCapacity)
        }
    }

    fn as_str(&self) -> &str {
        self.data.as_str()
    }

    fn as_mut_str(&mut self) -> &mut str {
        self.data.as_mut_str()
    }

    fn push(&mut self, ch: u8) -> Result<(), I2pStringError> {
        if self.len() < self.capacity() {
            self.data.push(ch as char);
            Ok(())
        } else {
            Err(I2pStringError::NotEnoughCapacity)
        }
    }

    fn push_str(&mut self, string: &str) -> Result<(), I2pStringError> {
        if string.len() < self.capacity() - self.len() {
            self.data.push_str(string);
            Ok(())
        } else {
            Err(I2pStringError::NotEnoughCapacity)
        }
    }

    fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }

    fn clear(&mut self) {
        self.data.clear();
        self.length = 0;
        self.data.push(0 as char);
    }
}


#[cfg(test)]
mod tests {
    use super::{I2pString, I2pStringError};


    #[test]
    fn test_push_str_should_accept_strings_of_max_length() {
        let mut string = String::new();
        // Generate a maximum length I2pString.
        for _ in 0..super::I2P_MAX_STRING_LENGTH {
            string.push('A');
        }

        let mut i2p_string = I2pString::new();
        let result = i2p_string.push_str(string.as_str());
        assert!(result.is_ok());
        assert_eq!(i2p_string.len(), i2p_string.capacity());
    }

    #[test]
    fn test_push_should_fail_only_when_string_is_max_length() {
        let mut string = String::new();
        // Generate a string.
        for _ in 0..super::I2P_MAX_STRING_LENGTH {
            string.push('A');
        }

        let mut i2p_string = I2pString::new();
        // Generate a maximum length i2p_string.
        let result = i2p_string.push(b'A');
        assert!(result.is_err());
    }
}
