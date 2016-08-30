use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_PRIVATE_KEY_LENGTH: usize = 256;

/// This structure is used in ElGamal decryption, representing only the exponent, not
/// the primes which are constant and defined in the cryptograph specification.
simple_data_structure!(PrivateKey, I2P_PRIVATE_KEY_LENGTH);
simple_data_structure_serialize_impl!(PrivateKey);
simple_data_structure_deserialize_impl!(PrivateKey, I2P_PRIVATE_KEY_LENGTH);


#[cfg(test)]
mod tests {

}
