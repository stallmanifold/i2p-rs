use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_SHA256_HASH_LENGTH: usize = 32;

simple_data_structure!(Hash256, I2P_SHA256_HASH_LENGTH);

pub trait Hashable256 {
    fn hash_sha256(&self) -> Hash256;
}

#[cfg(test)]
mod tests {

}
