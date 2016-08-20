use common::I2pString;
use rand::Rand;
use quickcheck;


impl quickcheck::Arbitrary for I2pString {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> I2pString {
        Rand::rand(random)
    }
}


#[cfg(test)]
mod tests {
    use quickcheck;
    use common::I2pString;
    use common;


    #[test]
    fn prop_i2p_string_capacity_should_be_255_characters() {
        fn property(string: I2pString) -> bool {
            string.capacity() == common::I2P_MAX_STRING_LENGTH
        }
        quickcheck::quickcheck(property as fn(I2pString) -> bool);
    }

    #[test]
    fn prop_i2p_string_length_should_be_at_most_255_characters() {
        fn property(string: I2pString) -> bool {
            string.len() <= common::I2P_MAX_STRING_LENGTH
        }
        quickcheck::quickcheck(property as fn(I2pString) -> bool);
    }

    #[test]
    fn prop_i2p_string_should_be_no_more_than_capacity() {
        fn property(string: I2pString) -> bool {
            string.len() <= string.capacity()
        }
        quickcheck::quickcheck(property as fn(I2pString) -> bool);
    }

    #[test]
    fn prop_i2p_string_should_be_valid_utf8_string() {
        fn property(string: I2pString) -> bool {
            let slice = string.as_str();
            let i2p_string = I2pString::from_str(slice);
            i2p_string.is_ok()
        }
        quickcheck::quickcheck(property as fn(I2pString) -> bool);
    }
}
