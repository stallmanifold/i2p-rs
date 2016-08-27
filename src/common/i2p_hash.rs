use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;

use common::simple_data_structure;


const I2P_SHA256_HASH_LENGTH: usize = 32;

simple_data_structure!(Hash256, I2P_SHA256_HASH_LENGTH);


#[cfg(test)]
mod tests {

}
