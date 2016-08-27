use common::{I2pInt8, I2pInt16};
use common::signature::{SignatureType, SigningPublicKey};
use common::PublicKey;


/// Defines the kind of cryptography described in the certificate.
pub enum CryptoType {
    ElGamal
}

/// Defines the kind of certificates that can be computed.
pub enum CertificateType {
    NULL     = 0,
    /// Experimental, unused. Payload contains an ASCII colon-separated hashcash string.
    HASHCASH = 1,
    /// Experimental, unused. Hidden routers generally do ot announce they are hidden.
    HIDDEN   = 2,
    /// Experimental, unused. Payload contains a 40-byte DSA signature,
    /// optionally followed by a 32-byte Hash of the signing Destination.
    SIGNED   = 3,
    // Experimental, unused. Payload contains multiple certificates.
    MULTIPLE = 4,
    // Introduced in version 0.9.12.
    KEY      = 5
}

/// A `Certificate` is a container for various receipts or proofs of work used
/// throughout the I2P network.
pub struct Certificate {
    certificate_type: CertificateType,
    length:  I2pInt16,
    signing_key: SigningPublicKey,
    public_key: PublicKey,
    certificate: Vec<u8>
}

impl Certificate {
    fn new(certificate_type: CertificateType,
           length: I2pInt16,
           signing_key: SigningPublicKey,
           public_key: PublicKey,
           certificate: Vec<u8>) -> Certificate
    {
        Certificate {
            certificate_type: certificate_type,
            length: length,
            signing_key: signing_key,
            public_key: public_key,
            certificate: certificate
        }
    }

    fn type_code_to_sigtype(type_code: usize) -> Option<SignatureType> {
        match type_code {
            0 => Some(SignatureType::DSA_SHA1),
            1 => Some(SignatureType::ECDSA_SHA256_P256),
            2 => Some(SignatureType::ECDSA_SHA384_P384),
            3 => Some(SignatureType::ECDSA_SHA512_P521),
            4 => Some(SignatureType::RSA_SHA256_2048),
            5 => Some(SignatureType::RSA_SHA384_3072),
            6 => Some(SignatureType::RSA_SHA512_4096),
            7 => Some(SignatureType::EdDSA_SHA512_Ed25519),
            8 => Some(SignatureType::EdDSA_SHA512_Ed25519ph),
            _ => None
        }
    }

    fn type_code_to_crypto_type(type_code: usize) -> Option<CryptoType> {
        match type_code {
            0 => Some(CryptoType::ElGamal),
            _ => None
        }
    }
}


#[cfg(test)]
mod tests {

}
