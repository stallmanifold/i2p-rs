/// The `simple_data_structure` macro is a typeclass describing structures that
/// are composed of fixed-length arrays.
macro_rules! simple_data_structure {
    ($TYPE_NAME:ident, $ARRAY_LENGTH:expr) => {
        #[derive(Eq)]
        pub struct $TYPE_NAME {
            data: [u8; $ARRAY_LENGTH]
        }

        impl $TYPE_NAME {
            fn new(data: [u8; $ARRAY_LENGTH]) -> $TYPE_NAME {
                $TYPE_NAME {
                    data: data
                }
            }

            /// Returns the length of an `$TYPE_NAME` in bytes.
            pub fn len(&self) -> usize {
                $ARRAY_LENGTH
            }

            fn from_bytes(bytes: &[u8]) -> Option<$TYPE_NAME> {
                if bytes.len() <= $ARRAY_LENGTH {
                    let mut key_bytes = [0x00; $ARRAY_LENGTH];
                    let offset = $ARRAY_LENGTH - bytes.len();

                    for (i, byte) in bytes.iter().enumerate() {
                        key_bytes[i + offset] = *byte;
                    }

                    Some($TYPE_NAME::new(key_bytes))
                } else {
                    None
                }
            }

            fn as_slice(&self) -> &[u8] {
                self.data.as_ref()
            }
        }

        impl Default for $TYPE_NAME {
            fn default() -> $TYPE_NAME {
                $TYPE_NAME::new([0x00; $ARRAY_LENGTH])
            }
        }

        impl Clone for $TYPE_NAME {
            fn clone(&self) -> $TYPE_NAME {
                let mut cloned_hash = [0x00; $ARRAY_LENGTH];
                for (i, byte) in self.data.iter().enumerate() {
                    cloned_hash[i] = *byte;
                }

                $TYPE_NAME::new(cloned_hash)
            }
        }

        impl PartialEq for $TYPE_NAME {
            fn eq(&self, other: &$TYPE_NAME) -> bool {
                for i in 0..self.len() {
                    if self.data[i] != other.data[i] {
                        return false;
                    }
                }

                true
            }
        }

        impl From<[u8; $ARRAY_LENGTH]> for $TYPE_NAME {
            fn from(data: [u8; $ARRAY_LENGTH]) -> $TYPE_NAME {
                $TYPE_NAME::new(data)
            }
        }

        impl<'a> From<&'a [u8; $ARRAY_LENGTH]> for $TYPE_NAME {
            fn from(data: &'a [u8; $ARRAY_LENGTH]) -> $TYPE_NAME {
                let mut cloned_data = [0x00; $ARRAY_LENGTH];
                for i in 0..data.len() {
                    cloned_data[i] = data[i];
                }

                $TYPE_NAME::new(cloned_data)
            }
        }

        impl fmt::Display for $TYPE_NAME {
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

        impl fmt::LowerHex for $TYPE_NAME {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut output = String::new();
                for byte in self.as_ref() {
                    write!(output, "{:02x}", byte).unwrap();
                }

                write!(f, "{}", output)
            }
        }

        impl fmt::UpperHex for $TYPE_NAME {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut output = String::new();
                for byte in self.as_ref() {
                    write!(output, "{:02X}", byte).unwrap();
                }

                write!(f, "{}", output)
            }
        }

        impl fmt::Binary for $TYPE_NAME {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut output = String::new();
                for byte in self.as_ref() {
                    write!(output, "{:08b}", byte).unwrap();
                }

                write!(f, "{}", output)
            }
        }

        impl AsRef<[u8]> for $TYPE_NAME {
            fn as_ref(&self) -> &[u8] {
                self.as_slice()
            }
        }
    }
}

macro_rules! simple_data_structure_serialize_impl {
    ($TYPE_NAME:ty) => {
        use serialize;

        // Serialize simple data structures in a big endian manner.
        impl serialize::Serialize for $TYPE_NAME {
            fn serialize(&self, buf: &mut [u8]) -> serialize::Result<usize> {
                // If the data fits inside the buffer, write to it.
                if self.len() <= buf.len() {
                    for (i, byte) in self.data.iter().enumerate() {
                        buf[i] = *byte;
                    }
                    Ok(self.len())
                } else {
                    Err(serialize::Error::buffer_too_small(self.len(), buf.len()))
                }
            }
        }
    }
}

macro_rules! simple_data_structure_deserialize_impl {
    ($TYPE_NAME:ident, $ARRAY_LENGTH:expr) => {
        impl serialize::Deserialize for $TYPE_NAME {
            type Output = $TYPE_NAME;

            fn deserialize(buf: &[u8]) -> serialize::Result<Self::Output> {
                if buf.len() >= $ARRAY_LENGTH {
                    let mut data = [0x00; $ARRAY_LENGTH];
                    for i in 0..data.len() {
                        data[i] = buf[i];
                    }
                    Ok($TYPE_NAME::new(data))
                } else {
                    Err(serialize::Error::buffer_too_small($ARRAY_LENGTH, buf.len()))
                }
            }
        }
    }
}
