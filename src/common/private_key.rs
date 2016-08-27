use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


use common::simple_data_structure;

const I2P_PRIVATE_KEY_LENGTH: usize = 256;

simple_data_structure!(PrivateKey, I2P_PRIVATE_KEY_LENGTH);


#[cfg(test)]
mod tests {

}
