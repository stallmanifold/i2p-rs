use std::convert::From;
use std::str;


const I2P_MAX_STRING_LENGTH: usize = 256;
const I2P_MAX_NUMBER_OF_CHARS: usize = 255;

/// The `I2pString` type represents a UTF-8 encoded string.
/// An `I2pString` has length at most 255 bytes. It may have a length of 0.
// The length of an I2pString includes the leading byte describing the length of the string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct I2pString {
    length: usize,
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
        self.length
    }

    /// Returns the maximum number of characters a string can store.
    fn capacity(&self) -> usize {
        I2P_MAX_NUMBER_OF_CHARS
    }

    fn from_utf8(slice: &[u8]) -> Result<I2pString, I2pStringError> {
        if slice.len() < I2P_MAX_NUMBER_OF_CHARS {
            let str_slice = str::from_utf8(slice);
            if str_slice.is_ok() {
                let length = slice.len();
                let mut data = String::with_capacity(I2P_MAX_STRING_LENGTH);
                data.push(length as u8 as char);
                data.push_str(str_slice.unwrap());

                let string = I2pString {
                    length: length,
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

    fn push_u8(&mut self, ch: u8) -> Result<(), I2pStringError> {
        if self.len() < self.capacity() {
            self.data.push(ch as char);
            self.length += 1;
            Ok(())
        } else {
            Err(I2pStringError::NotEnoughCapacity)
        }
    }

    fn push(&mut self, ch: char) -> Result<(), I2pStringError> {
        if self.len() < self.capacity() {
            self.data.push(ch);
            self.length += 1;
            Ok(())
        } else {
            Err(I2pStringError::NotEnoughCapacity)
        }
    }

    fn push_str(&mut self, string: &str) -> Result<(), I2pStringError> {
        if string.len() <= self.capacity() - self.len() {
            self.data.push_str(string);
            self.length = string.len();
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
        for _ in 0..super::I2P_MAX_NUMBER_OF_CHARS {
            string.push('A');
        }

        let mut i2p_string = I2pString::new();
        assert_eq!(string.len(), i2p_string.capacity());
        let result = i2p_string.push_str(string.as_str());
        assert!(result.is_ok());
        assert_eq!(i2p_string.len(), i2p_string.capacity());
    }

    #[test]
    fn test_push_str_should_fail_when_string_is_longer_than_max_length() {
        let mut string = String::new();
        // Generate a maximum length I2pString.
        for _ in 0..super::I2P_MAX_NUMBER_OF_CHARS+1 {
            string.push('A');
        }

        let mut i2p_string = I2pString::new();
        let old_length = i2p_string.len();
        let result = i2p_string.push_str(string.as_str());
        assert!(result.is_err());
        assert_eq!(old_length, i2p_string.len());
    }

    #[test]
    fn test_push() {
        let mut string = String::new();
        // Generate a string.
        for _ in 0..super::I2P_MAX_NUMBER_OF_CHARS-1 {
            string.push('A');
        }

        let mut i2p_string = I2pString::new();
        assert_eq!(i2p_string.len(), 0);
        i2p_string.push_str(string.as_str());
        let old_length = i2p_string.len();
        let result = i2p_string.push('A');
        assert!(result.is_ok());
        assert_eq!(i2p_string.len(), old_length+1);
    }

    #[test]
    fn test_push_should_fail_only_when_string_is_at_least_max_length() {
        let mut string = String::new();
        // Generate a string.
        for _ in 0..super::I2P_MAX_NUMBER_OF_CHARS {
            string.push('A');
        }

        let mut i2p_string = I2pString::new();
        assert_eq!(i2p_string.len(), 0);
        let result = i2p_string.push_str(string.as_str());
        assert_eq!(i2p_string.len(), i2p_string.capacity());
        // Generate a maximum length i2p_string.
        let result = i2p_string.push('A');
        assert!(result.is_err());
    }
}
