use std::convert::From;
use std::str;
use std::fmt;
use serialize;
use rand;


pub const I2P_MAX_STRING_LENGTH: usize = 255;

/// The `I2pString` type represents a UTF-8 encoded string.
/// An `I2pString` has length at most 255 bytes. It may have a length of 0.
/// The length of an I2pString includes the leading byte describing the length of the string.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct I2pString {
    length: usize,
    data: String
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum I2pStringError {
    NotEnoughCapacity(usize, usize),
    InvalidUtf8,
}

impl I2pString {
    pub fn new() -> I2pString {
        let data = String::with_capacity(I2P_MAX_STRING_LENGTH);

        I2pString {
            length: 0,
            data: data
        }
    }

    /// Returns the length of the string, in bytes.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns the maximum number of characters a string can store.
    pub fn capacity(&self) -> usize {
        I2P_MAX_STRING_LENGTH
    }

    /// Returns a new I2pString from a moved vector.
    fn from_vec(vec: Vec<u8>) -> Result<I2pString, I2pStringError> {
        if vec.len() <= I2P_MAX_STRING_LENGTH {
            let data = match String::from_utf8(vec) {
                Ok(string) => string,
                Err(_) => return Err(I2pStringError::InvalidUtf8)
            };

            let i2p_string = I2pString {
                length: data.len(),
                data: data
            };

            Ok(i2p_string)
        } else {
            Err(I2pStringError::NotEnoughCapacity(vec.len(), I2P_MAX_STRING_LENGTH))
        }
    }

    pub fn from_utf8(slice: &[u8]) -> Result<I2pString, I2pStringError> {
        if slice.len() <= I2P_MAX_STRING_LENGTH {
            let str_slice = str::from_utf8(slice);
            if str_slice.is_ok() {
                let length = slice.len();
                let mut data = String::with_capacity(I2P_MAX_STRING_LENGTH);
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
            Err(I2pStringError::NotEnoughCapacity(slice.len(), I2P_MAX_STRING_LENGTH))
        }
    }

    pub fn from_str(slice: &str) -> Result<I2pString, I2pStringError> {
        if slice.len() <= I2P_MAX_STRING_LENGTH {
            let length = slice.len();
            let mut data = String::with_capacity(I2P_MAX_STRING_LENGTH);
            data.push_str(slice);

            let string = I2pString {
                length: length,
                data: data
            };
            Ok(string)
        } else {
            Err(I2pStringError::NotEnoughCapacity(slice.len(), I2P_MAX_STRING_LENGTH))
        }
    }

    pub fn as_str(&self) -> &str {
        self.data.as_str()
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        self.data.as_mut_str()
    }

    pub fn push_u8(&mut self, ch: u8) -> Result<(), I2pStringError> {
        if self.len() < self.capacity() {
            self.data.push(ch as char);
            self.length += 1;
            Ok(())
        } else {
            Err(I2pStringError::NotEnoughCapacity(self.len()+1, self.capacity()))
        }
    }

    pub fn push(&mut self, ch: char) -> Result<(), I2pStringError> {
        if self.len() < self.capacity() {
            self.data.push(ch);
            self.length += 1;
            Ok(())
        } else {
            Err(I2pStringError::NotEnoughCapacity(self.len()+1, self.capacity()))
        }
    }

    pub fn push_str(&mut self, string: &str) -> Result<(), I2pStringError> {
        if string.len() <= self.capacity() - self.len() {
            self.data.push_str(string);
            self.length += string.len();
            Ok(())
        } else {
            Err(I2pStringError::NotEnoughCapacity(self.len()+string.len(), self.capacity()))
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_bytes()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.length = 0;
    }
}

impl From<char> for I2pString {
    fn from(ch: char) -> I2pString {
        let mut i2p_string = I2pString::new();
        i2p_string.push(ch).unwrap();

        i2p_string
    }
}

impl fmt::Display for I2pString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl rand::Rand for I2pString {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        let mut string = String::new();
        let length: usize = rng.gen_range::<usize>(0, I2P_MAX_STRING_LENGTH+1);

        // We loop in this way because String's len() function counts bytes,
        // not graphemes. Otherwise our I2pStrings end up being too long.
        while string.len() < length {
            string.push(rng.gen::<u8>() as char);
        }

        // Trim off excess bytes generated by the previous loop. The char
        // generator can produce chars that are larger than one byte.
        while string.len() > length {
            string.pop();
        }

        I2pString::from_str(string.as_str()).unwrap()
    }
}

impl serialize::Serialize for I2pString {
    fn serialize(&self, buf: &mut [u8]) -> Result<usize, serialize::Error> {
        // If the data fits inside the buffer, write to it.
        if self.len() < buf.len() {
            let str_data = self.as_bytes();
            buf[0] = self.len() as u8;
            for i in 1..self.len()+1 {
                buf[i] = str_data[i];
            }
            Ok(self.len())
        } else {
            Err(serialize::Error::buffer_too_small(self.len()+1, buf.len()))
        }
    }
}

impl serialize::Deserialize for I2pString {
    type Output = I2pString;

    fn deserialize(buf: &[u8]) -> Result<I2pString, serialize::Error> {
        // The number of bytes in an I2pString follows the first byte.
        if buf.is_empty() {
            return Err(serialize::Error::buffer_too_small(1, buf.len()));
        }

        let nbytes = buf[0] as usize;
        if buf.len() > nbytes {
            let mut data: Vec<u8> = Vec::with_capacity(I2P_MAX_STRING_LENGTH);
            for i in 1..nbytes+1 {
                data.push(buf[i]);
            }

            let i2p_string = match I2pString::from_vec(data) {
                Ok(string) => string,
                Err(_) => {
                    return Err(serialize::Error::DecodingError(String::from("Invalid UTF8 data.")));
                }
            };

            Ok(i2p_string)
        } else {
            Err(serialize::Error::buffer_too_small(nbytes, buf.len()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::I2pString;


    #[test]
    fn test_push_str_should_accept_strings_of_max_length() {
        let mut string = String::new();
        // Generate a maximum length I2pString.
        for _ in 0..super::I2P_MAX_STRING_LENGTH {
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
        for _ in 0..super::I2P_MAX_STRING_LENGTH+1 {
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
        for _ in 0..super::I2P_MAX_STRING_LENGTH-1 {
            string.push('A');
        }

        let mut i2p_string = I2pString::new();
        assert_eq!(i2p_string.len(), 0);
        let result = i2p_string.push_str(string.as_str());
        assert!(result.is_ok());

        let old_length = i2p_string.len();
        let result = i2p_string.push('A');
        assert!(result.is_ok());
        assert_eq!(i2p_string.len(), old_length+1);
    }

    #[test]
    fn test_push_should_fail_only_when_string_is_at_least_max_length() {
        let mut string = String::new();
        // Generate a string.
        for _ in 0..super::I2P_MAX_STRING_LENGTH {
            string.push('A');
        }

        let mut i2p_string = I2pString::new();
        assert_eq!(i2p_string.len(), 0);
        let result = i2p_string.push_str(string.as_str());
        assert!(result.is_ok());
        assert_eq!(i2p_string.len(), i2p_string.capacity());
        // Generate a maximum length i2p_string.
        let result = i2p_string.push('A');
        assert!(result.is_err());
    }
}
