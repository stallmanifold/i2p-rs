use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SignatureType {
    DSA_SHA1,
    ECDSA_SHA256_P256,
    ECDSA_SHA384_P384,
    ECDSA_SHA512_P521,
    RSA_SHA256_2048,
    RSA_SHA384_3072,
    RSA_SHA512_4096,
    EdDSA_SHA512_Ed25519,
    EdDSA_SHA512_Ed25519ph
}

// The macro invocation chain occurs as follows:
// data_structure_def!(TypeName);
//
// impl SigningLength for TypeName {
//     fn signing_length(sigtype: SignatureType) -> usize {
//         ...
//     }
// }
//
// data_structure_impl!(TypeName);
macro_rules! data_structure_def {
    ($TYPE_NAME:ident) => {
        #[derive(Clone, PartialEq, Eq, Debug)]
        pub struct $TYPE_NAME {
            sigtype: SignatureType,
            data: Vec<u8>
        }
    }
}

trait SigningLength {
    fn signing_length(sigtype: SignatureType) -> usize;
}

macro_rules! data_structure_impl {
    ($TYPE_NAME:ident) => {
        impl $TYPE_NAME {
            fn new(sigtype: SignatureType) -> $TYPE_NAME {
                let mut data: Vec<u8> = Vec::with_capacity(Self::signing_length(sigtype));
                for _ in 0..data.capacity() {
                    data.push(0x00);
                }

                $TYPE_NAME {
                    sigtype: sigtype,
                    data: data
                }
            }

            fn from_bytes(sigtype: SignatureType, bytes: &[u8]) -> Option<$TYPE_NAME> {
                if Self::signing_length(sigtype) == bytes.len() {
                    let data: Vec<u8> = bytes.iter().map(|c: &u8| *c).collect();

                    let key = $TYPE_NAME {
                        sigtype: sigtype,
                        data: data
                    };

                    Some(key)
                } else {
                    None
                }
            }

            /// Returns the length of a `$TYPE_NAME` in bytes.
            fn len(&self) -> usize {
                self.data.len()
            }

            fn as_slice(&self) -> &[u8] {
                self.data.as_ref()
            }
        }

        impl Default for $TYPE_NAME {
            fn default() -> $TYPE_NAME {
                $TYPE_NAME::new(SignatureType::DSA_SHA1)
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

data_structure_def!(SigningPublicKey);

impl SigningLength for SigningPublicKey {
    fn signing_length(sigtype: SignatureType) -> usize {
        match sigtype {
            SignatureType::DSA_SHA1               => 128,
            SignatureType::ECDSA_SHA256_P256      => 64,
            SignatureType::ECDSA_SHA384_P384      => 96,
            SignatureType::ECDSA_SHA512_P521      => 132,
            SignatureType::RSA_SHA256_2048        => 256,
            SignatureType::RSA_SHA384_3072        => 384,
            SignatureType::RSA_SHA512_4096        => 512,
            SignatureType::EdDSA_SHA512_Ed25519   => 32,
            SignatureType::EdDSA_SHA512_Ed25519ph => 32
        }
    }
}
data_structure_impl!(SigningPublicKey);

data_structure_def!(SigningPrivateKey);

impl SigningLength for SigningPrivateKey {
    fn signing_length(sigtype: SignatureType) -> usize {
        match sigtype {
            SignatureType::DSA_SHA1               => 20,
            SignatureType::ECDSA_SHA256_P256      => 32,
            SignatureType::ECDSA_SHA384_P384      => 48,
            SignatureType::ECDSA_SHA512_P521      => 66,
            SignatureType::RSA_SHA256_2048        => 512,
            SignatureType::RSA_SHA384_3072        => 768,
            SignatureType::RSA_SHA512_4096        => 1024,
            SignatureType::EdDSA_SHA512_Ed25519   => 32,
            SignatureType::EdDSA_SHA512_Ed25519ph => 32
        }
    }
}
data_structure_impl!(SigningPrivateKey);

data_structure_def!(Signature);

impl SigningLength for Signature {
    fn signing_length(sigtype: SignatureType) -> usize {
        match sigtype {
            SignatureType::DSA_SHA1               => 40,
            SignatureType::ECDSA_SHA256_P256      => 64,
            SignatureType::ECDSA_SHA384_P384      => 96,
            SignatureType::ECDSA_SHA512_P521      => 132,
            SignatureType::RSA_SHA256_2048        => 256,
            SignatureType::RSA_SHA384_3072        => 384,
            SignatureType::RSA_SHA512_4096        => 512,
            SignatureType::EdDSA_SHA512_Ed25519   => 64,
            SignatureType::EdDSA_SHA512_Ed25519ph => 64
        }
    }
}
data_structure_impl!(Signature);

#[cfg(test)]
mod tests {

}
