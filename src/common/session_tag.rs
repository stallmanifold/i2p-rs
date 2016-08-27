use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_SESSION_TAG_LENGTH: usize = 32;

/// A `SessionTag` is a random number of length 32 bytes.
simple_data_structure!(SessionTag, I2P_SESSION_TAG_LENGTH);


#[cfg(test)]
mod tests {

}
