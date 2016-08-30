use std::fmt;
use std::fmt::Write;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64;
use rand;


const I2P_SESSION_TAG_LENGTH: usize = 32;

/// A `SessionTag` is a random number of length 32 bytes.
simple_data_structure!(SessionTag, I2P_SESSION_TAG_LENGTH);
simple_data_structure_serialize_impl!(SessionTag);

impl rand::Rand for SessionTag {
    fn rand<R: rand::Rng>(rng: &mut R) -> SessionTag {
        let mut data = [0x00; I2P_SESSION_TAG_LENGTH];
        rng.fill_bytes(data.as_mut());

        SessionTag::new(data)
    }
}
