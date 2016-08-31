use common::I2pString;
use common;
use rand::Rand;
use quickcheck;


impl quickcheck::Arbitrary for I2pString {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> I2pString {
        Rand::rand(random)
    }
}


mod property_tests {
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

mod serialization_property_tests {
    use common::I2pString;
    use common;
    use quickcheck;
    use serialize::{Serialize, Deserialize};


    const PROP_BUFFER_LENGTH: usize = 256;

    fn equal(slice1: &[u8], slice2: &[u8]) -> bool {
        if slice1.len() == slice2.len() {
            for (byte1, byte2) in slice1.iter().zip(slice2.iter()) {
                if byte1 != byte1 {
                    return false;
                }
            }

            true
        } else {
            false
        }
    }

    #[test]
    fn prop_i2p_string_serialize_deserialize_output_should_be_equal_to_input() {
        fn property(string: I2pString) -> bool {
            let mut serialized_string = [0x00; PROP_BUFFER_LENGTH];
            Serialize::serialize(&string, serialized_string.as_mut()).unwrap();
            let deserialized_string = <I2pString as Deserialize>::deserialize(serialized_string.as_ref()).unwrap();

            string == deserialized_string
        }
        quickcheck::quickcheck(property as fn(I2pString) -> bool);
    }

    #[test]
    fn prop_i2p_string_serialize_deserialize_output_should_have_equal_length_to_input() {
        fn property(string: I2pString) -> bool {
            let mut serialized_string = [0x00; PROP_BUFFER_LENGTH];
            Serialize::serialize(&string, serialized_string.as_mut()).unwrap();
            let deserialized_string = <I2pString as Deserialize>::deserialize(serialized_string.as_ref()).unwrap();

            string.len() == deserialized_string.len()
        }
        quickcheck::quickcheck(property as fn(I2pString) -> bool);
    }

    #[test]
    fn prop_i2p_string_deserialize_serialize_output_should_be_equal_to_input() {
        fn property(string: I2pString) -> bool {
            let mut serialized_string = [0x00; PROP_BUFFER_LENGTH];
            Serialize::serialize(&string, serialized_string.as_mut()).unwrap();
            let deserialized_string = <I2pString as Deserialize>::deserialize(serialized_string.as_ref()).unwrap();

            let mut reserialized_string = [0x00; PROP_BUFFER_LENGTH];
            Serialize::serialize(&deserialized_string, reserialized_string.as_mut()).unwrap();

            // Check that the serialization and the reserialization match bytewise.
            equal(serialized_string.as_ref(), reserialized_string.as_ref())
        }
        quickcheck::quickcheck(property as fn(I2pString) -> bool);
    }


    #[test]
    fn prop_serialize_should_fail_without_writing_to_buffer_when_buffer_too_small() {
        fn property(string: I2pString) -> bool {
            let mut serialized_string: [u8; 1] = [0x00; 1];
            if string.len() < serialized_string.len() {
                return true;
            }

            match Serialize::serialize(&string, serialized_string.as_mut()) {
                Ok(_)  => false,
                Err(_) => true
            }
        }
        quickcheck::quickcheck(property as fn(I2pString) -> bool);
    }

    #[test]
    fn prop_i2p_string_encoded_length_should_not_exceed_max_string_length() {
        fn property(string: I2pString) -> bool {
            assert!(PROP_BUFFER_LENGTH < 300);
            let mut serialized_string = [0x00; 300];
            let bytes_written = match Serialize::serialize(&string, serialized_string.as_mut()) {
                Ok(bytes) => bytes,
                Err(_) => unreachable!()
            };

            assert!(bytes_written <= PROP_BUFFER_LENGTH);
            assert_eq!(bytes_written, string.len()+1);
            for byte in serialized_string.iter().skip(bytes_written) {
                if *byte != 0x00 {
                    return false;
                }
            }
            true
        }
        quickcheck::quickcheck(property as fn(I2pString) -> bool);
    }
}

mod serialization_tests {
    use common::I2pString;
    use common;
    use serialize::{Serialize, Deserialize};


    const TEST_BUFFER_LENGTH: usize = 256;

    #[test]
    fn test_serialize_should_fail_without_writing_to_buffer_when_buffer_too_small() {
        let raw_string_bytes: [u8; TEST_BUFFER_LENGTH-1] = [b'A'; TEST_BUFFER_LENGTH-1];
        let string = I2pString::from_utf8(raw_string_bytes.as_ref()).unwrap();
        let mut buffer: [u8; 100] = [0x00; 100];
        assert!(TEST_BUFFER_LENGTH > 100);

        let result = Serialize::serialize(&string, buffer.as_mut());
        assert!(result.is_err());

        for byte in buffer.iter() {
            assert_eq!(*byte, 0x00);
        }
    }
}
