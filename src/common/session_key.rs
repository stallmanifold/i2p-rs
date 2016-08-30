use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_SESSION_KEY_LENGTH: usize = 32;

/// This structure is used for AES256 encryption and decryption. It has a length of 32 bytes.
simple_data_structure!(SessionKey, I2P_SESSION_KEY_LENGTH);
simple_data_structure_serialize_impl!(SessionKey);
simple_data_structure_deserialize_impl!(SessionKey, I2P_SESSION_KEY_LENGTH);


#[cfg(test)]
mod tests {

}
