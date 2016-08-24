use std::fmt;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SignatureType {
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

impl SignatureType {
    fn signing_public_key_length(&self) -> usize {
        match *self {
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

    fn signing_private_key_length(&self) -> usize {
        match *self {
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SigningPublicKey {
    sigtype: SignatureType,
    data: Vec<u8>
}

impl SigningPublicKey {
    fn new(sigtype: SignatureType) -> SigningPublicKey {
        let mut data: Vec<u8> = Vec::with_capacity(sigtype.signing_public_key_length());
        for i in 0..data.capacity() {
            data.push(0x00);
        }

        SigningPublicKey {
            sigtype: sigtype,
            data: data
        }
    }

    fn from_bytes(sigtype: SignatureType, bytes: &[u8]) -> Option<SigningPublicKey> {
        if sigtype.signing_public_key_length() == bytes.len() {
            let data: Vec<u8> = bytes.iter().map(|c: &u8| *c).collect();

            let key = SigningPublicKey {
                sigtype: sigtype,
                data: data
            };

            Some(key)
        } else {
            None
        }
    }

    /// Returns the length of a `SigningPublicKey` in bytes.
    fn len(&self) -> usize {
        self.data.len()
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Default for SigningPublicKey {
    fn default() -> SigningPublicKey {
        SigningPublicKey::new(SignatureType::DSA_SHA1)
    }
}

impl fmt::Display for SigningPublicKey {
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

impl AsRef<[u8]> for SigningPublicKey {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SigningPrivateKey {
    sigtype: SignatureType,
    data: Vec<u8>
}

impl SigningPrivateKey {
    fn new(sigtype: SignatureType) -> SigningPrivateKey {
        let mut data: Vec<u8> = Vec::with_capacity(sigtype.signing_public_key_length());
        for i in 0..data.capacity() {
            data.push(0x00);
        }

        SigningPrivateKey {
            sigtype: sigtype,
            data: data
        }
    }

    fn from_bytes(sigtype: SignatureType, bytes: &[u8]) -> Option<SigningPrivateKey> {
        if sigtype.signing_public_key_length() == bytes.len() {
            let data: Vec<u8> = bytes.iter().map(|c: &u8| *c).collect();

            let key = SigningPrivateKey {
                sigtype: sigtype,
                data: data
            };

            Some(key)
        } else {
            None
        }
    }

    /// Returns the length of a `SigningPublicKey` in bytes.
    fn len(&self) -> usize {
        self.data.len()
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Default for SigningPrivateKey {
    fn default() -> SigningPrivateKey {
        SigningPrivateKey::new(SignatureType::DSA_SHA1)
    }
}

impl fmt::Display for SigningPrivateKey {
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

impl AsRef<[u8]> for SigningPrivateKey {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}


#[cfg(test)]
mod tests {

}
