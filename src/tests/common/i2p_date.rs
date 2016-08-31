use common::I2pDate;
use rand::Rand;
use quickcheck;


impl quickcheck::Arbitrary for I2pDate {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> I2pDate {
        Rand::rand(random)
    }
}

mod property_tests {
    use common::I2pDate;
    use quickcheck;


    #[test]
    fn prop_i2p_date_should_always_be_positive() {
        fn property(date: I2pDate) -> bool {
            date >= I2pDate::min_value()
        }
        quickcheck::quickcheck(property as fn(I2pDate) -> bool);
    }
}

mod serialization_tests {
    use common::I2pDate;
    use quickcheck;
    use serialize::{Serialize, Deserialize};


    #[test]
    fn prop_i2p_date_serialize_deserialize_output_should_be_equal_to_input() {
        fn property(date: I2pDate) -> bool {
            let mut serialized_date: [u8; 8] = [0x00; 8];
            Serialize::serialize(&date, serialized_date.as_mut()).unwrap();
            let deserialized_date = <I2pDate as Deserialize>::deserialize(serialized_date.as_ref()).unwrap();

            date == deserialized_date
        }
        quickcheck::quickcheck(property as fn(I2pDate) -> bool);
    }

    #[test]
    fn prop_i2p_date_deserialize_serialize_output_should_be_equal_to_input() {
        fn property(date: I2pDate) -> bool {
            let mut serialized_date: [u8; 8] = [0x00; 8];
            Serialize::serialize(&date, serialized_date.as_mut()).unwrap();
            let deserialized_date = <I2pDate as Deserialize>::deserialize(serialized_date.as_ref()).unwrap();

            let mut reserialized_date: [u8; 8] = [0x00; 8];
            Serialize::serialize(&deserialized_date, reserialized_date.as_mut()).unwrap();

            serialized_date == reserialized_date
        }
        quickcheck::quickcheck(property as fn(I2pDate) -> bool);
    }

    #[test]
    fn prop_i2p_date_serialization_should_be_8_bytes_long() {
        fn property(date: I2pDate) -> bool {
            let mut serialized_date: [u8; 8] = [0x00; 8];

            let bytes_written = Serialize::serialize(&date, serialized_date.as_mut()).unwrap();

            bytes_written == date.len()

        }
        quickcheck::quickcheck(property as fn(I2pDate) -> bool);
    }
}
