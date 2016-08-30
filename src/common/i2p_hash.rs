use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_SHA256_HASH_LENGTH: usize = 32;

/// A `Hash256` type represents the SHA256 hash of some data.
simple_data_structure!(Hash256, I2P_SHA256_HASH_LENGTH);
simple_data_structure_serialize_impl!(Hash256);

/// The trait `Hashable256` is an interface for generating a SHA256 hash of
/// a piece of data.
pub trait Hashable256 {
    /// Computes the SHA256 hash.
    fn hash_sha256(&self) -> Hash256;
}

impl Hashable256 for Hash256 {
    fn hash_sha256(&self) -> Hash256 {
        self.clone()
    }
}

#[cfg(test)]
mod tests {

}
