use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_PUBLIC_KEY_LENGTH: usize = 256;

/// This structure is used in ElGamal encryption, representing only the exponent, not
/// the primes, which are constant and define in the cryptography specification.
simple_data_structure!(PublicKey, I2P_PUBLIC_KEY_LENGTH);


#[cfg(test)]
mod tests {

}
