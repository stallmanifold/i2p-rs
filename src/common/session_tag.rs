use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;


const I2P_SESSION_TAG_LENGTH: usize = 32;

simple_data_structure!(SessionTag, I2P_SESSION_TAG_LENGTH);


#[cfg(test)]
mod tests {

}
