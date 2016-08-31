use common::I2pInt32;
use rand::Rand;
use quickcheck;


macro_rules! arbitrary_impl {
    ($T:ty) => {
        impl quickcheck::Arbitrary for $T {
            fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> $T {
                Rand::rand(random)
            }
        }
    }
}

arbitrary_impl!(I2pInt32);


mod property_tests {
    use quickcheck;
    use common::I2pInt32;


    #[test]
    fn prop_i2p_integer_addition_is_commutative() {
        fn property(val1: I2pInt32, val2: I2pInt32) -> bool {
            val1 + val2 == val2 + val1
        }
        quickcheck::quickcheck(property as fn(I2pInt32, I2pInt32) -> bool);
    }

    #[test]
    fn prop_i2p_integer_multiplication_is_commutative() {
        fn property(val1: I2pInt32, val2: I2pInt32) -> bool {
            val1 * val2 == val2 * val1
        }
        quickcheck::quickcheck(property as fn(I2pInt32, I2pInt32) -> bool);
    }

    #[test]
    fn prop_i2p_integer_addition_is_preserved_under_constructor() {
        fn property(val1: I2pInt32, val2: I2pInt32) -> bool {
            let sum = val1.to_u64() + val2.to_u64();
            let total = I2pInt32::new(sum);

            val1 + val2 == total
        }
        quickcheck::quickcheck(property as fn(I2pInt32, I2pInt32) -> bool);
    }

    #[test]
    fn prop_i2p_integer_multiplication_is_preserved_under_constructor() {
        fn property(val1: I2pInt32, val2: I2pInt32) -> bool {
            let product = val1.to_u64() * val2.to_u64();
            let total = I2pInt32::new(product);

            val1 * val2 == total
        }
        quickcheck::quickcheck(property as fn(I2pInt32, I2pInt32) -> bool);
    }

    #[test]
    fn prop_i2p_integer_addition_is_associative() {
        fn property(val1: I2pInt32, val2: I2pInt32, val3: I2pInt32) -> bool {
            val1 + (val2 + val3) == val1 + (val2 + val3)
        }
        quickcheck::quickcheck(property as fn(I2pInt32, I2pInt32, I2pInt32) -> bool);
    }

    #[test]
    fn prop_i2p_integer_multiplication_is_associative() {
        fn property(val1: I2pInt32, val2: I2pInt32, val3: I2pInt32) -> bool {
            val1 * (val2 * val3) == (val1 * val2) * val3
        }
        quickcheck::quickcheck(property as fn(I2pInt32, I2pInt32, I2pInt32) -> bool);
    }
}


mod serialization_tests {
    use quickcheck;
    use common::I2pInt32;
    use serialize::{Serialize, Deserialize};


    #[test]
    fn prop_i2p_integer_serialize_deserialize_output_should_be_equal_to_input() {
        fn property(i2p_integer: I2pInt32) -> bool {
            let mut serialized_int: [u8; 4] = [0x00; 4];
            Serialize::serialize(&i2p_integer, serialized_int.as_mut());
            let deserialized_int = <I2pInt32 as Deserialize>::deserialize(serialized_int.as_ref()).unwrap();
            assert_eq!(deserialized_int, i2p_integer);
            i2p_integer == deserialized_int
        }
        quickcheck::quickcheck(property as fn(I2pInt32) -> bool);
    }

    #[test]
    fn prop_i2p_integer_deserialize_serialize_output_should_be_equal_to_input() {
        fn property(i2p_integer: I2pInt32) -> bool {
            let mut serialized_int: [u8; 8] = [0x00; 8];
            Serialize::serialize(&i2p_integer, serialized_int.as_mut()).unwrap();
            let deserialized_int = <I2pInt32 as Deserialize>::deserialize(serialized_int.as_ref()).unwrap();

            let mut reserialized_int: [u8; 8] = [0x00; 8];
            Serialize::serialize(&deserialized_int, reserialized_int.as_mut()).unwrap();

            serialized_int == reserialized_int
        }
        quickcheck::quickcheck(property as fn(I2pInt32) -> bool);
    }

    #[test]
    fn prop_i2p_integer_serialization_should_be_correct_number_of_bytes_long() {
        fn property(i2p_integer: I2pInt32) -> bool {
            let mut serialized_int: [u8; 8] = [0x00; 8];

            let bytes_written = match Serialize::serialize(&i2p_integer, serialized_int.as_mut()) {
                Ok(nbytes) => nbytes,
                Err(e) => panic!("Error: {:?}", e)
            };

            bytes_written == i2p_integer.len()

        }
        quickcheck::quickcheck(property as fn(I2pInt32) -> bool);
    }
}
