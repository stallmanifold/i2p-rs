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
