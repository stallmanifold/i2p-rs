use common::I2pDate;
use rand::Rand;
use quickcheck;


impl quickcheck::Arbitrary for I2pDate {
    fn arbitrary<G: quickcheck::Gen>(random: &mut G) -> I2pDate {
        Rand::rand(random)
    }
}

#[test]
fn prop_i2p_date_should_always_be_positive() {
    fn property(date: I2pDate) -> bool {
        date >= I2pDate::min_value()
    }
    quickcheck::quickcheck(property as fn(I2pDate) -> bool);
}
