use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_PUBLIC_KEY_LENGTH: usize = 256;

simple_data_structure!(PublicKey, I2P_PUBLIC_KEY_LENGTH);


#[cfg(test)]
mod tests {

}
