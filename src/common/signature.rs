use std::fmt;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SigningPublicKey {
    sigtype: SignatureType,
    data: Vec<u8>
}

impl SigningPublicKey {
    fn new(sigtype: SignatureType) -> SigningPublicKey {
        let mut data: Vec<u8> = Vec::with_capacity(Self::signing_public_key_length(sigtype));
        for _ in 0..data.capacity() {
            data.push(0x00);
        }

        SigningPublicKey {
            sigtype: sigtype,
            data: data
        }
    }

    fn from_bytes(sigtype: SignatureType, bytes: &[u8]) -> Option<SigningPublicKey> {
        if Self::signing_public_key_length(sigtype) == bytes.len() {
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

    pub fn signing_public_key_length(sigtype: SignatureType) -> usize {
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
        let mut data: Vec<u8> = Vec::with_capacity(Self::signing_private_key_length(sigtype));
        for _ in 0..data.capacity() {
            data.push(0x00);
        }

        SigningPrivateKey {
            sigtype: sigtype,
            data: data
        }
    }

    fn from_bytes(sigtype: SignatureType, bytes: &[u8]) -> Option<SigningPrivateKey> {
        if Self::signing_private_key_length(sigtype) == bytes.len() {
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

    fn signing_private_key_length(sigtype: SignatureType) -> usize {
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

pub struct Signature {
    sigtype: SignatureType,
    data: Vec<u8>
}

impl Signature {
    fn signature_length(sigtype: SignatureType) -> usize {
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

    fn from_bytes(sigtype: SignatureType, bytes: &[u8]) -> Option<Signature> {
        if Self::signature_length(sigtype) == bytes.len() {
            let data: Vec<u8> = bytes.iter().map(|c: &u8| *c).collect();

            let key = Signature {
                sigtype: sigtype,
                data: data
            };

            Some(key)
        } else {
            None
        }
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Default for Signature {
    fn default() -> Signature {
        let sigtype = SignatureType::DSA_SHA1;
        let mut data = Vec::with_capacity(Signature::signature_length(sigtype));
        for _ in 0..data.capacity() {
            data.push(0x00);
        }

        Signature {
            sigtype: sigtype,
            data: data
        }
    }
}

impl fmt::Display for Signature {
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

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

#[cfg(test)]
mod tests {

}
